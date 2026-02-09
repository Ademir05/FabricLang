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
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDeclaration {
        ty: Token,
        name: String,
        initializer: Option<Expr>,
    },
    Expression(Expr),
    Return(Expr),
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    // Aquí podrías añadir While, For, etc.
}