use fabric_lang::core::lexer::Lexer;
use fabric_lang::core::parser::Parser;
use fabric_lang::persistence::{self};

fn main() {
//     let input = "
//     int x = (10 + 5) * 2;
//     x = 100;
// ";
let input = "
    int function suma(int a, int b) {
        return a + b;
    }

    void function saludar() {
        // código...
    }

    int x = suma(5, 10);
    ";
    // let input = "int edad = 10 * 5 + 2;";

    let syntax_config = persistence::loader::read_config_file("src/config/syntax.toml").unwrap();

    let mut lexer = Lexer::new(input, &syntax_config);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    println!("{:#?}", ast);
}