use crate::{
    object::ObjFunc,
    token::{Literal, Token, TokenType},
    value::ValueType,
};

// TODO: see if you can restrict the visibility of some fields

#[derive(Debug, Clone, Copy)]
pub struct Local<'a> {
    pub name: Token<'a>,
    pub kind: ValueType,
    pub depth: usize,
}
impl<'a> Local<'a> {
    pub fn new(name: Token<'a>, depth: usize, kind: ValueType) -> Self {
        Self { name, depth, kind }
    }
}

pub struct CompilerStack<'a> {
    pub compilers: Vec<Compiler<'a>>,
    pub current: usize,
}
impl<'a> CompilerStack<'a> {
    // Create a new stack with a root compiler (no parent)
    pub fn new() -> Self {
        let root = Compiler::new(None, "".to_string());
        Self {
            compilers: vec![root],
            current: 0, // Root is at index 0
        }
    }

    // Push a new compiler onto the stack, with the current compiler as its parent
    pub fn push(&mut self, func_name: String) {
        // dbg!(534345);
        let new_compiler = Compiler::new(Some(self.current), func_name);
        self.compilers.push(new_compiler);
        self.current = self.compilers.len() - 1; // Update current to the new compiler
    }

    // Pop the current compiler and restore the enclosing one
    pub fn pop(&mut self) -> Compiler {
        let c = self.compilers.pop().unwrap();
        if let Some(parent_idx) = c.enclosing {
            self.current = parent_idx;
        }
        c
    }

    // Get the current compiler (immutable)
    pub fn current(&self) -> &Compiler {
        &self.compilers[self.current]
    }
}

pub const MAX_LOCAL_AMT: usize = u8::MAX as usize;
pub struct Compiler<'a> {
    pub enclosing: Option<usize>,
    pub locals: [Local<'a>; MAX_LOCAL_AMT],
    pub local_count: usize,
    pub scope_depth: usize,
    pub func: ObjFunc,
}
impl<'a> Compiler<'a> {
    pub fn new(enclosing: Option<usize>, func_name: String) -> Self {
        let name = Token::new(TokenType::Equal, "", Literal::None, 0);

        let local = Local::new(name, 0, ValueType::None);
        // let locals = [local; MAX_LOCAL_AMT];
        Self {
            enclosing,
            locals: [local; MAX_LOCAL_AMT],
            local_count: 1,
            scope_depth: 0,
            func: ObjFunc::new(func_name),
        }
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
    Grouping,
    Unary,
    Binary,
    Number,
    String,
    Literal,
    Variable,
    Empty,
}

#[derive(Clone, Copy)]
pub struct ParseRule {
    pub prefix: FnType,
    pub infix: FnType,
    pub precedence: Precedence,
}

#[rustfmt::skip]
pub const PARSE_RULES: [ParseRule; 39] = {
    use FnType::*;
    use Precedence as P;

    macro_rules! none {
        () => {
            ParseRule { prefix: Empty, infix: Empty, precedence: P::None }
        }
    }

    [
        
        ParseRule { prefix: Grouping, infix: Empty, precedence: P::None, }, // left paren
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
        none!(), // var
        none!(), // while
        none!(), // EOF
    ]
};
