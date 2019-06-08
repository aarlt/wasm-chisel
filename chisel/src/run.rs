use std::error::Error;

use clap::ArgMatches;

use crate::config::resolve_config_path;

/// Execute chisel in config-driven mode.
pub fn subcommand_run(args: Option<&ArgMatches>) -> Result<(), Box<dyn Error>> {
    // Get config file.
    if let Some(matches) = args {
        let config_path = resolve_config_path(matches.value_of("CONFIG"))?;
    }
    // Parse config file.
    // Prepare module set and options.
    Ok(())
}
