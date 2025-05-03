#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    Return,
    Constant,
    Pop,

    Jump,
    JumpIfFalse,
    Loop,

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
            5 => OpCode::Loop,
            6 => OpCode::Print,
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
