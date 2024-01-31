

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("命令{0}处理失败")]
    CommandHandleError(String),
    #[error("文件{0}不存在")]
    FileNotExist(String),
    #[error("网络异常")]
    Internet,
    #[error("未找到git程序异常")]
    NotFoundGitError,
    #[error("git 仓库clone 异常")]
    GitCloneError,
    #[error("未知异常")]
    Unknown,
}


