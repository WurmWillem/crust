#[repr(u8)]
pub enum OpCode {
    Return,
    Constant,
    Negate,

    Add,
    Sub,
    Mul,
    Div,
}
impl std::convert::From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Return,
            1 => OpCode::Constant,
            2 => OpCode::Negate,
            3 => OpCode::Add,
            4 => OpCode::Sub,
            5 => OpCode::Mul,
            6 => OpCode::Div,
            _ => panic!("Not a valid opcode."),
        }
    }
}
