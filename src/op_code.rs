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
    MethodCall,

    GetLocal,
    SetLocal,

    AllocArr,
    IndexArr,
    AssignIndex,

    AllocInstance,
    GetProperty,
    SetProperty,
    GetField,

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
            8 => OpCode::MethodCall,
            9 => OpCode::GetLocal,
            10 => OpCode::SetLocal,
            11 => OpCode::AllocArr,
            12 => OpCode::AllocInstance,
            13 => OpCode::GetProperty,
            14 => OpCode::SetProperty,
            15 => OpCode::IndexArr,
            16 => OpCode::AssignIndex,
            17 => OpCode::True,
            18 => OpCode::False,
            19 => OpCode::Null,
            20 => OpCode::Negate,
            21 => OpCode::Not,
            22 => OpCode::Add,
            23 => OpCode::Sub,
            24 => OpCode::Mul,
            25 => OpCode::Div,
            26 => OpCode::And,
            27 => OpCode::Or,
            28 => OpCode::Equal,
            29 => OpCode::NotEqual,
            30 => OpCode::Greater,
            31 => OpCode::GreaterEqual,
            32 => OpCode::Less,
            33 => OpCode::LessEqual,
            _ => panic!("Not a valid opcode."),
        }
    }
}
