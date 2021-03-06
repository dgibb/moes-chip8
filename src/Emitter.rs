use executable_block::executable_block;

pub struct Emitter {
    pub page: u64,
    pub iter: u64,
}

impl Emitter {
    pub fn new() -> Emitter{
        Emitter {
            page:0,
            iter:0,
        }
    }

    pub fn emit(&mut self, dyna_rec_code: &Vec<u8>, executable_block: &mut executable_block){
        for (index,i) in dyna_rec_code.iter().enumerate() {
            executable_block[index] = *i;
        }
    }
}
