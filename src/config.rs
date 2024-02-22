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
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let config_path = get_config_file()?;
    let config_raw = fs::read_to_string(config_path);
    if config_raw.is_err()
    {
        // recreate empty config file
        let config = Config::new("".to_string(), "".to_string());
        set_config(config)?;

        return Err(
            Box::new(
                CustomError(
                    "Could not read config file in ~/.config/soundboard-cli/config.toml or AppData\\Roaming\\soundboard-cli\\config.toml"
                        .into()
                )
            )
        )
    }
    let config = toml::from_str::<Config>(&config_raw.unwrap());
    if config.is_err()
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
    Ok(config.unwrap())
}

pub fn set_token(token: String) -> Result<(), Box<dyn Error>> {
    let mut config = get_config();
    if config.is_err()
    {
        config = Ok(Config::new("".to_string(), "".to_string()));
    }
    let mut safe_config = config.unwrap();
    safe_config.token = token;
    set_config(safe_config)
}

pub fn set_server(server: String) -> Result<(), Box<dyn Error>> {
    let mut config = get_config();
    if config.is_err()
    {
        config = Ok(Config::new("".to_string(), "".to_string()));
    }
    let mut safe_config = config.unwrap();
    safe_config.server = server;
    set_config(safe_config)
}

fn set_config(config: Config) -> Result<(), Box<dyn Error>> {
    let config_path = get_config_file()?;
    let config_raw = toml::to_string(&config)?;
    fs::write(config_path, config_raw)?;
    Ok(())
}

#[cfg(unix)]
fn get_config_file() -> Result<PathBuf, Box<dyn Error>> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    let config_path = home.join(".config/soundboard-cli/config.toml");

    Ok(config_path)
}

#[cfg(windows)]
fn get_config_file() -> Result<PathBuf, Box<dyn Error>> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    let config_path = home.join("AppData\\Roaming\\soundboard-cli\\config.toml");
    Ok(config_path)
}