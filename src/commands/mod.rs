use clap::Command;

use crate::errors::ApplicationError;

pub mod fresh;
pub mod transfer;


pub(crate) fn configure(command: clap::Command) -> Command {
    command.subcommand(fresh::configure())
    .subcommand(transfer::configure())
}

pub(crate) fn handle(matches: &clap::ArgMatches) -> Result<(),ApplicationError> {
    fresh::handle(matches)?;
    transfer::handle(matches)?;
    Ok(())
}