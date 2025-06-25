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
    PushMethod,

    GetLocal,
    SetLocal,

    AllocArr,
    AllocInstance,

    GetPubField,
    SetPubField,
    GetSelfField,
    SetSelfField,

    IndexArr,
    AssignIndex,

    CastToI64,
    CastToU64,
    CastToF64,

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
            8 => OpCode::PushMethod,
            9 => OpCode::GetLocal,
            10 => OpCode::SetLocal,
            11 => OpCode::AllocArr,
            12 => OpCode::AllocInstance,
            13 => OpCode::GetPubField,
            14 => OpCode::SetPubField,
            15 => OpCode::GetSelfField,
            16 => OpCode::SetSelfField,
            17 => OpCode::IndexArr,
            18 => OpCode::AssignIndex,
            19 => OpCode::CastToI64,
            20 => OpCode::CastToU64,
            21 => OpCode::CastToF64,
            22 => OpCode::True,
            23 => OpCode::False,
            24 => OpCode::Null,
            25 => OpCode::Negate,
            26 => OpCode::Not,
            27 => OpCode::Add,
            28 => OpCode::Sub,
            29 => OpCode::Mul,
            30 => OpCode::Div,
            31 => OpCode::And,
            32 => OpCode::Or,
            33 => OpCode::Equal,
            34 => OpCode::NotEqual,
            35 => OpCode::Greater,
            36 => OpCode::GreaterEqual,
            37 => OpCode::Less,
            38 => OpCode::LessEqual,
            _ => panic!("Not a valid opcode."),
        }
    }
}
