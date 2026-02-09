use::serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub types: Types,
    pub keywords: Keywords,
}

#[derive(Debug, Deserialize)]
pub struct Types {
    pub IntType: String,
    pub BigIntType: String,
    pub FloatType: String,
    pub DoubleType: String,
    pub StringType: String,
    pub BoolType: String,
    pub CharType: String,
    pub VoidType: String,
}

#[derive(Debug, Deserialize)]
pub struct Keywords {
    pub If: String,
    pub Else: String,
    pub While: String,
    pub For: String,
    pub Switch: String,
    pub Case: String,
    pub Default: String,
    pub Function: String,
    pub Return: String,
}