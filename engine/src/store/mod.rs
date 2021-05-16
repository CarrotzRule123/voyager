mod memory;
mod global;

pub use memory::*;
pub use global::*;

#[derive(Debug)]
pub struct Store {
    pub memory: Vec<MemInst>,
    pub global: Vec<GlobalInst>
}

impl Store {
    pub fn new() -> Self {
        Store {
            memory: Vec::new(),
            global: Vec::new()
        }
    }
}