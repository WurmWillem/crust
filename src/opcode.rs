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

    GetProp,

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
            11 => OpCode::GetProp,
            12 => OpCode::True,
            13 => OpCode::False,
            14 => OpCode::Null,
            15 => OpCode::Negate,
            16 => OpCode::Not,
            17 => OpCode::Add,
            18 => OpCode::Sub,
            19 => OpCode::Mul,
            20 => OpCode::Div,
            21 => OpCode::Equal,
            22 => OpCode::BangEqual,
            23 => OpCode::Greater,
            24 => OpCode::GreaterEqual,
            25 => OpCode::Less,
            26 => OpCode::LessEqual,
            _ => panic!("Not a valid opcode."),
        }
    }
}
