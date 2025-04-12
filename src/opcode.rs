#[repr(u8)]
pub enum OpCode {
    Return,
    Constant,
    Pop,

    Jump,
    JumpIfFalse,
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
            3 => OpCode::Jump,
            4 => OpCode::JumpIfFalse,
            5 => OpCode::Print,
            6 => OpCode::GetLocal,
            7 => OpCode::SetLocal,
            8 => OpCode::True,
            9 => OpCode::False,
            10 => OpCode::Null,
            11 => OpCode::Negate,
            12 => OpCode::Not,
            13 => OpCode::Add,
            14 => OpCode::Sub,
            15 => OpCode::Mul,
            16 => OpCode::Div,
            17 => OpCode::Equal,
            18 => OpCode::BangEqual,
            19 => OpCode::Greater,
            20 => OpCode::GreaterEqual,
            21 => OpCode::Less,
            22 => OpCode::LessEqual,
            _ => panic!("Not a valid opcode."),
        }
    }
}
