use std::collections::HashMap;

pub struct Interpreter {
    // Un mapa para guardar variables: "x" -> 10, "i" -> 0
    environment: HashMap<String, i32>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { environment: HashMap::new() }
    }
    
    // El método principal que "visita" y ejecuta cada sentencia
    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for stmt in statements {
            self.execute(stmt); 
        }
    }
    fn execute(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::VarDeclaration { name, initializer, .. } => {
                let value = self.evaluate(initializer.unwrap());
                self.environment.insert(name, value);
            }
            Stmt::Assignment { name, value } => {
                let new_val = self.evaluate(value);
                self.environment.insert(name, new_val);
            }
            Stmt::Return(val) => {
                // Aquí manejaríamos el retorno de funciones
            }
            // ... (resto de las sentencias)
            _ => {}
        }
    }

    // Esta función resuelve expresiones (5 + 10 = 15)
    fn evaluate(&mut self, expr: Expr) -> i32 {
        match expr {
            Expr::Literal(Token::Integer(val)) => val,
            Expr::Variable(name) => *self.environment.get(&name).unwrap(),
            Expr::Binary { left, operator, right } => {
                let l = self.evaluate(*left);
                let r = self.evaluate(*right);
                match operator {
                    Token::Plus => l + r,
                    Token::Minus => l - r,
                    Token::Multiply => l * r,
                    Token::Divide => l / r,
                    _ => panic!("Operador no soportado"),
                }
            }
            _ => 0,
        }
    }
}