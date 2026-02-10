use fabric_lang::core::lexer::Lexer;
use fabric_lang::core::parser::Parser;
use fabric_lang::persistence::{self, loader};


fn test_lexer() {
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

    let mut lexer = Lexer::new(input, &loader::read_config_file("src/config/syntax.toml").unwrap());
    let tokens = lexer.tokenize();

    for token in &tokens {
        println!(
            "Token: {:?}, Line: {}, Column: {}",
            token.kind, token.line, token.col
        );
    }
}

fn main() {
    let input = "int edad = (10 * 5) + 2;";

    let syntax_config = persistence::loader::read_config_file("src/config/syntax.toml").unwrap();

    let mut lexer = Lexer::new(input, &syntax_config);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    println!("{:#?}", ast);
}