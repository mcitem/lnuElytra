use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("[init_firstXkkzId] 不存在，未到选课时间")]
    NotyetStarted,

    #[error("[{0}] 找不到教学班")]
    JxbNotFound(&'static str),

    #[error("[select_course] 学号长度不足")]
    InvalidXhId,

    #[error("Login failed")]
    LoginFailed,

    #[error("Missing field: {0}")]
    Missing(String),

    #[error("Missing field: {0}")]
    MissingField(&'static str),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("RSA error: {0}")]
    Rsa(#[from] rsa::errors::Error),

    #[error("RSA key parse error: {0}")]
    ParseRsaKeyError(&'static str),

    #[error("Base64 decode error: {0}")]
    Base64Decode(#[from] base64::DecodeError),

    #[error("System time error: {0}")]
    SystemTime(#[from] std::time::SystemTimeError),
}

pub type R<T = ()> = Result<T, Error>;
