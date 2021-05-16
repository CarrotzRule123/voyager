use crate::ValType;

#[derive(Debug)]
pub struct GlobalInst {
    pub value: ValType
}

impl GlobalInst {
    pub fn new(value: ValType) -> Self {
        GlobalInst { value }
    }
}

