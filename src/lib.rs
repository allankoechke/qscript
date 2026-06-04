mod lexer;
mod parser;
mod token;

mod prelude {
    pub use crate::lexer::Lexer;
    pub use crate::token::{Token, TokenType};
    // pub use crate::parser::Parser;
}
