use crate::token::{Token, TokenType, Op};

pub enum Expr {
    StrLiteral(String),
    NumLiteral(f64),
    /// Unary operator with operator and expression
    Unary(Op, Box<Expr>),
    /// Binary operator with operator, left and right expressions
    Binary(Op, Box<Expr>, Box<Expr>),
}

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) {
        self.parse_assignment();
    }

    fn parse_assignment(&mut self) -> Expr {
        let left = self.parse_expression();
        if self.match_token(TokenType::Assign) {
            let op = Op::Assign;
            let right = self.parse_assignment();
            return Expr::Binary(op, Box::new(left), Box::new(right));
        }
        left
    }

    fn parse_unary(&mut self) -> Expr {
        if self.match_token(TokenType::Bang) {
            let op = Op::Bang;
            let expr = self.parse_unary();
            return Expr::Unary(op, Box::new(expr));
        }

        if self.match_token(TokenType::Minus) {
            let op = Op::Minus;
            let expr = self.parse_unary();
            return Expr::Unary(op, Box::new(expr));
        }

        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Expr {
        // For simplicity, we only handle literals here
        if let Some(token) = self.tokens.get(self.pos) {
            match &token.ttype {
                TokenType::Num(num) => {
                    self.pos += 1;
                    Expr::NumLiteral(*num)
                }
                TokenType::Str(s) => {
                    self.pos += 1;
                    Expr::StrLiteral(s.clone())
                }
                _ => panic!("Unexpected token: {:?}", token),
            }
        } else {
            panic!("Unexpected end of input");
        }
    }

    fn match_token(&mut self, expected: TokenType) -> bool {
        if let Some(token) = self.tokens.get(self.pos) {
            if token.ttype == expected {
                self.pos += 1;
                return true;
            }
        }
        false
    }
}