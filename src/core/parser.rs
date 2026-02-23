use crate::core::ast::{Expr, Stmt};
use crate::core::token::{Token, TokenData};

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl ParseError {
    pub fn new(message: &str, line: usize, column: usize) -> Self {
        Self {
            message: message.to_string(),
            line,
            column,
        }
    }
}

pub struct Parser {
    tokens: Vec<TokenData>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenData>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        let t = self
            .peek()
            .ok_or(ParseError::new("Unexpected end of input", 0, 0))?;

        match &t.kind {
            Token::IntType
            | Token::FloatType
            | Token::StringType
            | Token::BoolType
            | Token::CharType
            | Token::BigIntType
            | Token::DoubleType
            | Token::VoidType => {
                if self
                    .tokens
                    .get(self.current + 1)
                    .map_or(false, |t| t.kind == Token::Function)
                {
                    self.parse_function_declaration()
                } else {
                    self.parse_var_declaration()
                }
            }
            Token::If => self.parse_if_statement(),
            Token::LeftBrace => Ok(Stmt::Block(self.parse_block()?)),
            Token::Identifier(_) => self.parse_assignment_or_expression(),
            Token::Return => self.parse_return_statement(),
            Token::While => self.parse_while_statement(),
            Token::Print => self.parse_print_statement(),
            Token::Println => self.parse_println_statement(),
            Token::Switch => self.parse_switch_statement(),
            _ => Err(ParseError::new(
                &format!("Sentencia no reconocida: {:?}", t.kind),
                t.line,
                t.col,
            )),
        }
    }

    fn parse_print_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        self.consume(Token::LeftParen, "Se esperaba '(' después de 'if'")?;
        let value = self.parse_expression()?;
        self.consume(Token::RightParen, "Se esperaba ')' después de la condición")?;
        self.consume(Token::Semi, "Se esperaba ';' después de la expresión")?;
        Ok(Stmt::Print { value })
    }

    fn parse_println_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        self.consume(Token::LeftParen, "Se esperaba '(' después de 'if'")?;
        let value = self.parse_expression()?;
        self.consume(Token::RightParen, "Se esperaba ')' después de la condición")?;
        self.consume(Token::Semi, "Se esperaba ';' después de la expresión")?;
        Ok(Stmt::Println { value })
    }

    fn parse_return_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // Consume 'return'

        let mut value = None;
        // Si el siguiente token no es ';', significa que hay una expresión de retorno
        if !self.check(Token::Semi) {
            value = Some(self.parse_expression()?);
        }

        self.consume(Token::Semi, "Se esperaba ';' después del valor de retorno")?;
        Ok(Stmt::Return(value))
    }

    fn parse_function_declaration(&mut self) -> Result<Stmt, ParseError> {
        let return_type = self
            .advance()
            .expect("Se esperaba tipo de retorno")
            .kind
            .clone();
        self.consume(
            Token::Function,
            "Se esperaba la palabra reservada 'function'",
        )?;
        let name = match self.peek().map(|t| t.kind.clone()) {
            Some(Token::Identifier(n)) => {
                self.advance();
                n
            }
            Some(t) => {
                return Err(ParseError::new(
                    &format!("Se esperaba nombre de función, se obtuvo {:?}", t),
                    0,
                    0, // Note: In a real scenario, you'd pass line/col from TokenData
                ));
            }
            None => return Err(ParseError::new("Se esperaba nombre de función", 0, 0)),
        };
        // let name = if let Token::Identifier(n) = self.peek().unwrap().kind.clone() {
        //     self.advance();
        //     n
        // } else {
        //     panic!("Se esperaba nombre de función");
        // };

        self.consume(Token::LeftParen, "Se esperaba '('")?;
        let mut params = Vec::new();
        if !self.check(Token::RightParen) {
            loop {
                let p_type = self
                    .advance()
                    .expect("Se esperaba tipo de parámetro")
                    .kind
                    .clone();
                match self.advance() {
                    Some(TokenData {
                        kind: Token::Identifier(p_name),
                        line: _,
                        col: _,
                    }) => {
                        params.push((p_type, p_name.clone()));
                    }
                    Some(t) => {
                        return Err(ParseError::new(
                            "Se esperaba nombre de parámetro",
                            t.line,
                            t.col,
                        ));
                    }
                    None => return Err(ParseError::new("Se esperaba nombre de parámetro", 0, 0)),
                }

                // if let Token::Identifier(p_name) = self.advance().unwrap().kind.clone() {
                //     params.push((p_type, p_name));
                // } else {
                //     panic!("Se esperaba nombre de parámetro");
                // }

                if !self.check(Token::Comma) {
                    break;
                }
                self.advance();
            }
        }
        self.consume(Token::RightParen, "Se esperaba ')'")?;
        let body = self.parse_block()?;
        Ok(Stmt::Function {
            return_type,
            name,
            params,
            body,
        })
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        self.consume(Token::LeftBrace, "Se esperaba '{' para iniciar el bloque")?;

        let mut statements = Vec::new();

        // Recolectamos sentencias hasta que encontremos el cierre '}'
        while !self.check(Token::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.consume(Token::RightBrace, "Se esperaba '}' para cerrar el bloque")?;
        Ok(statements)
    }

    fn parse_assignment_or_expression(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.parse_expression()?;

        if self.check(Token::Assign) {
            self.advance(); // consume '='
            let value = self.parse_expression()?;
            self.consume(Token::Semi, "Se esperaba ';' después de la asignación")?;

            if let Expr::Variable(name) = expr {
                return Ok(Stmt::Assignment { name, value });
            } else {
                return Err(ParseError::new(
                    "Solo se puede asignar valores a variables",
                    0,
                    0,
                ));
            }
            // if let Expr::Variable(name) = expr {
            //     return Ok(Stmt::Assignment { name, value });
            // } else {
            //     panic!("Solo se puede asignar valores a variables");
            // }
        }

        self.consume(Token::Semi, "Se esperaba ';' después de la expresión")?;
        Ok(Stmt::Expression(expr))
    }

    fn parse_if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // consume 'if'
        self.consume(Token::LeftParen, "Se esperaba '(' después de 'if'")?;
        let condition = self.parse_expression()?;
        self.consume(Token::RightParen, "Se esperaba ')' después de la condición")?;

        let then_branch = self.parse_block()?;

        let mut else_branch = None;
        if self.check(Token::Else) {
            self.advance(); // consume 'else'
            // Si hay otro 'if' después del 'else', podemos parsearlo recursivamente
            if self.check(Token::If) {
                else_branch = Some(vec![self.parse_if_statement()?]);
            } else {
                else_branch = Some(self.parse_block()?);
            }
        }

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // consume 'while'
        self.consume(Token::LeftParen, "Se esperaba '(' después de 'while'")?;
        let condition = self.parse_expression()?;
        self.consume(Token::RightParen, "Se esperaba ')' después de la condición")?;

        let body = self.parse_block()?;

        Ok(Stmt::While { condition, body })
    }

    fn parse_var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let ty = self
            .advance()
            .ok_or(ParseError::new("Se esperaba un tipo", 0, 0))?
            .kind
            .clone();

        let name = if let Token::Identifier(ref n) = self.peek().unwrap().kind {
            let name_string = n.clone();
            self.advance();
            name_string
        } else {
            let t = self.peek().unwrap();
            return Err(ParseError::new(
                "Se esperaba un nombre de variable",
                t.line,
                t.col,
            ));
        };

        let mut initializer = None;
        if self.check(Token::Assign) {
            self.advance();
            initializer = Some(self.parse_expression()?);
        }

        self.consume(Token::Semi, "Se esperaba ';' al final de la declaración")?;
        Ok(Stmt::VarDeclaration {
            ty,
            name,
            initializer,
        })
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.comparison()
    }
    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.additive()?;

        while let Some(t) = self.peek() {
            if matches!(
                t.kind,
                Token::Greater
                    | Token::Less
                    | Token::GreaterEqual
                    | Token::LessEqual
                    | Token::EqualEqual
                    | Token::NotEqual
            ) {
                let operator = self.advance().unwrap().kind.clone();
                let right = self.additive()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn additive(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.multiplicative()?;

        while let Some(t) = self.peek() {
            if matches!(t.kind, Token::Plus | Token::Minus) {
                let operator = self.advance().unwrap().kind.clone();
                let right = self.multiplicative()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn multiplicative(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while let Some(t) = self.peek() {
            if matches!(t.kind, Token::Multiply | Token::Divide) {
                let operator = self.advance().unwrap().kind.clone();
                let right = self.unary()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }
    // fn multiplicative(&mut self) -> Result<Expr, ParseError> {
    //     let mut expr = self.primary()?;

    //     while let Some(t) = self.peek() {
    //         if matches!(t.kind, Token::Multiply | Token::Divide) {
    //             let operator = self.advance().unwrap().kind.clone();
    //             let right = self.primary()?;
    //             expr = Expr::Binary {
    //                 left: Box::new(expr),
    //                 operator,
    //                 right: Box::new(right),
    //             };
    //         } else {
    //             break;
    //         }
    //     }
    //     Ok(expr)
    // }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if let Some(t) = self.peek() {
            match t.kind {
                Token::Minus | Token::Not => {
                    // '-' o '!'
                    let operator = self.advance().unwrap().kind.clone();
                    let right = self.unary()?; // Recursivo para soportar --x o !!true
                    return Ok(Expr::Unary {
                        operator,
                        right: Box::new(right),
                    });
                }
                _ => {}
            }
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.peek().expect("Se esperaba un token");

        let token_kind = self.peek().unwrap().kind.clone();
        match token_kind {
            // --- Manejo de Paréntesis ---
            Token::LeftParen => {
                self.advance(); // consume '('
                let expr = self.parse_expression()?; // Vuelve al inicio de la jerarquía
                self.consume(Token::RightParen, "Se esperaba ')' después de la expresión")?;
                Ok(expr)
            }

            // --- Literales y Variables ---
            Token::IntegerLiteral(_) | Token::FloatLiteral(_) | Token::StringLiteral(_) => {
                let t = self.advance().unwrap();
                Ok(Expr::Literal(t.kind.clone()))
            }

            Token::Identifier(name) => {
                self.advance(); // Consumimos el nombre

                // ¿Viene un paréntesis después?
                if self.check(Token::LeftParen) {
                    self.advance(); // Consumimos '('
                    let mut arguments = Vec::new();

                    if !self.check(Token::RightParen) {
                        loop {
                            arguments.push(self.parse_expression()?);
                            if !self.check(Token::Comma) {
                                break;
                            }
                            self.advance(); // Consume ','
                        }
                    }
                    self.consume(Token::RightParen, "Se esperaba ')'")?;

                    // Retornamos un nuevo nodo de expresión: Llamada
                    Ok(Expr::Call {
                        callee: name.clone(),
                        arguments,
                    })
                } else {
                    // Si no hay paréntesis, es solo una variable normal
                    Ok(Expr::Variable(name.clone()))
                }
            }
            Token::Input => {
                self.advance();
                self.consume(Token::LeftParen, "Se esperaba '('")?;
                self.consume(Token::RightParen, "Se esperaba ')'")?;
                Ok(Expr::Input)
            }
            _ => {
                return Err(ParseError::new(
                    "Se esperaba una expresión",
                    token.line,
                    token.col,
                ));
            } // _ => panic!("Se esperaba una expresión en la línea {}", token.line),
        }
    }

    fn check(&self, kind: Token) -> bool {
        self.peek().map_or(false, |t| {
            std::mem::discriminant(&t.kind) == std::mem::discriminant(&kind)
        })
    }

    fn consume(&mut self, kind: Token, msg: &str) -> Result<&TokenData, ParseError> {
        if self.check(kind) {
            Ok(self.advance().unwrap())
        } else {
            let t = self
                .peek()
                .ok_or(ParseError::new("Unexpected end of input", 0, 0))?;
            Err(ParseError::new(msg, t.line, t.col))
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

    fn parse_switch_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // consume 'switch'
        self.consume(Token::LeftParen, "Se esperaba '('")?;
        let condition = self.parse_expression()?;
        self.consume(Token::RightParen, "Se esperaba ')'")?;
        self.consume(Token::LeftBrace, "Se esperaba '{'")?;

        let mut cases = Vec::new();
        let mut default_case = None;

        while !self.check(Token::RightBrace) && !self.is_at_end() {
            if self.check(Token::Case) {
                self.advance(); // consume 'case'
                let case_val = self.parse_expression()?;
                self.consume(Token::Colon, "Se esperaba ':'")?;
                let body = self.parse_block()?;
                cases.push((case_val, body));
            } else if self.check(Token::Default) {
                self.advance(); // consume 'default'
                self.consume(Token::Colon, "Se esperaba ':'")?;
                default_case = Some(self.parse_block()?);
            } else {
                let t = self.peek().unwrap(); // O manejar Option si quieres seguridad total
                return Err(ParseError::new(
                    "Se esperaba 'case' o 'default' dentro del switch",
                    t.line,
                    t.col,
                ));
            }
        }

        self.consume(Token::RightBrace, "Se esperaba '}'")?;
        Ok(Stmt::Switch {
            condition,
            cases,
            default: default_case,
        })
    }
}
