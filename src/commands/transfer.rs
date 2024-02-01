use std::{fs::File, path::Path};

use clap::Command;
use walkdir::WalkDir;

use crate::{
    errors::ApplicationError,
    utils::{
        git_operate::{git_clone, git_exist},
        stop_watch::StopWatch,
    },
};
use std::io::{self, BufRead};

const TRANSFER: &'static str = "transfer";

pub(crate) fn configure() -> Command {
    Command::new(TRANSFER)
        .short_flag('t')
        .about("进行winget source 的代理转换")
}

pub(crate) fn handle(matches: &clap::ArgMatches) -> Result<(), ApplicationError> {
    if let Some(_matches) = matches.subcommand_matches(TRANSFER) {
        StopWatch::new(Some("winget source 的下载网址转换".to_string())).running(|| {
            git_exist()?;
            //  应该在这里读取配置，获取应该clone的地址和本地位置。同时配置应该提供默认的仓库地址和本地位置
            let url = "https://github.com/microsoft/winget-pkgs.git";
            let path = "C:\\Users\\Administrator\\Desktop\\winget-pkgs";
            git_clone(url, path)?;
            let clone_path = Path::new(path);
            if !clone_path.exists() {
                println!("winget-pkgs 不存在");
                return Err(ApplicationError::FileNotExist(path.to_string()));
            }
            let manifests_path = format!("{}\\manifests", path);

            for entry in WalkDir::new(manifests_path) {
                match entry {
                    Ok(entry) => {
                        if entry.path().is_file() {
                            if entry.path().ends_with("installer.yaml") {
                                if let Ok(lines) = read_lines(entry.path()) {
                                    for line in lines {
                                        if let Ok(content) = line {
                                            println!("读取到内容是 {}", content);
                                            if content.starts_with("  InstallerUrl: ") {
                                                let installer_url = &content[16..];
                                                if installer_url.starts_with("https://github.com")
                                                    || installer_url.starts_with(
                                                        "https://raw.githubusercontent.com",
                                                    )
                                                    || installer_url
                                                        .starts_with("http://github.com")
                                                    || installer_url.starts_with(
                                                        "http://raw.githubusercontent.com",
                                                    )
                                                {
                                                    println!("InstallerUrl: {}", installer_url);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
            Ok(())
        });
    }

    Ok(())
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
