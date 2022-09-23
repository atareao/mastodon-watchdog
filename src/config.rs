use toml;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct Config{
    last_id: i64,
}

impl Config {
    pub fn new(last_id: i64) -> Self{
        Config { last_id }
    }

    pub fn read(filename: &str) -> Result<Config, std::io::Error>{
        if Path::new(filename).exists(){
            let data = fs::read_to_string(filename).unwrap();
            println!("{}", data);
            let config: Config = toml::from_str(&data).unwrap();
            return Ok(config);
        }
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"))
    }

    pub fn save(self, filename: &str) -> Result<(), std::io::Error>{
        let toml = toml::to_string(&self).unwrap();
        fs::write(filename, toml)
    }

    pub fn get_last_id(self) -> i64{
        self.last_id
    }
}
