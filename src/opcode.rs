#[repr(u8)]
pub enum OpCode {
    Return,
    Constant,

    Print,
    DefineGlobal,
    GetGlobal,

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
            4 => OpCode::GetGlobal,
            5 => OpCode::True,
            6 => OpCode::False,
            7 => OpCode::Null,
            8 => OpCode::Negate,
            9 => OpCode::Not,
            10 => OpCode::Add,
            11 => OpCode::Sub,
            12 => OpCode::Mul,
            13 => OpCode::Div,
            14 => OpCode::Equal,
            15 => OpCode::BangEqual,
            16 => OpCode::Greater,
            17 => OpCode::GreaterEqual,
            18 => OpCode::Less,
            19 => OpCode::LessEqual,
            _ => panic!("Not a valid opcode."),
        }
    }
}
