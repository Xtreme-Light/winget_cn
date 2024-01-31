use clap::Command;

use crate::errors::ApplicationError;

pub fn configure() -> Command{
    Command::new("fresh").about("刷新winget source").short_flag('f').long_flag("fresh")
}

pub(crate) fn handle(matches: &clap::ArgMatches) -> Result<(), ApplicationError> {
    if let Some(_matches) = matches.subcommand_matches("fresh") {
        println!("start fresh");
    }
    Ok(())
}
