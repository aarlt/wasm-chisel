use std::error::{self, Error};
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Unknown(String),
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> ConfigError {
        ConfigError::Io(e)
    }
}

impl error::Error for ConfigError {
    fn description(&self) -> &str {
        match self {
            ConfigError::Io(e) => e.description(),
            ConfigError::Unknown(s) => s.as_str(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Configuration error: {}", self.description())
    }
}
