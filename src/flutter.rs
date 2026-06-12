use delegate::delegate;

pub struct FClient(crate::Client);

pub struct FError {
    pub error: String,
    pub kind: FErrorKind,
}

pub enum FErrorKind {
    NotyetStarted,
    JxbNotFound,
    InvalidXhId,
    LoginFailed,
    Missing,
    MissingField,
    Reqwest,
    Rsa,
    ParseRsaKeyError,
    Base64Decode,
    SystemTime,
}

impl From<crate::Error> for FError {
    fn from(e: crate::Error) -> Self {
        Self {
            error: format!("{e:?}"),
            kind: match e {
                crate::Error::NotyetStarted => FErrorKind::NotyetStarted,
                crate::Error::JxbNotFound(_) => FErrorKind::JxbNotFound,
                crate::Error::InvalidXhId => FErrorKind::InvalidXhId,
                crate::Error::LoginFailed => FErrorKind::LoginFailed,
                crate::Error::Missing(_) => FErrorKind::Missing,
                crate::Error::MissingField(_) => FErrorKind::MissingField,
                crate::Error::Reqwest(_) => FErrorKind::Reqwest,
                crate::Error::Rsa(_) => FErrorKind::Rsa,
                crate::Error::ParseRsaKeyError(_) => FErrorKind::ParseRsaKeyError,
                crate::Error::Base64Decode(_) => FErrorKind::Base64Decode,
                crate::Error::SystemTime(_) => FErrorKind::SystemTime,
            },
        }
    }
}

impl FClient {
    /// flutter_rust_bridge:sync
    pub fn new() -> Self {
        Self(crate::Client::new())
    }

    pub fn new_with_base(backend: &str) -> Result<Self, String> {
        Ok(Self(crate::Client::new_with_base(
            url::Url::parse(backend).map_err(|e| e.to_string())?,
        )))
    }

    delegate! {
        #[expr($.map_err(Into::into))]
        to self.0 {
            pub async fn login(&mut self, username: &str, password: &str) -> Result<(), FError>;
            pub async fn check_login(&self) -> Result<String, FError>;
            pub async fn init(&mut self) -> Result<(), FError>;
            pub async fn fetch_courses(&self, q: &str) -> Result<crate::Course, FError>;
            pub async fn select_course(&self, course_id: &str, course_do_id: &str) -> Result<crate::SelectCourseResponse, FError>;
        }
    }

    #[cfg(feature = "cookie_override")]
    delegate! {
        to self.0 {
           pub fn set_cookie_override(&mut self, cookie: String);
           pub fn clear_cookie_override(&mut self);
        }
    }
}
