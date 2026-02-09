#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Types
    IntType,
    BigIntType,
    FloatType,
    DoubleType,
    StringType,
    BoolType,
    CharType,
    VoidType,

    // Literals
    Integer(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    CharLiteral(char),

    // Identifiers
    Identifier(String),

    // Aritmetic Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,

    // Logical Operators
    And,
    Or,

    // Parentheses
    LeftParen,
    RightParen,

    // Brackets
    LeftBracket,
    RightBracket,

    // Braces
    LeftBrace,
    RightBrace,

    // Commas
    Comma,

    // Comparison Operators
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Not,

    // Control Structures
    If,
    Else,
    While,
    For,
    Switch,
    Case,
    Default,

    // Functions
    Function,
    Return,

    Assign,
    Semi,
    EOF,
}

pub struct TokenData {
    pub kind: Token,
    pub line: usize,
    pub col: usize,
}

impl TokenData {
    pub fn new(kind: Token, line: usize, col: usize) -> Self {
        Self { kind, line, col }
    }
}