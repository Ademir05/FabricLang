use crate::compiler::{
    ast::Stmt,
    lexer::{Token, TokenData},
};

pub struct Parser {
    tokens: Vec<TokenData>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenData>) -> Self {
        Self { tokens, current: 0 }
    }

    // fn parse() -> Vec<Stmt> {
    //     let mut stmts: Vec<Stmt> = Vec::new();
    //     while let Some(token) = self.peek() {
    //         match token.kind {
    //             Token::IntType | Token::FloatType | Token::StringType | Token::BoolType | Token::CharType | Token::BigIntType | Token::DoubleType => {

    //         }
    //     }
    // }

    fn advance(&mut self) -> Option<&TokenData> {
        if self.current < self.tokens.len() {
            let token = &self.tokens[self.current];
            self.current += 1;
            Some(token)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&TokenData> {
        if self.current < self.tokens.len() {
            Some(&self.tokens[self.current])
        } else {
            None
        }
    }
}