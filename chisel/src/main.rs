mod errors;

use std::collections::HashMap;
use std::error::Error;
use std::fs::canonicalize;
use std::path::PathBuf;
use std::process;

use clap::{crate_description, crate_name, crate_version, App, Arg, ArgMatches, SubCommand};
use log::debug;

use libchisel::{
    checkstartfunc::*, deployer::*, remapimports::*, repack::*, trimexports::*, trimstartfunc::*,
    verifyexports::*, verifyimports::*,
};

use errors::ChiselError;
use errors::ConfigError;

const CHISEL_DEFAULT_CONFIG_PATH: &'static str = "./chisel.yml";
const CHISEL_DEFAULT_CONFIG_PATH_ALT: &'static str = "./.chisel.yml";

struct ChiselContext {
    config_path: PathBuf,
    opts: HashMap<String, String>,
}

// impl ChiselContext {
//
// }

fn resolve_config_path(matched: Option<&str>) -> Result<PathBuf, ConfigError> {
    if let Some(arg) = matched {
        match canonicalize(arg.to_string()) {
            Ok(ret) => Ok(ret),
            Err(_) => Err(ConfigError::NotFound(Some(format!(
                "Could not resolve config file path: {}",
                arg
            )))),
        }
    } else {
        if let Ok(default_path) = canonicalize(CHISEL_DEFAULT_CONFIG_PATH) {
            Ok(default_path)
        } else {
            match canonicalize(CHISEL_DEFAULT_CONFIG_PATH_ALT) {
                // FIXME: Handle permission errors as well
                Ok(ret) => Ok(ret),
                Err(_) => Err(ConfigError::NotFound(None)),
            }
        }
    }
}

/// Execute chisel given a configuration.
fn subcommand_run(args: Option<&ArgMatches>) -> Result<(), ChiselError> {
    // Get config file.
    if let Some(matches) = args {
        let config_path = resolve_config_path(matches.value_of("CONFIG"))?;
    }
    // Parse config file.
    // Prepare module set and options.
    Ok(())
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
            match subcommand_run(args) {
                Ok(()) => process::exit(0),
                Err(e) => exit_with_error(1, e.description()),
            };
        }
        _ => exit_with_error(-1, "No subcommand provided"),
    };
}
