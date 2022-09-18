use toml;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct Config{
    last_id: Option<String>,
}

impl Config {
    pub fn new(filename: &str) -> Self{
        Config { last_id: Some(filename.to_string()) }
    }

    pub fn read(filename: &str) -> Result<Config, std::io::Error>{
        if Path::new(filename).exists(){
            let data = fs::read_to_string(filename).unwrap();
            let conf: Config = toml::from_str(&data).unwrap();
            return Ok(conf);
        }
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"))
    }

    pub fn get_last_id(self) -> String{
        self.last_id.unwrap()
    }
    pub fn save(self, filename: &str) -> Result<(), std::io::Error>{
        let toml = toml::to_string(&self).unwrap();
        fs::write(filename, toml)
    }
}

pub fn read_config(filename: &str) -> Result<Config, std::io::Error>{
    let data = fs::read_to_string(filename).unwrap();
    let conf: Config = toml::from_str(&data).unwrap();
    Ok(conf)
}

pub fn save_config(filename: &str, config: Config) -> Result<(), std::io::Error>{
    let toml = toml::to_string(&config).unwrap();
    fs::write(filename, toml)
}
