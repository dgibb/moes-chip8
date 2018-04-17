use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use register_map::register_map;

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

    pub fn interpret(&mut self, inst: u16, register_map: &mut register_map, reached_unc_branch: &mut bool ){

        //update marker for end of code block
        if self.is_unc_br(inst){
            *reached_unc_branch = true;
        }

        //mark regs for allocation
        match inst & 0xF000 {
            0x6000 => {
                register_map.map(((inst>>8) & 0x0F) as u8)
            },
            _ => println!("unrecognized instruction or no regs to allocate"),
        }
    }

    pub fn compile_block(&mut self, pc: &u16, memory: &[u8]) -> Vec<u8>{
        //will run till unconditional branch or end of cache block

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
        println!("about to read from memory");
        let mut instruction = mem_cursor.read_u16::<BigEndian>().unwrap();
        println!("read {:X} from rom", instruction );

        //loop till unconditional branch is reached or register map is full
        while !reached_unc_branch && !register_map.full(){

            self.interpret(instruction, &mut register_map, &mut reached_unc_branch);
            chip8_code_block.push(instruction);

            mem_position += 2;
            mem_cursor.set_position(mem_position);
            instruction = mem_cursor.read_u16::<BigEndian>().unwrap();
        }
        
        println!("Pass 1 of compilation finished");
        println!("register map: {:?}", register_map.map);

        //--PASS 2--//
        //translation
        let mut x86_code_block: Vec<u8> = vec![];

        for i in chip8_code_block.iter() {
            println!(" i = {:X}", i );
            let mut instruction_vec: Vec<u8> = self.get_translation(*i, &mut register_map);
            x86_code_block.append(&mut instruction_vec);
        }

        x86_code_block.push(0xC3); //make sure code returns

        print!("chip8_code_block: [");
        for i in chip8_code_block.iter() {
            print!("{:X}, ", i );
        }
        println!("]");

        print!("x86_code_block: [");
        for i in x86_code_block.iter() {
            print!("{:X}, ", i );
        }
        println!("]");

        return x86_code_block;
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
                if register > 0x07 as u8 {
                    instruction_vec.push(0x48)
                } else {
                    instruction_vec.push(0x49)
                }
                instruction_vec.push(0xC7);
                instruction_vec.push((0xC0+(register&0x07)) as u8 );
                instruction_vec.push((inst&0xFF) as u8);
                instruction_vec.push(0);
                instruction_vec.push(0);
                instruction_vec.push(0);
            },
            0x7000 => println!("got 0x7000"),
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
}
