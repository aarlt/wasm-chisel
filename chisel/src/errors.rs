use std::error::{self, Error};
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ChiselError {
    Config(ConfigError),
    Unknown(String),
}

#[derive(Debug)]
pub enum ConfigError {
    NotFound(Option<String>),
    Unknown(String),
}

impl error::Error for ChiselError {
    fn description(&self) -> &str {
        match self {
            ChiselError::Config(e) => e.description(),
            ChiselError::Unknown(s) => s.as_str(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl error::Error for ConfigError {
    fn description(&self) -> &str {
        match self {
            ConfigError::NotFound(e) => {
                if let Some(s) = e {
                    s.as_str()
                } else {
                    "Could not find a configuration file"
                }
            }
            ConfigError::Unknown(s) => s.as_str(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error: {}", self.description())
    }
}

impl fmt::Display for ChiselError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error: {}", self.description())
    }
}

impl From<ConfigError> for ChiselError {
    fn from(e: ConfigError) -> Self {
        ChiselError::Config(e)
    }
}
