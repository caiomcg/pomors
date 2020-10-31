use std::error::Error;
use std::env::VarError;

#[derive(Debug)]
pub enum ConfigError {
    BadConfiguration,
    InvalidPath,
}

impl From<toml::de::Error> for ConfigError {
    fn from(_: toml::de::Error) -> Self {
        ConfigError::BadConfiguration
    }
}

impl From<VarError> for ConfigError {
    fn from(_: VarError) -> Self {
        ConfigError::InvalidPath
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::BadConfiguration => write!(f, "Bad configuration file"),
            ConfigError::InvalidPath => write!(f, "Could not specificy the proper path to the configuration file")
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}
