mod config;
mod errors;
mod run;

use std::process;

use clap::{crate_description, crate_name, crate_version, App, Arg, ArgMatches, SubCommand};

use run::subcommand_run;

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
            subcommand_run(args).unwrap_or_else(|e| exit_with_error(1, e.description()))
        }
        _ => exit_with_error(-1, "No subcommand provided"),
    };
}
