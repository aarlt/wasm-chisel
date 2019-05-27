extern crate libchisel;
extern crate parity_wasm;
#[macro_use]
extern crate clap;
extern crate serde;
extern crate serde_derive;
extern crate serde_yaml;

use std::fs::{read, read_to_string, canonicalize};
use std::io;
use std::error::{self, Error};
use std::process;
use std::fmt;
use std::collections::HashMap;
use std::path::PathBuf;

use libchisel::{
    checkstartfunc::*, deployer::*, remapimports::*, repack::*, trimexports::*, trimstartfunc::*,
    verifyexports::*, verifyimports::*,
};

use clap::{App, Arg, ArgMatches, SubCommand};
use parity_wasm::elements::{deserialize_buffer, serialize_to_file, Module, Serialize};
use serde_yaml::Value;

const CHISEL_DEFAULT_CONFIG_PATH_RELATIVE: &'static str = "chisel.yml";
const CHISEL_DEFAULT_CONFIG_PATH_RELATIVE_ALT: &'static str = ".chisel.yml";

#[derive(Debug)]
enum ConfigError {
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

struct ChiselContext {
    config_path: PathBuf,
    opts: HashMap<String, String>,
}

impl ChiselContext {
    pub fn new(args: &ArgMatches) -> Result<Self, ConfigError> {
        let config_path_match = args.value_of("CONFIG");

        let _config_path = if let Some(path) = config_path_match {
            canonicalize(path.to_string())?
        } else {
            canonicalize(CHISEL_DEFAULT_CONFIG_PATH_RELATIVE)
                .unwrap_or(canonicalize(CHISEL_DEFAULT_CONFIG_PATH_RELATIVE_ALT)?)
        };
        
        Ok(
            ChiselContext {
                config_path: _config_path,
                opts: HashMap::new(),
            }
        )
    }
}

/// Execute chisel given a configuration.
fn subcommand_run(context: &ChiselContext) {
    // Get config file.
    // Parse config file.
    // Prepare module set and options.
    unimplemented!();
}

fn exit_with_error(code: i32, message: &str) -> ! {
    eprintln!("{}: {}", crate_name!(), message);
    process::exit(code);
}

fn main() {
    let cli_matches = App::new("chisel")
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs chisel in config-driven mode.")
                .arg(
                    Arg::with_name("CONFIG")
                        .short("c")
                        .long("config")
                        .help("Overrides the configuration file")
                        .value_name("CONF_FILE")
                        .takes_value(true),
                ),
        )
        .get_matches();
    
    match cli_matches.subcommand() {
        ("run", args) => process::exit(0),
        _ => exit_with_error(-1, "No subcommand provided"),
    };
}
