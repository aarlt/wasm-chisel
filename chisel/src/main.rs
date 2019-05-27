mod errors;

use std::collections::HashMap;
use std::error::Error;
use std::fs::canonicalize;
use std::path::PathBuf;
use std::process;

use clap::{crate_description, crate_name, crate_version, App, Arg, ArgMatches, SubCommand};

use crate::errors::ConfigError;
use libchisel::{
    checkstartfunc::*, deployer::*, remapimports::*, repack::*, trimexports::*, trimstartfunc::*,
    verifyexports::*, verifyimports::*,
};

use log::debug;

const CHISEL_DEFAULT_CONFIG_PATH_RELATIVE: &'static str = "./chisel.yml";
const CHISEL_DEFAULT_CONFIG_PATH_RELATIVE_ALT: &'static str = "./.chisel.yml";

struct ChiselContext {
    config_path: PathBuf,
    opts: HashMap<String, String>,
}

impl ChiselContext {
    pub fn new() -> Result<Self, ConfigError> {
        let _config_path = canonicalize(CHISEL_DEFAULT_CONFIG_PATH_RELATIVE)
            .unwrap_or(canonicalize(CHISEL_DEFAULT_CONFIG_PATH_RELATIVE_ALT)?);

        Ok(ChiselContext {
            config_path: _config_path,
            opts: HashMap::new(),
        })
    }

    pub fn from_args(args: &ArgMatches) -> Result<Self, ConfigError> {
        let config_path_match = args.value_of("CONFIG");

        let _config_path = if let Some(path) = config_path_match {
            canonicalize(path.to_string())?
        } else {
            canonicalize(CHISEL_DEFAULT_CONFIG_PATH_RELATIVE)
                .unwrap_or(canonicalize(CHISEL_DEFAULT_CONFIG_PATH_RELATIVE_ALT)?)
        };

        Ok(ChiselContext {
            config_path: _config_path,
            opts: HashMap::new(),
        })
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
        ("run", args) => {
            let context = if let Some(matches) = args {
                ChiselContext::from_args(matches).unwrap_or_else(|err| {
                    exit_with_error(1, &format!("Failed to configure! {}", err.description()))
                })
            } else {
                ChiselContext::new().unwrap_or_else(|err| {
                    exit_with_error(1, &format!("Failed to configure! {}", err.description()))
                })
            };
            process::exit(0);
        }
        _ => exit_with_error(-1, "No subcommand provided"),
    };
}
