use crate::ast::{Program, Statement, Expr};
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn next(&mut self) -> Token {
        let t = self.peek().clone();
        self.pos += 1;
        t
    }

    fn expect(&mut self, expected: &Token) {
        let t = self.next();
        if &t != expected {
            panic!("Expected {:?}, got {:?}", expected, t);
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();
        while *self.peek() != Token::Eof {
            statements.push(self.parse_statement());
        }
        Program { statements }
    }

    fn parse_statement(&mut self) -> Statement {
        match self.peek() {
            Token::Print => {
                self.next();
                let expr = self.parse_expr();
                self.expect(&Token::Semicolon);
                Statement::Print { expr }
            }
            Token::Ident(_) => {
                let name = if let Token::Ident(id) = self.next() {
                    id
                } else {
                    unreachable!();
                };
                self.expect(&Token::Assign);
                let expr = self.parse_expr();
                self.expect(&Token::Semicolon);
                Statement::Assignment { name, expr }
            }
            other => panic!("Unexpected token in statement: {:?}", other),
        }
    }

    fn parse_expr(&mut self) -> Expr {
        let mut left = self.parse_term();
        loop {
            match self.peek() {
                Token::Plus => {
                    self.next();
                    let right = self.parse_term();
                    left = Expr::Add(Box::new(left), Box::new(right));
                }
                Token::Minus => {
                    self.next();
                    let right = self.parse_term();
                    left = Expr::Sub(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    fn parse_term(&mut self) -> Expr {
        match self.next() {
            Token::Number(n) => Expr::Number(n),
            Token::Ident(id) => Expr::Var(id),
            t => panic!("Unexpected token in term: {:?}", t),
        }
    }
}
