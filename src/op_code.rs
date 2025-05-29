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
    AssignIndex,

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
            12 => OpCode::AssignIndex,
            13 => OpCode::True,
            14 => OpCode::False,
            15 => OpCode::Null,
            16 => OpCode::Negate,
            17 => OpCode::Not,
            18 => OpCode::Add,
            19 => OpCode::Sub,
            20 => OpCode::Mul,
            21 => OpCode::Div,
            22 => OpCode::And,
            23 => OpCode::Or,
            24 => OpCode::Equal,
            25 => OpCode::NotEqual,
            26 => OpCode::Greater,
            27 => OpCode::GreaterEqual,
            28 => OpCode::Less,
            29 => OpCode::LessEqual,
            _ => panic!("Not a valid opcode."),
        }
    }
}
