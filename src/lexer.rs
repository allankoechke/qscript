use crate::token::{Token, TokenType};

#[derive(Debug, PartialEq, Clone)]
pub struct Lexer {
    src: String,
    pos: usize,
    tokens: Vec<Token>,
    line: usize,
}

impl Lexer {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            src: src.to_string(),
            pos: 0,
            tokens: vec![],
            line: 1,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        while self.pos < self.src.len() {
            // Get the char at the current position and decide what to do based on it
            let c = &self.src.chars().nth(self.pos).unwrap();

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

        while self.pos < self.src.len()
            && (self.src.chars().nth(self.pos).unwrap().is_ascii_digit()
                || self.src.chars().nth(self.pos).unwrap() == '.')
        {
            let c = self.src.chars().nth(self.pos).unwrap();
            // println!("Char: {}", c);

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
            num,
            start_pos,
        );
    }

    fn lex_ops(&mut self) {
        while self.pos < self.src.len() {
            let c = self.src.chars().nth(self.pos).unwrap();
            match c {
                '(' => {
                    self.create_token(TokenType::LParen, "(".to_string(), self.pos);
                    self.pos += 1;
                }
                ')' => {
                    self.create_token(TokenType::RParen, ")".to_string(), self.pos);
                    self.pos += 1;
                }
                '[' => {
                    self.create_token(TokenType::LSqBrace, "[".to_string(), self.pos);
                    self.pos += 1;
                }
                ']' => {
                    self.create_token(TokenType::RSqBrace, "]".to_string(), self.pos);
                    self.pos += 1;
                }
                '{' => {
                    self.create_token(TokenType::LBrace, "{".to_string(), self.pos);
                    self.pos += 1;
                }
                '}' => {
                    self.create_token(TokenType::RBrace, "}".to_string(), self.pos);
                    self.pos += 1;
                }
                ',' => {
                    self.create_token(TokenType::Comma, ",".to_string(), self.pos);
                    self.pos += 1;
                }
                ':' => {
                    self.create_token(TokenType::Colon, ":".to_string(), self.pos);
                    self.pos += 1;
                }
                ';' => {
                    self.create_token(TokenType::Semicolon, ";".to_string(), self.pos);
                    self.pos += 1;
                }
                '.' => {
                    if self.src.chars().nth(self.pos + 1).unwrap() == '.' {
                        self.create_token(TokenType::DotDot, "..".to_string(), self.pos);
                        self.pos += 2;
                    } else {
                        self.create_token(TokenType::Dot, ".".to_string(), self.pos);
                        self.pos += 1;
                    }
                }
                '+' => {
                    if self.src.chars().nth(self.pos + 1).unwrap() == '=' {
                        self.create_token(TokenType::PlusAssign, "+=".to_string(), self.pos);
                        self.pos += 2;
                    } else {
                        self.create_token(TokenType::Plus, "+".to_string(), self.pos);
                        self.pos += 1;
                    }
                }
                '-' => {
                    if self.src.chars().nth(self.pos + 1).unwrap() == '=' {
                        self.create_token(TokenType::MinusAssign, "-=".to_string(), self.pos);
                        self.pos += 2;
                    } else {
                        self.create_token(TokenType::Minus, "-".to_string(), self.pos);
                        self.pos += 1;
                    }
                }
                '*' => {
                    if self.src.chars().nth(self.pos + 1).unwrap() == '=' {
                        self.create_token(TokenType::StarAssign, "*=".to_string(), self.pos);
                        self.pos += 2;
                    } else {
                        self.create_token(TokenType::Star, "*".to_string(), self.pos);
                        self.pos += 1;
                    }
                }
                '/' => {
                    if self.src.chars().nth(self.pos + 1).unwrap() == '=' {
                        self.create_token(TokenType::SlashAssign, "/=".to_string(), self.pos);
                        self.pos += 2;
                    } else {
                        self.create_token(TokenType::Slash, "/".to_string(), self.pos);
                        self.pos += 1;
                    }
                }
                '>' => {
                    if self.src.chars().nth(self.pos + 1).unwrap() == '=' {
                        self.create_token(TokenType::GtOrEq, ">=".to_string(), self.pos);
                        self.pos += 2;
                    } else {
                        self.create_token(TokenType::Gt, ">".to_string(), self.pos);
                        self.pos += 1;
                    }
                }
                '<' => {
                    if self.src.chars().nth(self.pos + 1).unwrap() == '=' {
                        self.create_token(TokenType::LtOrEq, "<=".to_string(), self.pos);
                        self.pos += 2;
                    } else {
                        self.create_token(TokenType::Lt, "<".to_string(), self.pos);
                        self.pos += 1;
                    }
                }
                '=' => {
                    if self.src.chars().nth(self.pos + 1).unwrap() == '=' {
                        self.create_token(TokenType::Eq, "==".to_string(), self.pos);
                        self.pos += 2;
                    } else {
                        self.create_token(TokenType::Assign, "=".to_string(), self.pos);
                        self.pos += 1;
                    }
                }
                '!' => {
                    if self.src.chars().nth(self.pos + 1).unwrap() == '=' {
                        self.create_token(TokenType::BangEqual, "!=".to_string(), self.pos);
                        self.pos += 2;
                    } else {
                        self.create_token(TokenType::Bang, "!".to_string(), self.pos);
                    }
                }
                '&' => {
                    if self.src.chars().nth(self.pos + 1).unwrap() == '&' {
                        self.create_token(TokenType::AmpAmp, "&&".to_string(), self.pos);
                        self.pos += 2;
                    } else {
                        self.create_token(TokenType::Amp, "&".to_string(), self.pos);
                        self.pos += 1;
                    }
                }
                '|' => {
                    if self.src.chars().nth(self.pos + 1).unwrap() == '|' {
                        self.create_token(TokenType::PipePipe, "||".to_string(), self.pos);
                        self.pos += 2;
                    } else {
                        self.create_token(TokenType::Pipe, "|".to_string(), self.pos);
                        self.pos += 1;
                    }
                }
                _ => {
                    // If we reach here, it means we have an unknown character. We can either panic or skip it. For now, let's panic.
                    panic!("Unknown character '{}' at position {}", c, self.pos)
                },
            }
        }
    }

    fn lex_whitespaces(&mut self) {
        while self.pos < self.src.len()
            && self
                .src
                .chars()
                .nth(self.pos)
                .unwrap()
                .is_ascii_whitespace()
        {
            // Count lines
            if self.src.chars().nth(self.pos).unwrap() == '\n' {
                self.line += 1;
            }

            println!("Whitespace at position {} `{}`", self.pos, self.src.chars().nth(self.pos).unwrap());

            self.pos += 1; // Skip whitespaces
        }
    }

    fn lex_ident(&mut self) {
        panic!("Unknown character at position {}", self.pos)
    }

    fn create_token(&mut self, ttype: TokenType, lexeme: String, pos: usize) {
        let s = lexeme.len();
        self.tokens.push(Token {
            ttype,
            lexeme,
            pos,
            size: s,
            line: self.line,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_num() {
        let inp = "34";
        let mut lexer = Lexer::new(inp);
        let tokens = lexer.lex();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.iter().nth(0).unwrap().ttype, TokenType::Num(34.0));
    }

    #[test]
    fn lex_string() {
        let inp = "\"Hello, World!\"";
        let mut lexer = Lexer::new(inp);
        let tokens = lexer.lex();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.iter().nth(0).unwrap().ttype, TokenType::Str("Hello, World!".to_string()));
    }

    #[test]
    fn lex_expr() {
        let inp = "34 + 5 * (2 - 8)";
        let mut lexer = Lexer::new(inp);
        let tokens = lexer.lex();
        println!("{:?}", tokens);
        assert_eq!(tokens.len(), 9);
    }
}
