use std::convert::TryFrom;
use std::error::Error;
use std::env;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    cycle_duration: i32,
    short_break: i32,
    long_break: i32,
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound,
    BadConfiguration,
}

impl Config {
    pub fn guess_path() -> Result<String, Box<dyn Error>> {
        match env::var("POMORS_CONFIG") {
            Ok(val) => Ok(val),
            Err(e) => {
                if let Some(mut path) = dirs::config_dir() {
                    path.push("pomors.toml");
                    Ok(String::from(path.to_str().unwrap()))
                } else {
                    Err(Box::new(e))
                }
            },
        }
    }
}
impl TryFrom<&str> for Config {
    type Error = ConfigError;
    fn try_from(buffer: &str) -> Result<Self, Self::Error> {
        let config: Result<Config, toml::de::Error> = toml::from_str(buffer);

        match config {
            Ok(c) => Ok(c),
            Err(e) => Err(ConfigError::from(e)),
        }
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(_: toml::de::Error) -> Self {
        ConfigError::BadConfiguration
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::BadConfiguration => write!(f, "Bad configuration file"),
            ConfigError::FileNotFound => write!(f, "Could not find the configuration file"),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}
