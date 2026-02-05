// fn main() {
//     println!("FabricLang Compiler en Rust v0.1.0");
// }

use crate::compiler::lexer::Lexer;
mod compiler;

fn main() {
    // let input = "int edad = 25;\nstring msg = \"Hola\";";
    // let input = "if (a != b && c == true)";
    // let input = "int x = 10abc;";
    let input = "
    // Comentario inicial
    function suma(a, b) {
        return a + b;
    }
";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    println!("{:?}", tokens);
}
