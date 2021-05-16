// use crate::EngineError;

#[derive(Debug)]
pub struct Limit {
    min: u32,
    max: u32
}

#[derive(Debug)]
pub struct MemInst {
    pub memtype: Limit,
    pub data: Vec<u8>
}

impl MemInst {
    pub fn grow(&self) {
        // if self.data.len() < {
            
        // }
    }
}