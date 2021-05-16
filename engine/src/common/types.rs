#[derive(Debug, Copy, Clone)]
pub enum NumTypes {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

#[derive(Debug, Copy, Clone)]
pub enum RefTypes {
    FuncRef,
    ExternRef
}

#[derive(Debug, Copy, Clone)]
pub enum ValType {
    NumTypes(NumTypes),
    RefTypes(RefTypes)
}