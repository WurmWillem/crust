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
    SetProp,

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
            12 => OpCode::SetProp,
            13 => OpCode::True,
            14 => OpCode::False,
            15 => OpCode::Null,
            16 => OpCode::Negate,
            17 => OpCode::Not,
            18 => OpCode::Add,
            19 => OpCode::Sub,
            20 => OpCode::Mul,
            21 => OpCode::Div,
            22 => OpCode::Equal,
            23 => OpCode::BangEqual,
            24 => OpCode::Greater,
            25 => OpCode::GreaterEqual,
            26 => OpCode::Less,
            27 => OpCode::LessEqual,
            _ => panic!("Not a valid opcode."),
        }
    }
}
