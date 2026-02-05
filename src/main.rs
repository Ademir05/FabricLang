// fn main() {
//     println!("FabricLang Compiler en Rust v0.1.0");
// }

// use crate::compiler::lexer::Token;

mod compiler;
// use crate::compiler::lexer::Lexer;
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    IntType,
    Identifier(String),
    Integer(i64),
    Assign,
    Plus,
    DoubleEqual,
    Semi,
    EOF,
}

#[derive(Debug)]
struct LexicalError {
    message: String,
}

impl LexicalError {
    fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

#[derive(Debug)]
struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    fn next_token(&mut self) -> Result<Token, LexicalError> {
        self.skip_whitespace();
        
        if self.is_at_end() {
            return Ok(Token::EOF);
        }

        let c: char = self.peek();

        if c.is_alphabetic() {
            return self.read_identifier();
        }

        if c.is_ascii_digit() {
            return self.read_number();
        }

        match c {
            '=' => {
                self.position += 1;
                if self.input[self.position] == '=' {
                    self.position += 1;
                    return Ok(Token::DoubleEqual);
                } else {
                    return Ok(Token::Assign);
                }
            }
            ';' => {
                self.position += 1;
                return Ok(Token::Semi);
            }
            _ => Err(LexicalError::new("Not implemented")),
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn read_identifier(&mut self) -> Result<Token, LexicalError> {
        let start = self.position;

        println!("read_identifier: {}", self.input[self.position]);


        while !self.is_at_end() && (self.input[self.position].is_alphanumeric() || self.input[self.position] == '_') {
            self.position += 1;
        }
        let identifier: String = self.input[start..self.position].iter().collect();
        match identifier.as_str() {
            "int" => Ok(Token::IntType),
            _ => Ok(Token::Identifier(identifier)),
        }
    }

    fn read_number(&mut self) -> Result<Token, LexicalError> {
        let start = self.position;

        println!("read_number: {}", self.input[self.position]);


        while !self.is_at_end() && self.input[self.position].is_ascii_digit() {
            self.position += 1;
        }
        let identifier: String = self.input[start..self.position].iter().collect();
        let number = identifier.parse::<i64>();
        match number {
            Ok(n) => Ok(Token::Integer(n)),
            Err(_) => Err(LexicalError::new("Invalid integer literal")),
        }
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.position]
        }
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Ok(token) = self.next_token() {
            let is_eof = token == Token::EOF;
            tokens.push(token);
            if is_eof { break; }
        }
        tokens
    }  

}

fn main() {
    let input = "int 1x = 180;";

    let mut lexer = Lexer {
        input: input.chars().collect(),
        position: 0,
    };
    let tokens = lexer.tokenize();

    println!("{:?}", tokens);


    // let mut lexer = Lexer::new(input);
    // let tokens = lexer.tokenize();

    // println!("Código fuente: {}", input);
    // println!("Tokens generados:");
    // for token in tokens {
    //     println!("{:?}", token);
    // }
}
