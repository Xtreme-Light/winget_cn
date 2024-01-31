

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("命令{0}处理失败")]
    CommandHandleError(String),
    #[error("网络异常")]
    Internet,
    #[error("未知异常")]
    Unknown,
}


