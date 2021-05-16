use crate::common::{ValType, NumTypes};

const STACK_MAX: usize = 256;

#[derive(Debug)]
pub struct Stack {
    stack: [ValType; STACK_MAX],
    top: *mut ValType
}

impl Stack {
    pub fn new() -> Stack {
        let mut stack = [ValType::NumTypes(NumTypes::I32(0)); STACK_MAX];
        Stack { stack, top: &mut stack[0]}
    }

    pub fn reset(&mut self) {
        self.top = &mut self.stack[0];
    }

    pub fn push(&mut self, value: ValType) {
        unsafe {
            *self.top = value;
            self.top = self.top.add(1);
        }
    }

    pub fn pop(&mut self) -> ValType {
        unsafe {
            self.top = self.top.add(1);
            return *self.top;
        }
    }
}