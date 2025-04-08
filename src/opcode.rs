#[repr(u8)]
pub enum OpCode {
    Return,
    Constant,

    Print,
    DefineGlobal,

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
            3 => OpCode::DefineGlobal,
            4 => OpCode::True,
            5 => OpCode::False,
            6 => OpCode::Null,
            7 => OpCode::Negate,
            8 => OpCode::Not,
            9 => OpCode::Add,
            10 => OpCode::Sub,
            11 => OpCode::Mul,
            12 => OpCode::Div,
            13 => OpCode::Equal,
            14 => OpCode::BangEqual,
            15 => OpCode::Greater,
            16 => OpCode::GreaterEqual,
            17 => OpCode::Less,
            18 => OpCode::LessEqual,
            _ => panic!("Not a valid opcode."),
        }
    }
}
