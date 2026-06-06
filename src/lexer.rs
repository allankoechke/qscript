use crate::token::{Token, TokenType};

#[derive(Debug, PartialEq, Clone)]
pub struct Lexer {
    /// Source code to lex
    src: String,

    /// Position relative to src start
    pos: usize,

    /// Line number for error reporting
    line: usize,

    /// Position relative to line start
    col: usize,
}

impl Lexer {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            src: src.to_string(),
            pos: 0,
            line: 1,
            col: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.pos < self.src.len() {
            if self
                .src
                .chars()
                .nth(self.pos)
                .unwrap()
                .is_ascii_whitespace()
            {
                self.lex_whitespaces();
                continue;
            }

            let token = self.tokenize();
            tokens.push(token);
        }

        // Reset the lexer state for potential reuse
        self.pos = 0;
        self.line = 1;
        self.col = 0;
        
        tokens
    }

    fn tokenize(&mut self) -> Token {
        // Get the char at the current position and decide what to do based on it
        let c = &self.src.chars().nth(self.pos).unwrap();

        if c.is_ascii_digit() {
            return self.lex_num();
        } else if c.is_ascii_alphabetic() || c.to_string() == "_" {
            return self.lex_ident();
        } else if c.to_string() == "\"" || c.to_string() == "'" {
            return self.lex_str();
        } else {
            return self.lex_ops();
        }
    }

    fn lex_str(&mut self) -> Token {
        println!("Lexing string at position {}", self.pos);

        let quote = self.src.chars().nth(self.pos).unwrap();
        self.pos += 1; // Skip the opening quote

        let mut str = String::new();
        let start_pos = self.pos;

        while self.pos < self.src.len() {
            let c = self.src.chars().nth(self.pos).unwrap();
            println!("Lexing char '{}' at index {}", c, self.pos);
            if c == quote {
                break;
            }
            if c == '\\' {
                println!("Lexing escape sequence at index {}", self.pos);

                // Handle escape sequences
                self.pos += 1; // Skip the backslash
                if self.pos >= self.src.len() {
                    panic!("Unterminated string starting at position {}", start_pos);
                }

                let next_char = self.src.chars().nth(self.pos).unwrap();
                str.push(next_char)
            } else {
                str.push(c);
            }
            self.pos += 1;
        }

        if self.src.chars().nth(self.pos).unwrap() != quote {
            panic!("Unterminated string starting at position {}", start_pos);
        }

        // Consume the closing quote
        self.pos += 1;

        let token_size = self.pos - start_pos; // Includes the opening & closing quotes
        self.create_token(TokenType::Str(str.clone()), &str, start_pos, token_size)
    }

    fn lex_num(&mut self) -> Token {
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

        let token_size = num.len();
        self.create_token(
            if has_dot {
                TokenType::Num(num.parse::<f64>().unwrap())
            } else {
                TokenType::Num(num.parse::<i64>().unwrap() as f64)
            },
            &num,
            start_pos,
            token_size,
        )
    }

    fn lex_ops(&mut self) -> Token {
        println!(
            "Lexing operator '{}' at position {}",
            self.src.chars().nth(self.pos).unwrap(),
            self.pos
        );

        let c = self.src.chars().nth(self.pos).unwrap();
        match c {
            '(' => {
                self.pos += 1;
                return self.create_token(TokenType::LParen, "(", self.pos - 1, 1);
            }
            ')' => {
                self.pos += 1;
                return self.create_token(TokenType::RParen, ")", self.pos - 1, 1);
            }
            '[' => {
                self.pos += 1;
                return self.create_token(TokenType::LSqBrace, "[", self.pos - 1, 1);
            }
            ']' => {
                self.pos += 1;
                return self.create_token(TokenType::RSqBrace, "]", self.pos - 1, 1);
            }
            '{' => {
                self.pos += 1;
                return self.create_token(TokenType::LBrace, "{", self.pos - 1, 1);
            }
            '}' => {
                self.pos += 1;
                return self.create_token(TokenType::RBrace, "}", self.pos - 1, 1);
            }
            ',' => {
                self.pos += 1;
                return self.create_token(TokenType::Comma, ",", self.pos - 1, 1);
            }
            ':' => {
                self.pos += 1;
                return self.create_token(TokenType::Colon, ":", self.pos - 1, 1);
            }
            ';' => {
                self.pos += 1;
                return self.create_token(TokenType::Semicolon, ";", self.pos - 1, 1);
            }
            '.' => {
                if self.src.chars().nth(self.pos + 1).unwrap() == '.' {
                    self.pos += 2;
                    return self.create_token(TokenType::DotDot, "..", self.pos - 2, 2);
                } else {
                    self.pos += 1;
                    return self.create_token(TokenType::Dot, ".", self.pos - 1, 1);
                }
            }
            '+' => {
                if self.src.chars().nth(self.pos + 1).unwrap() == '=' {
                    self.pos += 2;
                    return self.create_token(TokenType::PlusAssign, "+=", self.pos - 2, 2);
                } else {
                    self.pos += 1;
                    return self.create_token(TokenType::Plus, "+", self.pos - 1, 1);
                }
            }
            '-' => {
                if self.src.chars().nth(self.pos + 1).unwrap() == '=' {
                    self.pos += 2;
                    return self.create_token(TokenType::MinusAssign, "-=", self.pos - 2, 2);
                } else {
                    self.pos += 1;
                    return self.create_token(TokenType::Minus, "-", self.pos - 1, 1);
                }
            }
            '*' => {
                if self.src.chars().nth(self.pos + 1).unwrap() == '=' {
                    self.pos += 2;
                    return self.create_token(TokenType::StarAssign, "*=", self.pos - 2, 2);
                } else {
                    self.pos += 1;
                    return self.create_token(TokenType::Star, "*", self.pos - 1, 1);
                }
            }
            '/' => {
                if self.src.chars().nth(self.pos + 1).unwrap() == '=' {
                    self.pos += 2;
                    return self.create_token(TokenType::SlashAssign, "/=", self.pos - 2, 2);
                } else {
                    self.pos += 1;
                    return self.create_token(TokenType::Slash, "/", self.pos - 1, 1);
                }
            }
            '>' => {
                if self.src.chars().nth(self.pos + 1).unwrap() == '=' {
                    self.pos += 2;
                    return self.create_token(TokenType::GtOrEq, ">=", self.pos - 2, 2);
                } else {
                    self.pos += 1;
                    return self.create_token(TokenType::Gt, ">", self.pos - 1, 1);
                }
            }
            '<' => {
                if self.src.chars().nth(self.pos + 1).unwrap() == '=' {
                    self.pos += 2;
                    return self.create_token(TokenType::LtOrEq, "<=", self.pos - 2, 2);
                } else {
                    self.pos += 1;
                    return self.create_token(TokenType::Lt, "<", self.pos - 1, 1);
                }
            }
            '=' => {
                if self.src.chars().nth(self.pos + 1).unwrap() == '=' {
                    self.pos += 2;
                    return self.create_token(TokenType::Eq, "==", self.pos - 2, 2);
                } else {
                    self.pos += 1;
                    return self.create_token(TokenType::Assign, "=", self.pos - 1, 1);
                }
            }
            '!' => {
                if self.src.chars().nth(self.pos + 1).unwrap() == '=' {
                    self.pos += 2;
                    return self.create_token(TokenType::BangEqual, "!=", self.pos - 2, 2);
                } else {
                    self.pos += 1;
                    return self.create_token(TokenType::Bang, "!", self.pos - 1, 1);
                }
            }
            '&' => {
                if self.src.chars().nth(self.pos + 1).unwrap() == '&' {
                    self.pos += 2;
                    return self.create_token(TokenType::AmpAmp, "&&", self.pos - 2, 2);
                } else {
                    self.pos += 1;
                    return self.create_token(TokenType::Amp, "&", self.pos - 1, 1);
                }
            }
            '|' => {
                if self.src.chars().nth(self.pos + 1).unwrap() == '|' {
                    self.pos += 2;
                    return self.create_token(TokenType::PipePipe, "||", self.pos - 2, 2);
                } else {
                    self.pos += 1;
                    return self.create_token(TokenType::Pipe, "|", self.pos - 1, 1);
                }
            }
            _ => {
                // If we reach here, it means we have an unknown character. We can either panic or skip it. For now, let's panic.
                panic!("Unknown character '{}' at position {}", c, self.pos)
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

            println!(
                "Whitespace at position {} `{}`",
                self.pos,
                self.src.chars().nth(self.pos).unwrap()
            );

            self.pos += 1; // Skip whitespaces
        }
    }

    fn lex_ident(&mut self) -> Token {
        panic!(
            "Unknown character at position {} while parsing IDENTIFIERS",
            self.pos
        )
    }

    fn create_token(&mut self, ttype: TokenType, lexeme: &str, pos: usize, size: usize) -> Token {
        Token {
            ttype,
            lexeme: lexeme.to_string(),
            pos,
            size,
            line: self.line,
        }
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
    fn lex_string_double() {
        let inp = "\"Hello, World!\"";
        let mut lexer = Lexer::new(inp);
        let tokens = lexer.lex();
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().nth(0).unwrap().ttype,
            TokenType::Str("Hello, World!".to_string())
        );
    }

    #[test]
    fn lex_string_single() {
        let inp = "'Hello, World!'";
        let mut lexer = Lexer::new(inp);
        let tokens = lexer.lex();
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().nth(0).unwrap().ttype,
            TokenType::Str("Hello, World!".to_string())
        );
    }

    #[test]
    fn lex_string_escaped() {
        let inp = "\"Hello, \nWorld!\"";
        let mut lexer = Lexer::new(inp);
        let tokens = lexer.lex();
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().nth(0).unwrap().ttype,
            TokenType::Str("Hello, \nWorld!".to_string())
        );
    }

    #[test]
    fn lex_string_escaped_str() {
        let inp = "\"Hello, 'World!\"";
        let mut lexer = Lexer::new(inp);
        let tokens = lexer.lex();
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().nth(0).unwrap().ttype,
            TokenType::Str("Hello, 'World!".to_string())
        );
    }

    #[test]
    fn lex_string_escaped_str_double() {
        let inp = "\"Hello, \\\"World!\""; // Escaped double quote
        let mut lexer = Lexer::new(inp);
        let tokens = lexer.lex();
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().nth(0).unwrap().ttype,
            TokenType::Str("Hello, \"World!".to_string())
        );
    }

    #[test]
    fn lex_string_escaped_str_single() {
        let inp = "'Hello, \\\'World!'";
        let mut lexer = Lexer::new(inp);
        let tokens = lexer.lex();
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().nth(0).unwrap().ttype,
            TokenType::Str("Hello, 'World!".to_string())
        );
    }

    #[test]
    fn lex_expr() {
        let inp = "34 + 5 * (2 - 8)";
        let mut lexer = Lexer::new(inp);
        let tokens = lexer.lex();
        println!("Tokens: {:?}", tokens);
        assert_eq!(tokens.len(), 9);
    }
}
