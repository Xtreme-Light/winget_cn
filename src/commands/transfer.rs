use clap::Command;

use crate::{errors::ApplicationError, utils::stop_watch::StopWatch};


const TRANSFER: &'static str = "transfer";

pub(crate) fn configure() -> Command {
    Command::new(TRANSFER).short_flag('t').about("进行winget source 的代理转换")
}

pub(crate) fn handle(matches: &clap::ArgMatches) -> Result<(),ApplicationError> {

    if let Some(_matches) = matches.subcommand_matches(TRANSFER){
        StopWatch::new(Some("winget source 的下载网址转换".to_string()))
        .running(||{
            
            
        });
    }

    Ok(())
}

