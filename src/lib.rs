//! [使用文档](https://lnu-elytra.mcitem.net)

#[cfg(feature = "blocking")]
pub mod blocking;

#[cfg(feature = "__pyo3")]
pub mod pyo3;

#[cfg(feature = "__flutter")]
pub mod flutter;

mod course;
mod error;
mod method;
mod utils;

pub use course::{Course, Jxb};
pub use error::Error;
pub use method::SelectCourseResponse;

use reqwest::Url;

#[cfg(feature = "reqwest_cookie_store")]
use {reqwest_cookie_store::CookieStoreRwLock, std::sync::Arc};

use scraper::Selector;
use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug)]
pub struct Client {
    base_url: Url,
    client: reqwest::Client,
    stores: HashMap<String, String>, // input[type="hidden"]
    #[cfg(feature = "cookie_override")]
    cookie_override: Option<String>, // 覆盖cookie
    #[cfg(feature = "reqwest_cookie_store")]
    pub cookie_store: Arc<CookieStoreRwLock>,
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
    const S_INPUT_MMSFJM: LazyLock<Selector> =
        LazyLock::new(|| Selector::parse("input[type='hidden'][name='mmsfjm']").unwrap());

    pub fn new() -> Self {
        Self::new_with_base(Url::parse("http://jw.lingnan.edu.cn").unwrap())
    }

    pub fn new_with_base(backend: Url) -> Self {
        #[cfg(feature = "reqwest_cookie_store")]
        let cookie_store = Arc::new(CookieStoreRwLock::default());

        let client = reqwest::Client::builder();

        #[cfg(not(feature = "reqwest_cookie_store"))]
        let client = client.cookie_store(true);

        #[cfg(feature = "reqwest_cookie_store")]
        let client = client.cookie_provider(cookie_store.clone());

        Self {
            base_url: backend,
            client: client.build().unwrap(),
            stores: HashMap::new(),
            #[cfg(feature = "cookie_override")]
            cookie_override: None,
            #[cfg(feature = "reqwest_cookie_store")]
            cookie_store,
        }
    }

    pub fn set_base(mut self, backend: Url) -> Self {
        self.base_url = backend;
        self
    }
}
