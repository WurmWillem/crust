#[repr(u8)]
pub enum OpCode {
    Return,
    Constant,

    Pop,

    Print,
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
            4 => OpCode::GetLocal,
            5 => OpCode::SetLocal,
            6 => OpCode::True,
            7 => OpCode::False,
            8 => OpCode::Null,
            9 => OpCode::Negate,
            10 => OpCode::Not,
            11 => OpCode::Add,
            12 => OpCode::Sub,
            13 => OpCode::Mul,
            14 => OpCode::Div,
            15 => OpCode::Equal,
            16 => OpCode::BangEqual,
            17 => OpCode::Greater,
            18 => OpCode::GreaterEqual,
            19 => OpCode::Less,
            20 => OpCode::LessEqual,
            _ => panic!("Not a valid opcode."),
        }
    }
}
