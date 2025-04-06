#[repr(u8)]
pub enum OpCode {
    Return,
    Constant,

    // literals
    True,
    False,
    Null,

    // unary
    Negate,
    Not,

    // binary
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
            2 => OpCode::True,
            3 => OpCode::False,
            4 => OpCode::Null,
            5 => OpCode::Not,
            6 => OpCode::Negate,
            7 => OpCode::Add,
            8 => OpCode::Sub,
            9 => OpCode::Mul,
            10 => OpCode::Div,
            _ => panic!("Not a valid opcode."),
        }
    }
}
