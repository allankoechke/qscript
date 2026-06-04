mod lexer;
mod token;
mod parser;

#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Colon,
    Semicolon,
    Dot,

    Num(f64),
    Str(String),

    // Arithmetic Ops
    Plus, Minus, // + -
    Star, Slash, // * /

    Assign, // =

    // Comparators
    Bang, // !
    BangEqual, // !=


    Gt, Lt, // > <
    GtOrEq, LtOrEq, // >= <=

    Eq, // ==
    NEq, // !

    Pipe, // |
    PipePipe, // ||

    Amp, // &
    AmpAmp, // &&
}

#[derive(Debug, PartialEq, Clone)]
struct Token {
    ttype: TokenType,
    lexeme: String,
    pos: usize,
    size: usize,
}

#[derive(Debug, PartialEq, Clone)]
struct Lexer {
    src: String,
    pos: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(src: String) -> Lexer {
        Lexer { src, pos: 0, tokens: vec![] }
    }

    fn lex(&mut self) -> Vec<Token> {
        while self.pos < self.src.len() {
            let c = &self.src.as_bytes()[self.pos];
            if c.is_ascii_whitespace() {
                self.lex_whitespaces();
            } else if c.is_ascii_digit() {
                self.lex_num();
            } else if c.is_ascii_alphabetic() || c.to_string() == "_" {
                self.lex_ident();
            } else if c.to_string() == "\"" || c.to_string() == "'" {
                self.lex_str();
            } else {
                self.lex_ops();
            }
        }

        self.tokens.clone()
    }

    fn lex_str(&mut self) {
        panic!("Unknown character at position {}", self.pos)
    }

    fn lex_num(&mut self) {
        let mut has_dot = false;
        let mut num = String::new();
        let start_pos = self.pos;

        while self.pos < self.src.len() &&
            self.src.as_bytes()[self.pos].is_ascii_digit() ||
            self.src.as_bytes()[self.pos].to_string() == "." {
            let c = self.src.chars().nth(self.pos).unwrap();

            if c == '.' {
                if has_dot {
                    break;
                }

                has_dot = true;
            }

            self.pos += 1;
            num.push(c);
        }

        self.create_token(
            if has_dot {
                TokenType::Num(num.parse::<f64>().unwrap())
            } else {
                TokenType::Num(num.parse::<i64>().unwrap() as f64)
            },
                          num, start_pos);
    }

    fn lex_ops(&mut self) {
        panic!("Unknown character at position {}", self.pos)
    }

    fn lex_whitespaces(&mut self) {
        while self.pos < self.src.len() && self.src.as_bytes()[self.pos].is_ascii_whitespace() {
            self.pos += 1; // Skip whitespaces
        }
    }

    fn lex_ident(&mut self) {
        panic!("Unknown character at position {}", self.pos)
    }

    fn create_token(&mut self, ttype: TokenType, lexeme: String, pos: usize) {
        let s = lexeme.len();
        self.tokens.push(Token { ttype, lexeme, pos, size: s });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_lex() {
        let inp = "34";
        let mut lexer = Lexer::new(inp.to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].ttype, TokenType::Num(34.0));
    }
}


fn main() {
    println!("Hello, world!");
}
