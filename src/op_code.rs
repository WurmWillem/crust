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
            12 => OpCode::IndexArr,
            13 => OpCode::AssignIndex,
            14 => OpCode::True,
            15 => OpCode::False,
            16 => OpCode::Null,
            17 => OpCode::Negate,
            18 => OpCode::Not,
            19 => OpCode::Add,
            20 => OpCode::Sub,
            21 => OpCode::Mul,
            22 => OpCode::Div,
            23 => OpCode::And,
            24 => OpCode::Or,
            25 => OpCode::Equal,
            26 => OpCode::NotEqual,
            27 => OpCode::Greater,
            28 => OpCode::GreaterEqual,
            29 => OpCode::Less,
            30 => OpCode::LessEqual,
            _ => panic!("Not a valid opcode."),
        }
    }
}
