use clap::{Arg, Command};
use errors::ApplicationError;

pub mod commands;
pub mod errors;
pub mod utils;

fn main() -> Result<(), ApplicationError> {
    println!("Hello, world!");

    let command = Command::new("winget cn tool")
        .version("1.0")
        .author("xtreme light")
        .about("对winget的源进行国产化封装，使其下载可以走国内代理，从而更加易用")
        .arg(
            Arg::new("position")
                .short('p')
                .long("posotion")
                .default_missing_value(".")
                .help("winget source的位置"),
        );

    let command = commands::configure(command);

    let matches = command.get_matches();

    commands::handle(&matches)?;

    Ok(())
}
