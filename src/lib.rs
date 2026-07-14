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

#[cfg(feature = "converter")]
use crate::error::R;

#[cfg(feature = "reqwest_cookie_store")]
use {reqwest_cookie_store::CookieStoreRwLock, std::sync::Arc};

use scraper::Selector;
use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug)]
pub struct Client {
    base_url: Url,
    client: reqwest::Client,
    stores: HashMap<String, String>, // input[type="hidden"]
    #[cfg(feature = "reqwest_cookie_store")]
    pub cookie_store: Arc<CookieStoreRwLock>,
    #[cfg(feature = "converter")]
    converter: fn(Url) -> R<Url>,
}

macro_rules! def {
    (u $name:ident = $value:literal; $($rest:tt)*) => {
        const $name: &str = $value;
        def!($($rest)*);
    };
    (s $name:ident = $sel:literal; $($rest:tt)*) => {
        const $name: LazyLock<Selector> = LazyLock::new(|| Selector::parse($sel).unwrap());
        def!($($rest)*);
    };
    () => {};
}

impl Client {
    def! {
    u JZIOTLOGIN_URL = "sso/jziotlogin";
    u LOGIN_URL = "xtgl/login_slogin.html";
    u PUBLIC_KEY_URL = "xtgl/login_getPublicKey.html";
    u SELECT_COURSE_URL = "xsxk/zzxkyzb_xkBcZyZzxkYzb.html?gnmkdm=N253512";
    u SELECT_COURSE_HTML_URL = "xsxk/zzxkyzb_cxZzxkYzbIndex.html?gnmkdm=N253512";
    u SELECT_COURSE_DISPLAY_URL = "xsxk/zzxkyzb_cxZzxkYzbDisplay.html?gnmkdm=N253512";
    u SELECT_COURSE_PART_DISPLAY_URL = "xsxk/zzxkyzb_cxZzxkYzbPartDisplay.html?gnmkdm=N253512";
    u SELECT_COURSE_QUERY_DO_WITH_COURSE_ID_URL = "xsxk/zzxkyzbjk_cxJxbWithKchZzxkYzb.html?gnmkdm=N253512";
    s S_CSRFTOKEN = "#csrftoken";
    s S_SESSION_USER_KEY = "#sessionUserKey";
    s S_INPUT_HIDDENT = "input[type='hidden']";
    s S_INPUT_MMSFJM = "input[type='hidden'][name='mmsfjm']";
    }

    pub fn new() -> Self {
        Self::new_with_base(Url::parse("http://jw.lingnan.edu.cn").unwrap())
    }

    pub fn new_with_base(backend: Url) -> Self {
        #[cfg(feature = "reqwest_cookie_store")]
        let cookie_store = Arc::new(CookieStoreRwLock::default());

        let client = reqwest::Client::builder();

        let client = client.user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Safari/537.36 Edg/149.0.0.0");

        #[cfg(not(feature = "reqwest_cookie_store"))]
        let client = client.cookie_store(true);

        #[cfg(feature = "reqwest_cookie_store")]
        let client = client.cookie_provider(cookie_store.clone());

        Self {
            base_url: backend,
            client: client.build().unwrap(),
            stores: HashMap::new(),
            #[cfg(feature = "reqwest_cookie_store")]
            cookie_store,
            #[cfg(feature = "converter")]
            converter: |url| Ok(url),
        }
    }

    pub fn set_base(mut self, backend: Url) -> Self {
        self.base_url = backend;
        self
    }

    #[cfg(feature = "converter")]
    pub fn set_converter(mut self, converter: fn(Url) -> R<Url>) -> Self {
        self.converter = converter;
        self
    }
}

#[cfg(feature = "converter")]
pub fn csvpn_converter(url: Url) -> R<Url> {
    let config = webvpn_converter::Config::default();
    Ok(config.encrypt_url(url)?)
}
