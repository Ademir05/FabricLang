use std::{env, fs};

use fabric_lang::core::interpreter::Interpreter;
use fabric_lang::core::lexer::Lexer;
use fabric_lang::core::parser::Parser;
use fabric_lang::persistence::models::CompilerConfig;
use fabric_lang::persistence::{self};

fn main() {
    let args: Vec<String> = env::args().collect();
    let compiler_config: CompilerConfig =
        persistence::loader::read_config_file("src/config/compiler.toml").unwrap();

    if args.len() < 2 {
        eprintln!("Uso: {} [-a] <archivo.{}>", args[0], compiler_config.extension);
        return;
    }

    // --- Detectar flags ---
    let mut show_ast = false;
    let mut file_path = "";

    for arg in &args[1..] {
        match arg.as_str() {
            "-a" => show_ast = true,
            "-h" => {
                println!("Uso: {} [-a] <archivo.{}>", args[0], compiler_config.extension);
                println!("Opciones:");
                println!("  -a    Mostrar AST en lugar de ejecutar");
                println!("  -h    Mostrar esta ayuda");
                return;
            }
            _ => file_path = arg,
        }
    }

    if file_path.is_empty() {
        eprintln!("Error: no se proporcionó archivo de entrada.");
        return;
    }

    if !file_path.ends_with(&compiler_config.extension) {
        eprintln!(
            "Error: el archivo debe tener extensión .{}",
            compiler_config.extension
        );
        return;
    }

    let input = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("No se pudo leer el archivo {}", file_path);
            return;
        }
    };

    let syntax_config = persistence::loader::read_config_file("src/config/syntax.toml").unwrap();

    let mut lexer = Lexer::new(&input, &syntax_config);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Lexical Error: {}", e.message);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Parser Error: {:?}", e);
            return;
        }
    };

    if show_ast {
        println!("{:#?}", ast);
        return;
    }

    let mut interpreter = Interpreter::new(syntax_config);
    interpreter.interpret(ast);
}