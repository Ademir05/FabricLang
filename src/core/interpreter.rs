use std::collections::HashMap;

use crate::{
    core::{
        ast::{Expr, Stmt},
        token::Token,
    },
    persistence::models::SyntaxConfig,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f32),
    BigInt(i64),
    Double(f64),
    String(String),
    Char(char),
    Bool(bool),
    Void,
}

#[derive(Clone)]
pub struct Variable {
    pub var_type: Token,
    pub value: Value,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::BigInt(v) => write!(f, "{}", v),
            Value::Double(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "{}", v),
            Value::Char(v) => write!(f, "{}", v),
            Value::Bool(v) => write!(f, "{}", v),
            Value::Void => write!(f, "void"),
        }
    }
}

pub struct Environment {
    values: HashMap<String, Variable>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn get(&self, name: &str) -> Option<Variable> {
        match self.values.get(name) {
            Some(val) => Some(val.clone()),
            None => {
                if let Some(ref parent) = self.parent {
                    parent.get(name)
                } else {
                    None
                }
            }
        }
    }
    pub fn insert(&mut self, name: String, value: Variable) {
        self.values.insert(name, value);
    }
}

pub struct Interpreter {
    environment: Environment,
    functions: HashMap<String, (Vec<(Token, String)>, Vec<Stmt>)>,
    pub config: SyntaxConfig,
}

impl Interpreter {
    pub fn new(config: SyntaxConfig) -> Self {
        Self {
            environment: Environment {
                values: HashMap::new(),
                parent: None,
            },
            functions: HashMap::new(),
            config: config,
        }
    }

    // pub fn interpret(&mut self, statements: Vec<Stmt>) {
    //     for stmt in statements {
    //         let _ = self.execute(stmt);
    //     }
    // }
    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for stmt in statements {
            if let Err(e) = self.execute(stmt) {
                println!("Error en ejecución: {}", e);
                continue;
            }
        }
    }

    fn update_variable(&mut self, name: &str, value: Variable) -> bool {
        let mut current = Some(&mut self.environment);
        while let Some(env) = current {
            if env.values.contains_key(name) {
                env.values.insert(name.to_string(), value);
                return true;
            }
            match env.parent {
                Some(ref mut next) => current = Some(next),
                None => break,
            }
        }
        false
    }

    fn execute(&mut self, stmt: Stmt) -> Result<Option<Variable>, String> {
        match stmt {
            Stmt::VarDeclaration {
                ty,
                name,
                initializer,
                ..
            } => {
                let value = self.evaluate(initializer.expect("Variable sin inicializador"))?;
                self.type_check(&ty, &value)?;
                self.environment.insert(
                    name,
                    Variable {
                        var_type: ty,
                        value,
                    },
                );
                Ok(None)
            }

            Stmt::Assignment { name, value } => {
                let new_val = self.evaluate(value)?;
                if let Some(var) = self.environment.get(&name) {
                    self.type_check(&var.var_type, &new_val)?;
                    self.update_variable(
                        &name,
                        Variable {
                            var_type: var.var_type,
                            value: new_val,
                        },
                    );
                    Ok(None)
                } else {
                    return Err(format!("Variable '{}' no definida", name));
                }
            }

            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_val = self.evaluate(condition)?;
                if self.is_truthy(condition_val) {
                    for stmt in then_branch {
                        let res = self.execute(stmt)?;
                        if res.is_some() {
                            return Ok(res);
                        }
                    }
                } else if let Some(branch) = else_branch {
                    for stmt in branch {
                        let res = self.execute(stmt)?;
                        if res.is_some() {
                            return Ok(res);
                        }
                    }
                }
                Ok(None)
            }
            Stmt::Print { value } => {
                println!("{}", self.evaluate_strings(value)?);
                Ok(None)
            }
            Stmt::Println { value } => {
                println!("{}", self.evaluate_strings(value)?);
                Ok(None)
            }
            Stmt::Expression(expr) => {
                self.evaluate(expr);
                Ok(None)
            }
            Stmt::Return(value) => {
                if let Some(val_expr) = value {
                    let val = self.evaluate(val_expr)?;
                    // Aquí creamos un Variable temporal para return
                    return Ok(Some(Variable {
                        var_type: Token::VoidType, // o deducir según tu lógica
                        value: val,
                    }));
                }
                Ok(Some(Variable {
                    var_type: Token::VoidType,
                    value: Value::Void,
                }))
            }
            // Stmt::Return(value) => {
            //     if let Some(val_expr) = value {
            //         let v = self.evaluate(val_expr);
            //         return Ok(Some(v));
            //     }
            //     Ok(Some(Value::Void))
            // }
            Stmt::Block(statements) => {
                let old_env = std::mem::replace(
                    &mut self.environment,
                    Environment {
                        values: HashMap::new(),
                        parent: None,
                    },
                );
                self.environment.parent = Some(Box::new(old_env));
                let mut block_result = Ok(None);
                for stmt in statements {
                    let result = self.execute(stmt);
                    if let Ok(Some(_)) = result {
                        block_result = result;
                        break;
                    }
                    if result.is_err() {
                        block_result = result;
                        break;
                    }
                }
                if let Some(parent_env) = self.environment.parent.take() {
                    self.environment = *parent_env;
                }

                block_result
            }

            Stmt::Function {
                return_type: _,
                name,
                params,
                body,
            } => {
                self.functions.insert(name, (params, body));
                Ok(None)
            }
            Stmt::While { condition, body } => {
                while {
                    let cond_val = self.evaluate(condition.clone())?;
                    self.is_truthy(cond_val)
                } {
                    for stmt in body.clone() {
                        match self.execute(stmt) {
                            Ok(Some(v)) => return Ok(Some(v)),
                            Ok(None) => {}
                            Err(e) => {
                                println!("Error en ejecución: {}", e);
                                break; // rompe el ciclo actual y vuelve al menú
                            }
                        }
                    }
                }
                Ok(None)
            }
            // Stmt::While { condition, body } => {
            //     while {
            //         let cond_val = self.evaluate(condition.clone())?;
            //         self.is_truthy(cond_val)
            //     } {
            //         for stmt in body.clone() {
            //             let res = self.execute(stmt)?;
            //             if res.is_some() {
            //                 return Ok(res);
            //             }
            //         }
            //     }
            //     Ok(None)
            // }

            Stmt::Switch {
                condition,
                cases,
                default,
            } => {
                let val = self.evaluate(condition)?;
                let mut executed = false;

                for (case_expr, body) in cases {
                    let case_val = self.evaluate(case_expr)?;
                    if val == case_val {
                        let res = self.execute_function_body(body)?;
                        if res.is_some() {
                            return Ok(res);
                        }
                        executed = true;
                        break;
                    }
                }
                if !executed {
                    if let Some(default_body) = default {
                        let res = self.execute_function_body(default_body)?;
                        if res.is_some() {
                            return Ok(res);
                        }
                    }
                }
                Ok(None)
            }

            _ => Ok(None),
        }
    }

    fn type_check(&self, expected: &Token, value: &Value) -> Result<(), String> {
        match (expected, value) {
            (Token::IntType, Value::Int(_)) => Ok(()),
            (Token::FloatType, Value::Float(_)) => Ok(()),
            (Token::DoubleType, Value::Double(_)) => Ok(()),
            (Token::BoolType, Value::Bool(_)) => Ok(()),
            (Token::StringType, Value::String(_)) => Ok(()),
            (Token::CharType, Value::Char(_)) => Ok(()),

            _ => Err(format!(
                "Error de tipo: se esperaba {:?} pero se recibió {:?}",
                expected, value
            )),
        }
    }

    fn evaluate(&mut self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Unary { operator, right } => {
                let val = self.evaluate(*right)?;
                match operator {
                    Token::Minus => match val {
                        Value::Int(n) => Ok(Value::Int(-n)),
                        Value::Float(f) => Ok(Value::Float(-f)),
                        Value::Double(d) => Ok(Value::Double(-d)),
                        _ => Err(format!("Operador '-' no aplicable a {:?}", val)),
                    },
                    Token::Not => match val {
                        Value::Bool(b) => Ok(Value::Bool(!b)),
                        _ => Err(format!("Operador '!' no aplicable a {:?}", val)),
                    },
                    _ => Err(format!("Operador unario desconocido {:?}", operator)),
                }
            }
            Expr::Literal(token) => {
                let value = match token {
                    Token::IntegerLiteral(n) => Value::Int(n as i32),
                    Token::FloatLiteral(n) => Value::Float(n as f32),
                    Token::StringLiteral(s) => Value::String(s),
                    Token::CharLiteral(c) => Value::Char(c),
                    Token::BoolLiteral(b) => Value::Bool(b),
                    _ => Value::Void,
                };
                Ok(value)
            }
            Expr::Variable(name) => self
                .environment
                .get(&name)
                .map(|v| v.value)
                .ok_or_else(|| format!("Variable no definida: {}", name)),
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let l = self.evaluate(*left)?;
                let r = self.evaluate(*right)?;
                Ok(self.execute_binary_op(l, operator, r))
            }
            Expr::Call { callee, arguments } => {
                let (params, body) = self
                    .functions
                    .get(&callee)
                    .cloned()
                    .ok_or_else(|| format!("Función no definida: {}", callee))?;

                let mut args_values = Vec::new();
                for arg in arguments {
                    args_values.push(self.evaluate(arg)?);
                }

                let previous_env = std::mem::replace(
                    &mut self.environment,
                    Environment {
                        values: HashMap::new(),
                        parent: None,
                    },
                );
                self.environment.parent = Some(Box::new(previous_env));

                for (i, (ty, name)) in params.iter().enumerate() {
                    self.environment.insert(
                        name.clone(),
                        Variable {
                            var_type: ty.clone(),
                            value: args_values[i].clone(),
                        },
                    );
                }

                let result = self.execute_function_body(body);

                if let Some(parent_env) = self.environment.parent.take() {
                    self.environment = *parent_env;
                }

                match result {
                    Ok(Some(var)) => Ok(var.value),
                    Ok(None) => Ok(Value::Void),
                    Err(e) => Err(e),
                }
            }
            Expr::Input => {
                let mut input_text = String::new();
                std::io::stdin().read_line(&mut input_text).unwrap();
                let trimmed = input_text.trim();

                if trimmed == self.config.literals.r#true {
                    return Ok(Value::Bool(true));
                }
                if trimmed == self.config.literals.r#false {
                    return Ok(Value::Bool(false));
                }

                // Primero intentar i32
                if let Ok(n) = trimmed.parse::<i32>() {
                    return Ok(Value::Int(n));
                }

                // Luego intentar f64
                if let Ok(n) = trimmed.parse::<f64>() {
                    return Ok(Value::Double(n));
                }

                // Luego char si es 1 carácter
                if trimmed.len() == 1 {
                    return Ok(Value::Char(trimmed.chars().next().unwrap()));
                }

                // Si nada funciona → String
                Ok(Value::String(trimmed.to_string()))
            }
            _ => Ok(Value::Int(0)),
        }
    }

    fn execute_binary_op(&self, left: Value, op: Token, right: Value) -> Value {
        // Usamos referencias para evitar el error de "moved value"
        match (&left, &op, &right) {
            // --- ARITMÉTICA PARA ENTEROS (i32) ---
            (Value::Int(a), op, Value::Int(b)) => match op {
                Token::Plus => Value::Int(a + b),
                Token::Minus => Value::Int(a - b),
                Token::Multiply => Value::Int(a * b),
                Token::Divide => Value::Int(a / b),
                Token::Modulo => Value::Int(a % b),
                Token::EqualEqual => Value::Bool(a == b),
                Token::NotEqual => Value::Bool(a != b),
                Token::Greater => Value::Bool(a > b),
                Token::Less => Value::Bool(a < b),
                _ => panic!("Operador {:?} no soportado para Int", op),
            },

            // --- ARITMÉTICA PARA DOUBLE (f64) Y MIXTA ---
            (Value::Double(a), op, Value::Double(b)) => match op {
                Token::Plus => Value::Double(a + b),
                Token::Minus => Value::Double(a - b),
                Token::Multiply => Value::Double(a * b),
                Token::Divide => Value::Double(a / b),
                Token::EqualEqual => Value::Bool(a == b),
                _ => todo!("Soportar más operaciones de Double"),
            },

            // Promoción automática: Int con Double
            (Value::Int(a), op, Value::Double(b)) => match op {
                Token::Plus => Value::Double(*a as f64 + b),
                Token::Minus => Value::Double(*a as f64 - b),
                Token::Multiply => Value::Double(*a as f64 * b),
                Token::EqualEqual => Value::Bool(*a as f64 == *b),
                _ => panic!("Operación mixta Int/Double no soportada"),
            },

            // Promoción automática: Double con Int
            (Value::Double(a), op, Value::Int(b)) => match op {
                Token::Plus => Value::Double(a + *b as f64),
                Token::Minus => Value::Double(a - *b as f64),
                Token::Multiply => Value::Double(a * *b as f64),
                Token::EqualEqual => Value::Bool(*a == *b as f64),
                _ => panic!("Operación mixta Double/Int no soportada"),
            },

            // --- OPERACIONES DE STRINGS ---
            (Value::String(a), Token::Plus, Value::String(b)) => {
                Value::String(format!("{}{}", a, b))
            }

            // --- COMPARACIÓN GENÉRICA (Fall-through) ---
            (l, Token::EqualEqual, r) => Value::Bool(l == r),
            (l, Token::NotEqual, r) => Value::Bool(l != r),

            _ => panic!(
                "Operación no soportada o tipos incompatibles: {:?} {:?} {:?}",
                left, op, right
            ),
        }
    }

    fn is_truthy(&self, value: Value) -> bool {
        match value {
            Value::Bool(b) => b,
            Value::Int(n) => n != 0,
            Value::Void => false,
            _ => true,
        }
    }

    fn execute_function_body(&mut self, body: Vec<Stmt>) -> Result<Option<Variable>, String> {
        for stmt in body {
            let result = self.execute(stmt)?;
            if result.is_some() {
                return Ok(result);
            }
        }
        Ok(None)
    }

    fn evaluate_strings(&mut self, expr: Expr) -> Result<String, String> {
        match expr {
            Expr::Literal(Token::StringLiteral(val)) => Ok(val),
            _ => Ok(self.evaluate(expr)?.to_string()),
        }
    }
}
