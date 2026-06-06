#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    LSqBrace,  // [
    RSqBrace,  // ]
    Comma,     // ,
    Colon,     // :
    Semicolon, // ;
    Dot,       // .
    DotDot,    // ..

    Num(f64),
    Str(String),

    // Arithmetic Ops
    Plus,
    PlusAssign,
    Minus,
    MinusAssign, // + -
    Star,
    StarAssign,
    Slash,
    SlashAssign, // * /

    Assign, // =

    // Comparators
    Bang,      // !
    BangEqual, // !=

    Gt,
    Lt, // > <
    GtOrEq,
    LtOrEq, // >= <=

    Eq,  // ==

    Pipe,     // |
    PipePipe, // ||

    Amp,    // &
    AmpAmp, // &&
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub pos: usize,
    pub size: usize,
    pub line: usize,
}

pub enum Op {
    // Arithmetic Ops
    Plus,
    PlusAssign,
    Minus,
    MinusAssign, // + -
    Star,
    StarAssign,
    Slash,
    SlashAssign, // * /

    Assign, // =

    // Comparators
    Bang,      // !
    BangEqual, // !=

    Gt,
    Lt, // > <
    GtOrEq,
    LtOrEq, // >= <=

    Eq,  // ==

    Pipe,     // |
    PipePipe, // ||

    Amp,    // &
    AmpAmp, // &&
}

