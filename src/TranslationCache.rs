use executable_block::executable_block;

pub struct TranslationCache {
    pub blocks: Vec<executable_block>,
    pub second_chance: Vec<bool>,
}

impl TranslationCache {
    pub fn new() -> TranslationCache{
        TranslationCache {
            blocks: vec![],
            second_chance: vec![],
        }
    }

    pub fn add(&mut self, ex_block: executable_block){
        self.blocks.push(ex_block);
    }
}
