use std::collections::HashMap;
use std::error::Error;
use std::fs::canonicalize;
use std::path::PathBuf;

use crate::errors::ConfigError;

const CHISEL_DEFAULT_CONFIG_PATH: &'static str = "./chisel.yml";
const CHISEL_DEFAULT_CONFIG_PATH_ALT: &'static str = "./.chisel.yml";

/// The configuration structure, parsed directly from the configuration file.
pub struct Configuration(Vec<Ruleset>);

pub struct Ruleset {
    file: PathBuf,
    out: Option<PathBuf>,
    module_configurations: Vec<ModuleConfiguration>,
}

pub struct ModuleConfiguration {
    flags: HashMap<String, String>,
}

pub fn resolve_config_path(matched: Option<&str>) -> Result<PathBuf, Box<dyn Error>> {
    if let Some(arg) = matched {
        match canonicalize(arg.to_string()) {
            Ok(ret) => Ok(ret),
            Err(_) => Err(ConfigError::NotFound(Some(format!(
                "Could not resolve config file path: {}",
                arg
            )))
            .into()),
        }
    } else {
        if let Ok(default_path) = canonicalize(CHISEL_DEFAULT_CONFIG_PATH) {
            Ok(default_path)
        } else {
            match canonicalize(CHISEL_DEFAULT_CONFIG_PATH_ALT) {
                // FIXME: Handle permission errors as well
                Ok(ret) => Ok(ret),
                Err(_) => Err(ConfigError::NotFound(None).into()),
            }
        }
    }
}
