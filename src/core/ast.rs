use crate::core::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Literal(Token),
    Variable(String),
    Call {
        callee: String,
        arguments: Vec<Expr>,
    },
    Input,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDeclaration {
        ty: Token,
        name: String,
        initializer: Option<Expr>,
    },
    Expression(Expr),
    Return(Option<Expr>),
    Assignment {
        name: String,
        value: Expr,
    },
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    Block(Vec<Stmt>),
    Function {
        return_type: Token,
        name: String,
        params: Vec<(Token, String)>,
        body: Vec<Stmt>,
    },
    Print {
        value: Expr,
    },
    Println {
        value: Expr,
    },
    Switch {
        condition: Expr,
        cases: Vec<(Expr, Vec<Stmt>)>,
        default: Option<Vec<Stmt>>,
    },
}
