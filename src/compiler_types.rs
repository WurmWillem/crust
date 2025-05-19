use crate::{
    error::ParseError,
    object::ObjFunc,
    token::{Literal, Token, TokenType},
    value::{StackValue, ValueType},
};

pub struct CompilerStack<'a> {
    comps: Vec<Compiler<'a>>,
    current: usize,
}
impl<'a> CompilerStack<'a> {
    pub fn new() -> Self {
        let root = Compiler::new("".to_string());
        Self {
            comps: vec![root],
            current: 0, // root is at index 0
        }
    }

    pub fn get_return_type(&self) -> ValueType {
        self.current().func.return_type
    }

    pub fn increment_scope_depth(&mut self) {
        self.comps[self.current].scope_depth += 1;
    }

    pub fn decrement_scope_depth(&mut self) {
        self.comps[self.current].scope_depth -= 1;
    }

    pub fn decrement_local_count(&mut self) {
        self.comps[self.current].local_count -= 1;
    }

    pub fn add_constant(&mut self, value: StackValue) -> usize {
        self.comps[self.current].func.chunk.add_constant(value)
    }

    pub fn write_byte_to_chunk(&mut self, byte: u8, line: u32) {
        self.comps[self.current]
            .func
            .chunk
            .write_byte_to_chunk(byte, line);
    }

    pub fn get_code_len(&self) -> usize {
        self.comps[self.current].func.chunk.code.len()
    }

    pub fn add_local(&mut self, name: Token<'a>, kind: ValueType) -> Result<(), ParseError> {
        if self.current().local_count == MAX_LOCAL_AMT {
            let msg = "Too many local variables in function.";
            return Err(ParseError::new(name.line, msg));
        }

        let local = Local::new(name, self.current().scope_depth, kind);

        let local_count = self.current().local_count;
        self.comps[self.current].locals[local_count] = local;
        self.comps[self.current].local_count += 1;
        Ok(())
    }

    pub fn patch_jump(&mut self, offset: usize) -> Result<(), ParseError> {
        let jump = self.get_code_len() - offset - 2;

        if jump > u16::MAX as usize {
            let msg = "Too much code to jump over.";
            return Err(ParseError::new(0, msg));
        }

        self.comps[self.current].func.chunk.code[offset] = ((jump >> 8) & 0xFF) as u8;
        self.comps[self.current].func.chunk.code[offset + 1] = (jump & 0xFF) as u8;
        Ok(())
    }

    pub fn patch_return_type(&mut self, return_type: ValueType) {
        self.comps[self.current].patch_return_type(return_type);
    }

    pub fn push(&mut self, func_name: String) {
        let new_compiler = Compiler::new(func_name);
        self.comps.push(new_compiler);
        self.current = self.comps.len() - 1;
    }

    pub fn pop(&mut self) -> Compiler {
        self.current = 0;
        self.comps.pop().unwrap()
    }

    pub fn resolve_local(&mut self, name: &str) -> Option<(u8, ValueType)> {
        // TODO: shadowing doesn't remove the old var as of now
        for i in (0..self.current().local_count).rev() {
            if self.current().locals[i].name.lexeme == name {
                return Some((i as u8, self.current().locals[i].kind));
            }
        }
        None
    }

    pub fn should_remove_local(&self) -> bool {
        let depth = self.current().locals[self.current().local_count - 1].depth;
        self.current().local_count > 0 && depth > self.current().scope_depth
    }

    fn current(&self) -> &Compiler {
        &self.comps[self.current]
    }
}

#[derive(Debug, Clone, Copy)]
struct Local<'a> {
    name: Token<'a>,
    kind: ValueType,
    depth: usize,
}
impl<'a> Local<'a> {
    fn new(name: Token<'a>, depth: usize, kind: ValueType) -> Self {
        Self { name, depth, kind }
    }
}

const MAX_LOCAL_AMT: usize = u8::MAX as usize;
pub struct Compiler<'a> {
    locals: [Local<'a>; MAX_LOCAL_AMT],
    local_count: usize,
    scope_depth: usize,
    func: ObjFunc,
}
impl<'a> Compiler<'a> {
    pub fn new(func_name: String) -> Self {
        let name = Token::new(TokenType::Equal, "", Literal::None, 0);

        let local = Local::new(name, 0, ValueType::None);
        Self {
            locals: [local; MAX_LOCAL_AMT],
            local_count: 1,
            scope_depth: 0,
            func: ObjFunc::new(func_name),
        }
    }

    pub fn get_func(self) -> ObjFunc {
        self.func
    }

    pub fn patch_return_type(&mut self, return_type: ValueType) {
        self.func.return_type = return_type;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}
impl std::convert::From<u8> for Precedence {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Assignment,
            2 => Self::Or,
            3 => Self::And,
            4 => Self::Equality,
            5 => Self::Comparison,
            6 => Self::Term,
            7 => Self::Factor,
            8 => Self::Unary,
            9 => Self::Call,
            10 => Self::Primary,
            _ => panic!("Not a valid value for Precedence."),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum FnType {
    Empty,
    Grouping,
    Unary,
    Binary,
    Number,
    String,
    Literal,
    Variable,
    Call,
}

#[derive(Clone, Copy)]
pub struct ParseRule {
    pub prefix: FnType, // stores in what way can it be used as prefix (if used at all)
    pub infix: FnType,
    pub precedence: Precedence,
}

#[rustfmt::skip]
pub const PARSE_RULES: [ParseRule; 43] = {
    use FnType::*;
    use Precedence as P;

    macro_rules! none {
        () => {
            ParseRule { prefix: Empty, infix: Empty, precedence: P::None }
        }
    }

    [
        
        ParseRule { prefix: Grouping, infix: Call, precedence: P::Call, }, // left paren
        none!(), // right paren
        none!(), // left brace
        none!(), // right brace
        none!(), // comma
        none!(), // dot
        none!(), // colon
        none!(), // semicolon
        ParseRule { prefix: Unary, infix: Binary, precedence: P::Term, }, // minus
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Term, }, // plus
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Factor, }, // slash
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Factor, }, // star
        ParseRule { prefix: Unary, infix: Empty, precedence: P::Factor, }, // bang
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Comparison, }, // bang equal
        none!(), // equal
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Comparison, }, // equal equal
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Comparison, }, // Greater
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Comparison, }, // Greater equal
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Comparison, }, // Less
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Comparison, }, // Less equal

        none!(), //Plus Equal
        none!(), //Minus Equal
        none!(), //Mul Equal
        none!(), //Div Equal

        ParseRule { prefix: Variable, infix: Empty, precedence: P::None, }, // identifier
        ParseRule { prefix: String, infix: Empty, precedence: P::None, }, // string
        ParseRule { prefix: Number, infix: Empty, precedence: P::None, }, // number
        none!(), // and
        none!(), // class
        none!(), // else
        ParseRule { prefix: Literal, infix: Empty, precedence: P::None, }, // false
        none!(), // for
        none!(), // fun
        none!(), // if
        ParseRule { prefix: Literal, infix: Empty, precedence: P::None, }, // nil
        none!(), // or
        none!(), // print
        none!(), // return
        none!(), // super
        none!(), // this
        ParseRule { prefix: Literal, infix: Empty, precedence: P::None, }, // true
        none!(), // while
        none!(), // EOF
    ]
};
