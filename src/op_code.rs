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
    SetProperty,

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
            13 => OpCode::SetProperty,
            14 => OpCode::IndexArr,
            15 => OpCode::AssignIndex,
            16 => OpCode::True,
            17 => OpCode::False,
            18 => OpCode::Null,
            19 => OpCode::Negate,
            20 => OpCode::Not,
            21 => OpCode::Add,
            22 => OpCode::Sub,
            23 => OpCode::Mul,
            24 => OpCode::Div,
            25 => OpCode::And,
            26 => OpCode::Or,
            27 => OpCode::Equal,
            28 => OpCode::NotEqual,
            29 => OpCode::Greater,
            30 => OpCode::GreaterEqual,
            31 => OpCode::Less,
            32 => OpCode::LessEqual,
            _ => panic!("Not a valid opcode."),
        }
    }
}
