#[repr(u8)]
pub enum OpCode {
    Return,
    Constant,

    Pop,

    Print,
    DefineGlobal,
    GetGlobal,
    SetGlobal,
    GetLocal,
    SetLocal,

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
            7 => OpCode::GetLocal,
            8 => OpCode::SetLocal,
            9 => OpCode::True,
            10 => OpCode::False,
            11 => OpCode::Null,
            12 => OpCode::Negate,
            13 => OpCode::Not,
            14 => OpCode::Add,
            15 => OpCode::Sub,
            16 => OpCode::Mul,
            17 => OpCode::Div,
            18 => OpCode::Equal,
            19 => OpCode::BangEqual,
            20 => OpCode::Greater,
            21 => OpCode::GreaterEqual,
            22 => OpCode::Less,
            23 => OpCode::LessEqual,
            _ => panic!("Not a valid opcode."),
        }
    }
}
