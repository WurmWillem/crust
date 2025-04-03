#[repr(u8)]
pub enum OpCode {
    Return,
    Constant,
}
impl std::convert::From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Return,
            1 => OpCode::Constant,
            _ => panic!("Not a valid opcode"),
        }
    }
}
