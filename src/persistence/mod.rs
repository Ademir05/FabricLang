use std::fs;

pub mod syntax;
pub mod compiler;

pub fn read_config_file<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, std::io::Error>{
    let toml_str = fs::read_to_string(path).expect("Error al leer el archivo");
    let config: T = toml::from_str(&toml_str).expect("Error al parsear el archivo");
    Ok(config)
}