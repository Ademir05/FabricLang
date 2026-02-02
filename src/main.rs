// fn main() {
    //     println!("FabricLang Compiler en Rust v0.1.0");
    // }
    
mod compiler;
use crate::compiler::lexer::Lexer;

fn main() {
    let input = "int x = 180;int y=200;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    println!("Código fuente: {}", input);
    println!("Tokens generados:");
    for token in tokens {
        println!("{:?}", token);
    }
}