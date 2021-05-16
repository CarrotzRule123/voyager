use crate::stack::Stack;
use crate::store::{Store, GlobalInst};
use wasmparser::Operator;

#[derive(Debug)]
pub struct Engine<'a> {
  store: Store,
  ip: *const Operator<'a>,
  stack: Stack,
}

pub enum InterpretResult {
  Ok(),
  CompileError(),
  RuntimeError(),
}

impl<'a> Engine<'a> {
  pub fn new() -> Self {
    let store = Store::new();
    let ip = &Operator::End;
    let stack = Stack::new();

    Self { store, ip, stack }
  }

  pub fn get_store(&mut self) -> &mut Store {
    &mut self.store
  }

  pub fn interpret(&mut self) -> InterpretResult {
    InterpretResult::Ok()
  }

  pub fn run(&mut self, op: Operator) -> InterpretResult {
    match op {
      Operator::GlobalGet { global_index } => {
        let var = &self.store.global[global_index as usize];
        self.stack.push(var.value)
      },
      Operator::GlobalSet { global_index } => {
        let var = self.stack.pop();
        self.store.global[global_index as usize] = GlobalInst::new(var)
      },
      _ => return InterpretResult::RuntimeError()
    };
    return InterpretResult::Ok();
  }
}
