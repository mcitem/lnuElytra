use delegate::delegate;

pub struct FClient(crate::Client);

pub struct FError {
    pub error: String,
    pub kind: FErrorKind,
}

pub enum FErrorKind {
    LoginFailed,
    NotyetStarted,
    JxbNotFound,
    Missing,
    Rsa,
    Reqwest,
    UrlParseError,
    Base64Decode,
    CookieError,
    ConverterError,
}

impl From<crate::Error> for FError {
    fn from(e: crate::Error) -> Self {
        Self {
            error: format!("{e:?}"),
            kind: match e {
                crate::Error::LoginFailed => FErrorKind::LoginFailed,
                crate::Error::NotyetStarted => FErrorKind::NotyetStarted,
                crate::Error::JxbNotFound(_) => FErrorKind::JxbNotFound,
                crate::Error::Missing(_) => FErrorKind::Missing,
                crate::Error::Rsa(_) => FErrorKind::Rsa,
                crate::Error::Reqwest(_) => FErrorKind::Reqwest,
                crate::Error::UrlParseError(_) => FErrorKind::UrlParseError,
                crate::Error::Base64Decode(_) => FErrorKind::Base64Decode,
                crate::Error::CookieError(_) => FErrorKind::CookieError,
                crate::Error::ConverterError(_) => FErrorKind::ConverterError,
            },
        }
    }
}

impl FClient {
    /// flutter_rust_bridge:sync
    pub fn new() -> Self {
        Self(crate::Client::new())
    }

    pub fn new_with_base(backend: &str) -> Result<Self, FError> {
        let backend: Result<_, FError> = url::Url::parse(backend)
            .map_err(crate::Error::UrlParseError)
            .map_err(Into::into);

        Ok(Self(crate::Client::new_with_base(backend?)))
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

    #[cfg(feature = "reqwest_cookie_store")]
    delegate! {
        to self.0 {
            #[expr($.map_err(Into::into))]
            pub fn insert_cookie(&mut self, cookie: &str) -> Result<(), FError>;
            pub fn clear_cookie(&mut self);
        }
    }
}
