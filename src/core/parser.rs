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

            Token::LeftBrace => Stmt::Block(self.parse_block()),

            Token::Identifier(_) => self.parse_assignment_or_expression(),

            Token::Return => self.parse_return_statement(),

            Token::While => self.parse_while_statement(),

            _ => panic!("Sentencia no reconocida: {:?}", t.kind),
        }
    }

    fn parse_return_statement(&mut self) -> Stmt {
        self.advance(); // Consume 'return'

        let mut value = None;
        // Si el siguiente token no es ';', significa que hay una expresión de retorno
        if !self.check(Token::Semi) {
            value = Some(self.parse_expression());
        }

        self.consume(Token::Semi, "Se esperaba ';' después del valor de retorno");
        Stmt::Return(value)
    }

    fn parse_function_declaration(&mut self) -> Stmt {
        // 1. Obtener tipo de retorno (int, string, void, etc.)
        let return_type = self
            .advance()
            .expect("Se esperaba tipo de retorno")
            .kind
            .clone();

        // 2. Palabra reservada 'function'
        self.consume(
            Token::Function,
            "Se esperaba la palabra reservada 'function'",
        );

        // 3. Nombre de la función
        let name = if let Token::Identifier(n) = self.peek().unwrap().kind.clone() {
            self.advance();
            n
        } else {
            panic!("Se esperaba nombre de función");
        };

        self.consume(Token::LeftParen, "Se esperaba '('");

        // 4. Parámetros tipados: (int a, string b)
        let mut params = Vec::new();
        if !self.check(Token::RightParen) {
            loop {
                let p_type = self
                    .advance()
                    .expect("Se esperaba tipo de parámetro")
                    .kind
                    .clone();
                if let Token::Identifier(p_name) = self.advance().unwrap().kind.clone() {
                    params.push((p_type, p_name));
                } else {
                    panic!("Se esperaba nombre de parámetro");
                }

                if !self.check(Token::Comma) {
                    break;
                }
                self.advance(); // consume ','
            }
        }

        self.consume(Token::RightParen, "Se esperaba ')'");

        // 5. Cuerpo de la función
        let body = self.parse_block();

        Stmt::Function {
            return_type,
            name,
            params,
            body,
        }
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        self.consume(Token::LeftBrace, "Se esperaba '{' para iniciar el bloque");

        let mut statements = Vec::new();

        // Recolectamos sentencias hasta que encontremos el cierre '}'
        while !self.check(Token::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement());
        }

        self.consume(Token::RightBrace, "Se esperaba '}' para cerrar el bloque");
        statements
    }

    fn parse_assignment_or_expression(&mut self) -> Stmt {
        // Aquí usamos un truco: leemos la expresión primero
        let expr = self.parse_expression();

        // Si después de la expresión hay un '=', era una asignación
        // Nota: Esto requiere que tu AST soporte Expr::Assignment o manejarlo como Stmt
        if self.check(Token::Assign) {
            self.advance(); // consume '='
            let value = self.parse_expression();
            self.consume(Token::Semi, "Se esperaba ';' después de la asignación");

            // Convertimos el identificador inicial en un nodo de asignación
            if let Expr::Variable(name) = expr {
                return Stmt::Assignment { name, value };
            } else {
                panic!("Solo se puede asignar valores a variables");
            }
        }

        self.consume(Token::Semi, "Se esperaba ';' después de la expresión");
        Stmt::Expression(expr)
    }

    fn parse_if_statement(&mut self) -> Stmt {
        self.advance(); // consume 'if'
        self.consume(Token::LeftParen, "Se esperaba '(' después de 'if'");
        let condition = self.parse_expression();
        self.consume(Token::RightParen, "Se esperaba ')' después de la condición");

        let then_branch = self.parse_block();

        let mut else_branch = None;
        if self.check(Token::Else) {
            self.advance(); // consume 'else'
            // Si hay otro 'if' después del 'else', podemos parsearlo recursivamente
            if self.check(Token::If) {
                else_branch = Some(vec![self.parse_if_statement()]);
            } else {
                else_branch = Some(self.parse_block());
            }
        }

        Stmt::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    fn parse_while_statement(&mut self) -> Stmt {
        self.advance(); // consume 'while'
        self.consume(Token::LeftParen, "Se esperaba '(' después de 'while'");
        let condition = self.parse_expression();
        self.consume(Token::RightParen, "Se esperaba ')' después de la condición");

        let body = self.parse_block();

        Stmt::While { condition, body }
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

    // fn parse_expression(&mut self) -> Expr {
    //     self.additive()
    // }
    fn parse_expression(&mut self) -> Expr {
        self.comparison()
    }
    fn comparison(&mut self) -> Expr {
        let mut expr = self.additive(); // Primero resolvemos sumas/restas

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
                let right = self.additive(); // Comparamos con otra expresión aritmética
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
        let token = self.peek().expect("Se esperaba un token");

        let token_kind = self.peek().unwrap().kind.clone();
        match token_kind {
            // --- Manejo de Paréntesis ---
            Token::LeftParen => {
                self.advance(); // consume '('
                let expr = self.parse_expression(); // Vuelve al inicio de la jerarquía
                self.consume(Token::RightParen, "Se esperaba ')' después de la expresión");
                expr
            }

            // --- Literales y Variables ---
            Token::Integer(_) | Token::FloatLiteral(_) | Token::StringLiteral(_) => {
                let t = self.advance().unwrap();
                Expr::Literal(t.kind.clone())
            }
            Token::Identifier(name) => {
                self.advance(); // Consumimos el nombre

                // ¿Viene un paréntesis después?
                if self.check(Token::LeftParen) {
                    self.advance(); // Consumimos '('
                    let mut arguments = Vec::new();

                    if !self.check(Token::RightParen) {
                        loop {
                            arguments.push(self.parse_expression());
                            if !self.check(Token::Comma) {
                                break;
                            }
                            self.advance(); // Consume ','
                        }
                    }
                    self.consume(Token::RightParen, "Se esperaba ')'");

                    // Retornamos un nuevo nodo de expresión: Llamada
                    Expr::Call {
                        callee: name.clone(),
                        arguments,
                    }
                } else {
                    // Si no hay paréntesis, es solo una variable normal
                    Expr::Variable(name.clone())
                }
            }
            _ => panic!("Se esperaba una expresión en la línea {}", token.line),
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

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if let Some(prev) = self.tokens.get(self.current - 1) {
                if prev.kind == Token::Semi {
                    return;
                }
            }

            match self.peek().unwrap().kind {
                Token::Function | Token::IntType | Token::If | Token::While | Token::Return => {
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }
}
