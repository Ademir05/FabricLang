// use std::iter::Peekable;
// use std::str::Chars;
// use crate::compiler::lexer::Token::{self, *};

// pub struct Lexer<'a> {
//     chars: Peekable<Chars<'a>>,
// }

// impl<'a> Lexer<'a> {
//     pub fn new(input: &'a str) -> Self {
//         Self {
//             chars: input.chars().peekable(),
//         }
//     }

//     pub fn tokenize(&mut self) -> Vec<Token> {
//         let mut tokens = Vec::new();
//         while let Some(token) = self.next_token() {
//             let is_eof = token == EOF;
//             tokens.push(token);
//             if is_eof { break; }
//         }
//         tokens
//     }

//     fn next_token(&mut self) -> Option<Token> {
//         self.skip_whitespace();

//         let c = match self.chars.next() {
//             Some(c) => c,
//             None => return Some(EOF),
//         };

//         match c {
//             ';' => Some(Semi),
//             '=' => Some(Assign),
//             '+' => Some(Plus),
//             'a'..='z' | 'A'..='Z' | '_' => {
//                 let mut identifier = String::from(c);
//                 while let Some(&next) = self.chars.peek() {
//                     if next.is_alphanumeric() || next == '_' {
//                         identifier.push(self.chars.next().unwrap());
//                     } else {
//                         break;
//                     }
//                 }
                
//                 // Verificar si es palabra reservada
//                 match identifier.as_str() {
//                     "int" => Some(IntType),
//                     _ => Some(Identifier(identifier)),
//                 }
//             }
//             '0'..='9' => {
//                 let mut number_str = String::from(c);
//                 while let Some(&next) = self.chars.peek() {
//                     if next.is_ascii_digit() {
//                         number_str.push(self.chars.next().unwrap());
//                     } else {
//                         break;
//                     }
//                 }
//                 Some(Integer(number_str.parse().unwrap()))
//             }
//             _ => None, // Aquí manejarías errores léxicos
//         }
//     }

//     fn skip_whitespace(&mut self) {
//         while let Some(&c) = self.chars.peek() {
//             if c.is_whitespace() {
//                 self.chars.next();
//             } else {
//                 break;
//             }
//         }
//     }
// }