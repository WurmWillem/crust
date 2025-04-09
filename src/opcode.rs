#[repr(u8)]
pub enum OpCode {
    Return,
    Constant,

    Pop,

    Print,
    DefineGlobal,
    GetGlobal,
    SetGlobal,

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
            2 => OpCode::Pop,
            3 => OpCode::Print,
            4 => OpCode::DefineGlobal,
            5 => OpCode::GetGlobal,
            6 => OpCode::SetGlobal,
            7 => OpCode::True,
            8 => OpCode::False,
            9 => OpCode::Null,
            10 => OpCode::Negate,
            11 => OpCode::Not,
            12 => OpCode::Add,
            13 => OpCode::Sub,
            14 => OpCode::Mul,
            15 => OpCode::Div,
            16 => OpCode::Equal,
            17 => OpCode::BangEqual,
            18 => OpCode::Greater,
            19 => OpCode::GreaterEqual,
            20 => OpCode::Less,
            21 => OpCode::LessEqual,
            _ => panic!("Not a valid opcode."),
        }
    }
}
