use crate::core::ast::{Expr, Stmt};
use crate::core::token::{Token, TokenData};

pub struct Parser {
    tokens: Vec<TokenData>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenData>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.parse_statement());
        }
        statements
    }

    fn parse_statement(&mut self) -> Stmt {
        let t = self.peek().expect("Unexpected end of input");
        match t.kind {
            Token::IntType
            | Token::FloatType
            | Token::StringType
            | Token::BoolType
            | Token::CharType
            | Token::BigIntType
            | Token::DoubleType => self.parse_var_declaration(),
            _ => panic!("Sentencia no reconocida: {:?}", t.kind),
        }
    }

    fn parse_var_declaration(&mut self) -> Stmt {
        // 1. Obtener el tipo (ya sabemos que es uno de los tipos por el match anterior)
        let ty = self.advance().unwrap().kind.clone();

        // 2. Obtener el nombre (Identifier)
        let name = if let Token::Identifier(ref n) = self.peek().unwrap().kind {
            let name_string = n.clone();
            self.advance();
            name_string
        } else {
            panic!("Se esperaba un nombre de variable");
        };

        // 3. ¿Tiene valor inicial? (Opcional)
        let mut initializer = None;
        if self.check(Token::Assign) {
            self.advance(); // consume '='
            initializer = Some(self.parse_expression()); // Aquí llamarías a la lógica de expresiones
        }

        // 4. Finalizar con ';'
        self.consume(Token::Semi, "Se esperaba ';' al final de la declaración");

        Stmt::VarDeclaration {
            ty,
            name,
            initializer,
        }
    }

    fn parse_expression(&mut self) -> Expr {
        self.additive()
    }

    fn additive(&mut self) -> Expr {
        let mut expr = self.multiplicative();

        while let Some(t) = self.peek() {
            if matches!(t.kind, Token::Plus | Token::Minus) {
                let operator = self.advance().unwrap().kind.clone();
                let right = self.multiplicative();
                expr = Expr::Binary {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        expr
    }

    fn multiplicative(&mut self) -> Expr {
        let mut expr = self.primary();

        while let Some(t) = self.peek() {
            if matches!(t.kind, Token::Multiply | Token::Divide) {
                let operator = self.advance().unwrap().kind.clone();
                let right = self.primary();
                expr = Expr::Binary {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        expr
    }

    fn primary(&mut self) -> Expr {
        let token = self.advance().expect("Se esperaba un valor");
        match &token.kind {
            Token::Integer(_) | Token::FloatLiteral(_) | Token::StringLiteral(_) => {
                Expr::Literal(token.kind.clone())
            }
            Token::Identifier(name) => Expr::Variable(name.clone()),
            _ => panic!("Se esperaba un valor o variable en la línea {}", token.line),
        }
    }

    fn check(&self, kind: Token) -> bool {
        self.peek().map_or(false, |t| {
            std::mem::discriminant(&t.kind) == std::mem::discriminant(&kind)
        })
    }

    fn consume(&mut self, kind: Token, msg: &str) -> &TokenData {
        if self.check(kind) {
            self.advance().unwrap()
        } else {
            panic!("Error: {} en la línea {}", msg, self.peek().unwrap().line);
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().map_or(true, |t| t.kind == Token::EOF)
    }

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
