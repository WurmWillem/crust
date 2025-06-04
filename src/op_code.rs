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
    SetField,

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
            15 => OpCode::GetField,
            16 => OpCode::SetField,
            17 => OpCode::IndexArr,
            18 => OpCode::AssignIndex,
            19 => OpCode::True,
            20 => OpCode::False,
            21 => OpCode::Null,
            22 => OpCode::Negate,
            23 => OpCode::Not,
            24 => OpCode::Add,
            25 => OpCode::Sub,
            26 => OpCode::Mul,
            27 => OpCode::Div,
            28 => OpCode::And,
            29 => OpCode::Or,
            30 => OpCode::Equal,
            31 => OpCode::NotEqual,
            32 => OpCode::Greater,
            33 => OpCode::GreaterEqual,
            34 => OpCode::Less,
            35 => OpCode::LessEqual,
            _ => panic!("Not a valid opcode."),
        }
    }
}
