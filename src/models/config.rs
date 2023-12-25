use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;
use tracing::{debug, info};
use super::Error;

#[derive(Deserialize, Serialize)]
pub struct Config{
    pub last_id: String,
}

impl Config {
    pub fn new(last_id: &str) -> Self{
        Config { last_id: last_id.to_string() }
    }

    pub fn read(filename: &str) -> Result<Config, Error>{
        info!("read");
        if Path::new(filename).exists(){
            let data = fs::read_to_string(filename)?;
            debug!("{}", data);
            let config: Config = toml::from_str(&data)?;
            Ok(config)
        }else{
            let config = Self::new("0");
            config.save(filename)?;
            Ok(config)
        }
    }

    pub fn save(&self, filename: &str) -> Result<(), std::io::Error>{
        info!("save");
        let toml = toml::to_string(&self).unwrap();
        fs::write(filename, toml)
    }

    pub fn get_last_id(&self) -> &str{
        &self.last_id
    }
}
