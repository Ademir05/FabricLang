use::serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CompilerConfig {
    pub name: String,
    pub executable_name: String,
    pub extension: String,
    pub version: String,
}