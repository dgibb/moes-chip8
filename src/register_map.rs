pub struct register_map {
    //[chip8 reg, x86 reg, second chance bit]
    pub map: Vec<[u8;3]>,
    pub valid_regs:[u8;8],
}

impl register_map {
    pub fn new() -> register_map{
        register_map {
            map: vec![],
            valid_regs: [ 0, 1, 2, 3, 4, 5, 6, 7],  //al, cl, dl, bl, ah, ch, dh, bh
        }
    }

    pub fn get_x86_reg(&mut self, chip8_register: u8) -> u8 {
        let mut ret_val: u8 = 0xFF;
        for i in self.map.iter() {
            if i[0] == chip8_register {
                ret_val = i[1];
            }
        }
        return ret_val;
    }

    pub fn map(&mut self, chip8_register: u8){
        if !self.contains_chip8_reg(chip8_register){
            let index: u8 = self.map.len() as u8;
            self.map.push([chip8_register, self.valid_regs[index as usize] , 0])
        }
    }

    pub fn contains_chip8_reg(&mut self, reg: u8) -> bool {
        for i in self.map.iter(){
            if i[0] == reg {
                return true;
            }
        }
        return false;
    }

    pub fn full(&mut self) -> bool {
        if self.map.len() == 8 {
            return true;
        }
        return false;
    }
}
