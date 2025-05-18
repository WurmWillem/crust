use crate::{
    error::ParseError,
    object::ObjFunc,
    token::{Literal, Token, TokenType},
    value::{StackValue, ValueType},
    vm::MAX_FUNC_AMT,
};

pub struct DeclaredFuncStack<'a> {
    funcs: [DeclaredFunc<'a>; MAX_FUNC_AMT],
    top: usize,
}
impl<'a> DeclaredFuncStack<'a> {
    pub fn new() -> Self {
        Self {
            funcs: std::array::from_fn(|_| DeclaredFunc::new("", None)),
            top: 0,
        }
    }

    pub fn patch_func(&mut self, name: &'a str, parameters: Vec<ValueType>) {
        self.funcs[self.top].name = name;
        self.funcs[self.top].parameters = parameters;
    }

    pub fn edit_value_and_increment_top(&mut self, value: StackValue) {
        self.funcs[self.top].value = Some(value);
        self.top += 1;
    }

    pub fn to_stack_value_arr(&self) -> [StackValue; MAX_FUNC_AMT] {
        // self.funcs
        //     .map(|func| func.value.unwrap_or(StackValue::Null))
        let mut arr = [StackValue::Null; MAX_FUNC_AMT];
        for (i, func) in self.funcs.iter().enumerate() {
            if let Some(val) = &func.value {
                arr[i] = *val;
            }
        }
        arr
    }

    pub fn resolve_func(&self, name: &str) -> Option<(u8, Vec<ValueType>)> {
        for i in 0..self.funcs.len() {
            if self.funcs[i].name == name {
                // TODO: only read access needed, maybe return reference?
                let parameters = self.funcs[i].parameters.clone();
                return Some((i as u8, parameters));
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
struct DeclaredFunc<'a> {
    name: &'a str,
    parameters: Vec<ValueType>,
    value: Option<StackValue>,
}
impl<'a> DeclaredFunc<'a> {
    fn new(name: &'a str, value: Option<StackValue>) -> Self {
        Self {
            name,
            value,
            parameters: Vec::new(),
        }
    }
}

// TODO: see if you can restrict the visibility of some fields
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

pub struct CompilerStack<'a> {
    compilers: Vec<Compiler<'a>>,
    current: usize,
}
impl<'a> CompilerStack<'a> {
    // create a new stack with a root compiler (no parent)
    pub fn new() -> Self {
        let root = Compiler::new("".to_string());
        Self {
            compilers: vec![root],
            current: 0, // Root is at index 0
        }
    }

    pub fn get_scope_depth(&self) -> usize {
        self.current().scope_depth
    }

    pub fn increment_scope_depth(&mut self) {
        self.compilers[self.current].scope_depth += 1;
    }

    pub fn decrement_scope_depth(&mut self) {
        self.compilers[self.current].scope_depth -= 1;
    }

    pub fn decrement_local_count(&mut self) {
        self.compilers[self.current].local_count -= 1;
    }

    pub fn add_constant(&mut self, value: StackValue) -> usize {
        self.compilers[self.current].func.chunk.add_constant(value)
    }

    pub fn write_byte_to_chunk(&mut self, byte: u8, line: u32) {
        self.compilers[self.current]
            .func
            .chunk
            .write_byte_to_chunk(byte, line);
    }

    pub fn get_code_len(&self) -> usize {
        self.compilers[self.current].func.chunk.code.len()
    }

    pub fn add_local(&mut self, name: Token<'a>, kind: ValueType) -> Result<(), ParseError> {
        if self.current().local_count == MAX_LOCAL_AMT {
            let msg = "Too many local variables in function.";
            return Err(ParseError::new(name.line, msg));
        }

        let local = Local::new(name, self.current().scope_depth, kind);

        let local_count = self.current().local_count;
        self.compilers[self.current].locals[local_count] = local;
        self.compilers[self.current].local_count += 1;
        Ok(())
    }

    pub fn patch_jump(&mut self, offset: usize) -> Result<(), ParseError> {
        let jump = self.get_code_len() - offset - 2;

        if jump > u16::MAX as usize {
            let msg = "Too much code to jump over.";
            return Err(ParseError::new(0, msg));
        }

        self.compilers[self.current].func.chunk.code[offset] = ((jump >> 8) & 0xFF) as u8;
        self.compilers[self.current].func.chunk.code[offset + 1] = (jump & 0xFF) as u8;
        Ok(())
    }

    pub fn push(&mut self, func_name: String) {
        let new_compiler = Compiler::new(func_name);
        self.compilers.push(new_compiler);
        self.current = self.compilers.len() - 1;
    }

    pub fn pop(&mut self) -> Compiler {
        self.current = 0;
        self.compilers.pop().unwrap()
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
        &self.compilers[self.current]
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
pub const PARSE_RULES: [ParseRule; 42] = {
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
        ParseRule { prefix: Unary, infix: Binary, precedence: P::Term, }, // minus
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Term, }, // plus
        none!(), // semicolon
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
