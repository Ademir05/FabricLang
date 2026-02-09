pub mod core {
    pub mod lexer;
    pub mod token;
    pub mod parser;
    pub mod ast;
}

pub mod persistence {
    pub mod loader;
    pub mod models;
}