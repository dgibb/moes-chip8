use TranslationCache::TranslationCache;
use Interpreter::Interpreter;
use Emitter::Emitter;
use executable_block::executable_block;

pub struct Emulator {
    pub rom: Vec<u8>,
    pub memory: [u8;0x1000],
    pub pc: u16,
    pub index: u16,
    pub gpr: [u8;16],
    pub stack: [u16;16],
    pub translation_cache: TranslationCache,
    pub interpreter : Interpreter,
    pub emitter : Emitter,
}

unsafe impl Send for Emulator {}
unsafe impl Sync for Emulator {}

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

    pub fn get_reentry_point(&mut self) -> Option< fn() -> u64> {

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
        let mut ex_block = executable_block::new(1);
        self.emitter.emit(&mut self.interpreter.compile_block(&self.pc, &self.memory), &mut ex_block);
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
        let return_val = (fn_ptr.unwrap())();
        self.evaluate_interrupt(return_val);

    }

    pub fn evaluate_interrupt(&mut self, t0: u64 ){
        match t0{

            0x3400 => {
                println!("congrats you got the correct exit code: {:X}", t0);
                self.gpr[9];
            }

            _ => {
                println!("Error! you got the wrong exit code: {:X}", t0);
            }
        }
    }

    pub fn init(&mut self){

        //write rom into memory
        for (index, i) in self.rom.iter().enumerate() {
            self.memory[0x200+index]=*i;
            if index < 10 {
                println!("placing {:?} to {:?}", *i, 0x200+index);
            }
        }
    }
}
