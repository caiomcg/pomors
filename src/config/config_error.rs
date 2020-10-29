use std::error::Error;

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound,
    BadConfiguration,
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
