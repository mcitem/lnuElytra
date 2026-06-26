use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Login failed")]
    LoginFailed,

    #[error("[init_firstXkkzId] 不存在，未到选课时间")]
    NotyetStarted,

    #[error("[{0}] 找不到教学班")]
    JxbNotFound(&'static str),

    #[error("Missing field: {0}")]
    Missing(String),

    #[error("RSA error: {0}")]
    Rsa(#[from] rsa::errors::Error),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("UrlParseError: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Base64 decode error: {0}")]
    Base64Decode(#[from] base64::DecodeError),
}

pub type R<T = ()> = Result<T, Error>;
