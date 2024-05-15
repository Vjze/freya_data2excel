
use thiserror::Error;
#[derive(Error, Debug)]
pub enum MyError {
    #[error("网络连接失败!")]
    IoError(#[from] std::io::Error),
    #[error("输入不能为空!")]
    Empty,
    #[error("输入的号码没有数据!")]
    NoneError,
    #[error("数据库连接失败!")]
    SqlError(#[from] tiberius::error::Error),
    #[error("数据导出失败!")]
    ExportError,
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
}

