#[repr(u8)]
pub enum OpCode {
    Return,
    Constant,

    Print,

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

    Equal,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}
impl std::convert::From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Return,
            1 => OpCode::Constant,
            2 => OpCode::Print,
            3 => OpCode::True,
            4 => OpCode::False,
            5 => OpCode::Null,
            6 => OpCode::Negate,
            7 => OpCode::Not,
            8 => OpCode::Add,
            9 => OpCode::Sub,
            10 => OpCode::Mul,
            11 => OpCode::Div,
            12 => OpCode::Equal,
            13 => OpCode::BangEqual,
            14 => OpCode::Greater,
            15 => OpCode::GreaterEqual,
            16 => OpCode::Less,
            17 => OpCode::LessEqual,
            _ => panic!("Not a valid opcode."),
        }
    }
}
