use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use register_map::register_map;
use executable_block::executable_block;

pub struct Interpreter {
    pub mappable_regs: u8,
}

impl Interpreter {
    pub fn new() -> Interpreter{
        Interpreter {
            mappable_regs: 12,
        }
    }

    pub fn is_unc_br(&mut self, inst: u16) -> bool {
        match inst & 0xF000 {

            0x0000 | 0x1000 | 0x2000 | 0xB000 => return true,
            _ => return false,
        }
    }

    pub fn interpret(&mut self, inst: u16, register_map: &mut register_map, reached_unc_branch: &mut bool, ex_block: &mut executable_block){

        //update marker for end of code block
        if self.is_unc_br(inst){
            *reached_unc_branch = true;
            ex_block.set_exit(inst&0x0FFF);
        }

        //mark regs for allocation
        match inst & 0xF000 {
            0x6000 => {
                register_map.map(((inst>>8) & 0x0F) as u8)
            },
            0x7000 => {
                register_map.map(((inst>>8) & 0x0F) as u8)
            }
            _ => println!("unrecognized instruction or no regs to allocate"),
        }
    }

    pub fn compile_block(&mut self, pc: &u16, memory: &[u8], ex_block: &mut executable_block) -> Vec<u8>{

        //--PASS 1--//
        //disassembly

        //create cursor to parse rom
        let mut mem_cursor = Cursor::new(&memory);
        let mut mem_position = *pc as u64;
        mem_cursor.set_position(mem_position);


        let mut register_map = register_map::new();         // create register mapping table
        let mut chip8_code_block: Vec<u16> = Vec::new();    //store retrieved opcodes
        let mut reached_unc_branch = false;                 // create marker for unconditional branch

        //read from rom
        let mut instruction = mem_cursor.read_u16::<BigEndian>().unwrap();

        //loop till unconditional branch is reached or register map is full
        while !reached_unc_branch && !register_map.full(){

            self.interpret(instruction, &mut register_map, &mut reached_unc_branch, ex_block);
            chip8_code_block.push(instruction);

            mem_position += 2;
            mem_cursor.set_position(mem_position);
            instruction = mem_cursor.read_u16::<BigEndian>().unwrap();
        }

        println!("Pass 1 of compilation finished");
        println!("register map: {:?}", register_map.map);

        //--PASS 2--//
        //translation

        let mut x86_code_block: Vec<u8> = vec![0x49, 0x89, 0xFF];

        //load chip8 reg values into host cpu
        let mut prefix = self.generate_prefix(&mut register_map);
        x86_code_block.append(&mut prefix);

        //translate chip8 code
        for i in chip8_code_block.iter() {
            let mut instruction_vec: Vec<u8> = self.get_translation(*i, &mut register_map);
            x86_code_block.append(&mut instruction_vec);
        }

        //return registers back into cpu object
        let mut suffix = self.generate_suffix(&mut register_map);
        x86_code_block.append(&mut suffix);

        //print code blocks
        self.print_code_block_16(&mut chip8_code_block, "chip8_code_block".to_string());
        self.print_code_block_8(&mut x86_code_block, "x86_code_block".to_string());

        return x86_code_block;
    }

    pub fn generate_prefix(&mut self, reg_map: &mut register_map) -> Vec<u8> {

        let mut prefix: Vec<u8> = vec![];

        for (index, i) in reg_map.map.iter().enumerate(){

            if index >= 4 {
                prefix.push(0x48);                      //shift ah into al
                prefix.push(0xc1);
                prefix.push(0xe0|(index&0x03) as u8);   //x86 reg, e0 for high byte
                prefix.push(0x08);                      //8 bits
            }

            prefix.push(0x41);                          //mov byte ptr
            prefix.push(0x8a);
            prefix.push((0x47|((i[1] & 0x03) << 3)) as u8);//x86 reg (lower byte)
            prefix.push(i[0]);

        }
        return prefix;
    }

    pub fn generate_suffix(&mut self, reg_map: &mut register_map) -> Vec<u8> {

        let mut suffix: Vec<u8> = vec![];

        //store regs
        for (index, i) in reg_map.map.iter().enumerate(){

            if index >= 4 {
                suffix.push(0x48);                      //shift ah into al
                suffix.push(0xc1);
                suffix.push(0xe8|(index&0x03) as u8);   //x86 reg e8, for low byte
                suffix.push(0x08);                      //8 bits
            }

            suffix.push(0x41);                          //mov byte ptr
            suffix.push(0x88);
            suffix.push((0x47|((i[1] & 0x03) << 3)) as u8);//x86 reg (lower byte)
            suffix.push(i[0]);                          //offset (chip8 reg)
        }

        //place out of code interrupt?
        //enter exit code?

        //return
        suffix.push(0xC3);

        return suffix;
    }

    pub fn get_translation(&mut self, inst: u16, register_map: &mut register_map) -> Vec<u8> {

        let mut instruction_vec: Vec<u8> = vec![];

        match inst & 0xF000 {
            0x0000 => println!("got 0x1000"),
            0x2000 => println!("got 0x2000"),
            0x3000 => println!("got 0x3000"),
            0x4000 => println!("got 0x4000"),
            0x5000 => println!("got 0x5000"),
            0x6000 => {
                println!("got 0x6000");
                let register = register_map.get_x86_reg(((inst>>8)&0x0F) as u8);
                instruction_vec.push(0xB0 + register);
                instruction_vec.push((inst & 0xFF) as u8);
            },
            0x7000 => {
                println!("got 0x7000");
                let register = register_map.get_x86_reg(((inst>>8)&0x0F) as u8);
                if register == 0 {
                    instruction_vec.push(0x04);
                } else {
                    instruction_vec.push(0x80);
                    instruction_vec.push(0xC0|((inst>>8)&0x0F) as u8);
                }
                instruction_vec.push((inst&0x00FF) as u8);
            },
            0x8000 => println!("got 0x8000"),
            0x9000 => println!("got 0x9000"),
            0xA000 => {
                println!("got 0xA000");

            },
            0xB000 => println!("got 0xB000"),
            0xC000 => println!("got 0xC000"),
            0xD000 => println!("got 0xD000"),
            0xE000 => println!("got 0xE000"),
            0xF000 => println!("got 0xF000"),
            _ => println!("got _"),
        }

        return instruction_vec;
    }

    pub fn print_code_block_16(&mut self, code_block: &mut Vec<u16>, name: String){
        print!("{:?}: [", name );
        for i in code_block.iter() {
            print!("{:04X}, ", i );
        }
        println!("]");
    }

    pub fn print_code_block_8(&mut self, code_block: &mut Vec<u8>, name: String){
        print!("{:?}: [", name );
        for i in code_block.iter() {
            print!("{:02X}, ", i );
        }
        println!("]");
    }
}
