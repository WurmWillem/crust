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

    GetFunc,

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
            10 => OpCode::GetFunc,
            11 => OpCode::True,
            12 => OpCode::False,
            13 => OpCode::Null,
            14 => OpCode::Negate,
            15 => OpCode::Not,
            16 => OpCode::Add,
            17 => OpCode::Sub,
            18 => OpCode::Mul,
            19 => OpCode::Div,
            20 => OpCode::Equal,
            21 => OpCode::BangEqual,
            22 => OpCode::Greater,
            23 => OpCode::GreaterEqual,
            24 => OpCode::Less,
            25 => OpCode::LessEqual,
            _ => panic!("Not a valid opcode."),
        }
    }
}
