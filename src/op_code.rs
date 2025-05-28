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

    AllocArr,
    IndexArr,

    // literals
    True,
    False,
    Null,

    // unary
    Negate,
    Not,

    // binary arithmetic
    Add,
    Sub,
    Mul,
    Div,

    // binary logic
    And,
    Or,

    Equal,
    NotEqual,
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
            10 => OpCode::AllocArr,
            11 => OpCode::IndexArr,
            12 => OpCode::True,
            13 => OpCode::False,
            14 => OpCode::Null,
            15 => OpCode::Negate,
            16 => OpCode::Not,
            17 => OpCode::Add,
            18 => OpCode::Sub,
            19 => OpCode::Mul,
            20 => OpCode::Div,
            21 => OpCode::And,
            22 => OpCode::Or,
            23 => OpCode::Equal,
            24 => OpCode::NotEqual,
            25 => OpCode::Greater,
            26 => OpCode::GreaterEqual,
            27 => OpCode::Less,
            28 => OpCode::LessEqual,
            _ => panic!("Not a valid opcode."),
        }
    }
}
