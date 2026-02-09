// fn main() {
//     println!("FabricLang Compiler en Rust v0.1.0");
// }

use crate::{compiler::lexer::Lexer, persistence::syntax::AppConfig};
mod compiler;

mod persistence;
use ::std::fs;

fn test_toml() {
    let toml_str = fs::read_to_string("src/config.toml").expect("Error al leer el archivo");
    let config: AppConfig = toml::from_str(&toml_str).expect("Error al parsear el archivo");
    println!("{:?}", config);
    println!("{:?}", config.types);
    println!("{:?}", config.keywords);
}

fn main() {
    test_toml();
    // let input = "int edad = 25;\nstring msg = \"Hola\";";
    // let input = "if (a != b && c == true)";
    // let input = "int x = 10abc;";
    let input = "// Comentario inicial
float num1 = 10.2;
int num2 = 5;
string = \"Hola\";
char = 'a';
bool = true;

    function suma(a, b) {
    return a + b;
}";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    for token in &tokens {
        println!(
            "Token: {:?}, Line: {}, Column: {}",
            token.kind, token.line, token.col
        );
    }
}
