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

    FuncCall,

    GetLocal,
    SetLocal,

    AllocArr,
    IndexArr,
    AssignIndex,

    AllocInstance,
    GetProperty,

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
            7 => OpCode::FuncCall,
            8 => OpCode::GetLocal,
            9 => OpCode::SetLocal,
            10 => OpCode::AllocArr,
            11 => OpCode::AllocInstance,
            12 => OpCode::GetProperty,
            13 => OpCode::IndexArr,
            14 => OpCode::AssignIndex,
            15 => OpCode::True,
            16 => OpCode::False,
            17 => OpCode::Null,
            18 => OpCode::Negate,
            19 => OpCode::Not,
            20 => OpCode::Add,
            21 => OpCode::Sub,
            22 => OpCode::Mul,
            23 => OpCode::Div,
            24 => OpCode::And,
            25 => OpCode::Or,
            26 => OpCode::Equal,
            27 => OpCode::NotEqual,
            28 => OpCode::Greater,
            29 => OpCode::GreaterEqual,
            30 => OpCode::Less,
            31 => OpCode::LessEqual,
            _ => panic!("Not a valid opcode."),
        }
    }
}
