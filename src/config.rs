use std::error::Error;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::custom_error::CustomError;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server: String,
    pub token: String,
}

impl Config {
    fn new(server: String, token: String) -> Self {
        Self {
            server,
            token,
        }
    }
    
    pub fn clone(&self) -> Self {
        Self {
            server: self.server.clone(),
            token: self.token.clone(),
        }
    }
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let config_path = get_config_file()?;
    let config_raw = fs::read_to_string(config_path);
    let mut config = Config::new("".to_string(), "".to_string());
    if config_raw.is_err()
    {
        // no config file found. Create empty config file
        set_config(config.clone())?;
    } else {
        let file_config = toml::from_str::<Config>(&config_raw.unwrap());
        if file_config.is_err()
        {
            return Err(
                Box::new(
                    CustomError(
                        "Could not parse config file"
                            .into()
                    )
                )
            )
        }
        config = file_config.unwrap();
    }
    Ok(config)
}

pub fn set_token(token: String) -> Result<(), Box<dyn Error>> {
    let mut config = get_config()?;
    config.token = token;
    set_config(config)
}

pub fn set_server(server: String) -> Result<(), Box<dyn Error>> {
    let mut config = get_config()?;
    config.server = server;
    set_config(config)
}

fn set_config(config: Config) -> Result<(), Box<dyn Error>> {
    let config_path = get_config_file()?;
    let config_raw = toml::to_string(&config)?;
    fs::write(config_path, config_raw)?;
    Ok(())
}

#[cfg(unix)]
fn get_config_file() -> Result<PathBuf, Box<dyn Error>> {
    let home = dirs::home_dir().ok_or("Could not determine home directory")?;
    let config_folder = home.join(".config/thunderboard");
    fs::create_dir_all(config_folder)?;
    let config_path = home.join(".config/thunderboard/config.toml");

    Ok(config_path)
}

#[cfg(windows)]
fn get_config_file() -> Result<PathBuf, Box<dyn Error>> {
    let home = dirs::home_dir().ok_or("Could not determine home directory")?;
    let config_folder = home.join("AppData\\Roaming\\thunderboard");
    fs::create_dir_all(config_folder)?;
    let config_path = home.join("AppData\\Roaming\\thunderboard\\config.toml");
    Ok(config_path)
}