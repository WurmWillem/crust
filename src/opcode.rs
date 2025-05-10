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

    Call,

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
            7 => OpCode::Call,
            8 => OpCode::GetLocal,
            9 => OpCode::SetLocal,
            10 => OpCode::True,
            11 => OpCode::False,
            12 => OpCode::Null,
            13 => OpCode::Negate,
            14 => OpCode::Not,
            15 => OpCode::Add,
            16 => OpCode::Sub,
            17 => OpCode::Mul,
            18 => OpCode::Div,
            19 => OpCode::Equal,
            20 => OpCode::BangEqual,
            21 => OpCode::Greater,
            22 => OpCode::GreaterEqual,
            23 => OpCode::Less,
            24 => OpCode::LessEqual,
            _ => panic!("Not a valid opcode."),
        }
    }
}
