#[cfg(feature = "blocking")]
pub mod blocking;

#[cfg(feature = "__pyo3")]
pub mod pyo3;

mod course;
mod error;
mod method;
mod utils;

pub use course::{Course, Jxb};
pub use error::Error;
pub use method::SelectCourseResponse;

use reqwest::Url;
use scraper::Selector;
use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug)]
pub struct Client {
    base_url: Url,
    client: reqwest::Client,
    stores: HashMap<String, String>, // input[type="hidden"]
    #[cfg(feature = "cookie_override")]
    cookie_override: Option<String>, // 覆盖cookie
}

impl Client {
    const LOGIN_URL: &str = "/xtgl/login_slogin.html";
    const PUBLIC_KEY_URL: &str = "/xtgl/login_getPublicKey.html";

    const SELECT_COURSE_URL: &str = "/xsxk/zzxkyzb_xkBcZyZzxkYzb.html?gnmkdm=N253512";
    const SELECT_COURSE_HTML_URL: &str = "/xsxk/zzxkyzb_cxZzxkYzbIndex.html?gnmkdm=N253512";
    const SELECT_COURSE_DISPLAY_URL: &str = "/xsxk/zzxkyzb_cxZzxkYzbDisplay.html?gnmkdm=N253512";
    const SELECT_COURSE_PART_DISPLAY_URL: &str =
        "/xsxk/zzxkyzb_cxZzxkYzbPartDisplay.html?gnmkdm=N253512";
    const SELECT_COURSE_QUERY_DO_WITH_COURSE_ID_URL: &str =
        "/xsxk/zzxkyzbjk_cxJxbWithKchZzxkYzb.html?gnmkdm=N253512";

    const S_CSRFTOKEN: LazyLock<Selector> =
        LazyLock::new(|| Selector::parse("#csrftoken").unwrap());
    const S_SESSION_USER_KEY: LazyLock<Selector> =
        LazyLock::new(|| Selector::parse("#sessionUserKey").unwrap());
    const S_INPUT_HIDDENT: LazyLock<Selector> =
        LazyLock::new(|| Selector::parse("input[type='hidden']").unwrap());

    pub fn new() -> Self {
        Self::new_with_base(Url::parse("http://jw.lingnan.edu.cn").unwrap())
    }

    pub fn new_with_base(backend: Url) -> Self {
        Self {
            base_url: backend,
            client: reqwest::Client::builder()
                .cookie_store(true)
                .build()
                .unwrap(),
            stores: HashMap::new(),
            #[cfg(feature = "cookie_override")]
            cookie_override: None,
        }
    }

    pub fn with_base(mut self, backend: Url) -> Self {
        self.base_url = backend;
        self
    }
}
