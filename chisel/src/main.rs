extern crate libchisel;
extern crate parity_wasm;
#[macro_use]
extern crate clap;
extern crate serde;
extern crate serde_derive;
extern crate serde_yaml;

use std::fs::{read, read_to_string};
use std::process;

use libchisel::{
    checkstartfunc::*, deployer::*, remapimports::*, repack::*, trimexports::*, trimstartfunc::*,
    verifyexports::*, verifyimports::*,
};

use clap::{App, Arg, ArgMatches, SubCommand};
use parity_wasm::elements::{deserialize_buffer, serialize_to_file, Module, Serialize};
use serde_yaml::Value;

struct ChiselOptions {
    opts: HashMap<String, String>,
}

fn subcommand_run() {
    // Get config file.
    // Parse config file.
    // Prepare module set and options.
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
}
