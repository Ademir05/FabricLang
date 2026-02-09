use::serde::Deserialize;

// Compiler Config
#[derive(Debug, Deserialize)]
pub struct CompilerConfig {
    pub name: String,
    pub executable_name: String,
    pub extension: String,
    pub version: String,
}

// Syntax Config
#[derive(Debug, Deserialize)]
pub struct SyntaxConfig {
    pub types: Types,
    pub keywords: Keywords,
    pub literals: Literals,
}

#[derive(Debug, Deserialize)]
pub struct Types {
    #[serde(rename = "IntType")]
    pub int_type: String,
    #[serde(rename = "BigIntType")]
    pub big_int_type: String,
    #[serde(rename = "FloatType")]
    pub float_type: String,
    #[serde(rename = "DoubleType")]
    pub double_type: String,
    #[serde(rename = "StringType")]
    pub string_type: String,
    #[serde(rename = "BoolType")]
    pub bool_type: String,
    #[serde(rename = "CharType")]
    pub char_type: String,
    #[serde(rename = "VoidType")]
    pub void_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Keywords {
    #[serde(rename = "If")]
    pub r#if: String,
    #[serde(rename = "Else")]
    pub r#else: String,
    #[serde(rename = "While")]
    pub r#while: String,
    #[serde(rename = "For")]
    pub r#for: String,
    #[serde(rename = "Switch")]
    pub switch: String,
    #[serde(rename = "Case")]
    pub case: String,
    #[serde(rename = "Default")]
    pub default: String,
    #[serde(rename = "Function")]
    pub function: String,
    #[serde(rename = "Return")]
    pub r#return: String,
}

#[derive(Debug, Deserialize)]
pub struct Literals {
    #[serde(rename = "True")]
    pub r#true: String,
    #[serde(rename = "False")]
    pub r#false: String,
}