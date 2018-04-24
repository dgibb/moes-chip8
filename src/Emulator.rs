//extern crate rustc_serialize;

use TranslationCache::TranslationCache;
use Interpreter::Interpreter;
use Emitter::Emitter;
use executable_block::executable_block;
use serde::ser::{Serialize, Serializer, SerializeStruct};


pub struct Emulator {
    pub rom: Vec<u8>,
    pub memory: [u8;0x1000],
    pub pc: u16,
    pub index: u16,
    pub gpr: [u8;16],
    pub stack: [u16;16],
    pub translation_cache: TranslationCache,
    pub interpreter: Interpreter,
    pub emitter: Emitter,
}

pub struct emulator_state{
    pub pc: u16,
    pub index: u16,
    pub gpr: [u8;16],
    pub stack: [u16;16],
    pub blocks: Vec<u16>,
}

unsafe impl Send for Emulator {}
unsafe impl Sync for Emulator {}
unsafe impl Send for emulator_state {}
unsafe impl Sync for emulator_state {}

impl Emulator {
    pub fn new() -> Emulator{
        Emulator {
            rom: vec![],
            memory: [0;0x1000],
            pc: 0x200,
            index: 0,
            gpr: [0;16],
            stack: [0;16],
            translation_cache: TranslationCache::new(), //use 32 or 64MB cache currently 4KB
            interpreter: Interpreter::new(),
            emitter: Emitter::new(),
         }
    }

    pub fn get_reentry_point(&mut self) -> Option< fn(&mut [u8;16]) -> u64> {

        println!("get_reentry_point entered");
        //finds and returns already compiled_block if it exists
        for i in self.translation_cache.blocks.iter() {
            if i.chip8_entry == self.pc {
                println!("block already recompiled");
                return i.fn_ptr;
            }
        }

        //else creates new executable_block
        //recompiles code at pc
        //adds recompiled block to the translation_cache
        //returns pointer to recompiled binary code as re-entry point for emulator

        println!("creating new executable_block");
        let mut ex_block = executable_block::new(self.pc);
        self.emitter.emit(&mut self.interpreter.compile_block(&self.pc, &self.memory, &mut ex_block), &mut ex_block);
        let fn_ptr = ex_block.assign_fn_ptr();
        self.translation_cache.add(ex_block);
        return fn_ptr;
    }

    // psuedo run indefinitely function
    pub fn run(&mut self){
        loop {
            self.run_block();
            //self.handle_interrupts();
            //self.handle_io();
        }
    }

    pub fn run_block(&mut self) {

        println!("run_block entered");
        let fn_ptr = self.get_reentry_point();
        println!("line before x86_code_block");
        let return_val = (fn_ptr.unwrap())(&mut self.gpr);
        println!("Emulator.gpr: {:?}", self.gpr);
        //println!("Emulator.gpr: {:?}", self.gpr);
        self.evaluate_interrupt(return_val);

    }

    pub fn evaluate_interrupt(&mut self, return_val: u64 ){
        match return_val{

            0x3400 => {
                //println!("congrats you got the correct exit code: {:X}", t0);
            }

            _ => {
                println!("Error! you got the wrong exit code: {:X}", return_val);
            }
        }
    }

    pub fn init(&mut self){
        //write rom into memory
        for (index, i) in self.rom.iter().enumerate() {
            self.memory[0x200+index]=*i;
        }
    }

    pub fn get_state(&mut self) -> emulator_state{
        let mut state: emulator_state = emulator_state::new(&self);
        return state;
    }
}

impl emulator_state {
    pub fn new(emulator: &Emulator) -> emulator_state{
        let mut blocks:Vec<u16> = vec![];
        for i in emulator.translation_cache.blocks.iter(){
            blocks.push(i.chip8_entry);
        }
        emulator_state {
            pc: emulator.pc,
            index: emulator.index,
            gpr: emulator.gpr,
            stack: emulator.stack,
            blocks: blocks,
         }
    }
}

impl Serialize for emulator_state {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("emulator_state", 5)?;
        state.serialize_field("pc", &self.pc)?;
        state.serialize_field("index", &self.index)?;
        state.serialize_field("gpr", &self.gpr)?;
        state.serialize_field("stack", &self.stack)?;
        state.serialize_field("blocks", &self.blocks)?;
        state.end()
    }
}
