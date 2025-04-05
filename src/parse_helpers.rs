pub struct ParseError {
    pub msg: String,
    pub line: u32,
}
impl ParseError {
    pub fn new(line: u32, msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
            line,
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
        // left paren
        ParseRule { prefix: Grouping, infix: Empty, precedence: P::None, },
        none!(), // right paren
        none!(), // left brace
        none!(), // right brace
        none!(), // comma
        none!(), // dot
        // minus
        ParseRule { prefix: Unary, infix: Binary, precedence: P::Term, },
        // plus
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Term, },
                 //
        none!(), // semicolon
        // slash
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Factor, },
        // star
        ParseRule { prefix: Empty, infix: Binary, precedence: P::Factor, },
        none!(), // bang
        none!(), // bang equal
        none!(), // equal
        none!(), // equal equal
        none!(), // greater
        none!(), // greater equal
        none!(), // less
        none!(), // less equal
        none!(), // identifier
        none!(), // string
        // number
        ParseRule { prefix: Number, infix: Empty, precedence: P::None, },
        none!(), // and
        none!(), // class
        none!(), // else
        none!(), // false
        none!(), // for
        none!(), // fun
        none!(), // if
        none!(), // nil
        none!(), // or
        none!(), // print
        none!(), // return
        none!(), // super
        none!(), // this
        none!(), // true
        none!(), // var
        none!(), // while
        none!(), // EOF
    ]
};
