use base64::{Engine, prelude::BASE64_STANDARD};
use reqwest::{Method, RequestBuilder, Response};
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use scraper::{Html, Selector, selector::ToCss};
use serde::{Deserialize, de::DeserializeOwned};

use crate::{
    Client,
    error::{Error, R},
    utils::macros::error,
};

pub(crate) mod macros {

    macro_rules! error {
        ($($arg:tt)*) => {
            #[cfg(feature = "tracing")]
            tracing::error!($($arg)*);
        };
    }

    macro_rules! tracing_warn{
        ($($arg:tt)*) => {
            #[cfg(feature = "tracing")]
            tracing::warn!($($arg)*);
        };
    }

    macro_rules! info {
        ($($arg:tt)*) => {
            #[cfg(feature = "tracing")]
            tracing::info!($($arg)*);
        };
    }

    macro_rules! debug {
        ($($arg:tt)*) => {
            #[cfg(feature = "tracing")]
            tracing::debug!($($arg)*);
        };
    }

    macro_rules! trace {
        ($($arg:tt)*) => {
            #[cfg(feature = "tracing")]
            tracing::trace!($($arg)*);
        };
    }

    pub(crate) use {debug, error, info, trace, tracing_warn as warn};
}

#[derive(Deserialize, Debug)]
pub struct PublicKey {
    modulus: String,
    exponent: String,
}

impl PublicKey {
    pub fn into_rsa_key(self) -> R<RsaPublicKey> {
        let rsa_n = hex::encode(BASE64_STANDARD.decode(self.modulus)?);
        let rsa_e = hex::encode(BASE64_STANDARD.decode(self.exponent)?);

        let n = rsa::BigUint::parse_bytes(rsa_n.as_bytes(), 16)
            .ok_or(crate::error::Error::ParseRsaKeyError("rsa_n"))?;
        let e = rsa::BigUint::parse_bytes(rsa_e.as_bytes(), 16)
            .ok_or(crate::error::Error::ParseRsaKeyError("rsa_e"))?;

        let public_key = RsaPublicKey::new(n, e)?;

        Ok(public_key)
    }
}

pub trait EncPwd {
    fn enc_pwd(&self, pwd: &str) -> R<String>;
}

impl EncPwd for RsaPublicKey {
    fn enc_pwd(&self, pwd: &str) -> R<String> {
        let encropy_pwd =
            self.encrypt(&mut rand::thread_rng(), Pkcs1v15Encrypt, &pwd.as_bytes())?;
        Ok(BASE64_STANDARD.encode(encropy_pwd))
    }
}

pub trait ToJson {
    async fn jsonr<T: DeserializeOwned>(self) -> R<T>;
}

impl ToJson for Response {
    async fn jsonr<T: DeserializeOwned>(self) -> R<T> {
        if self.url().path().contains(&Client::LOGIN_URL) {
            error!("登录已失效");
            return Err(Error::LoginFailed);
        };
        Ok(self.json().await?)
    }
}

pub trait ToHtml {
    async fn doc(self) -> R<Html>;
    async fn _doc(self) -> R<Html>;
}

impl ToHtml for Response {
    async fn doc(self) -> R<Html> {
        if self.url().path().contains(&Client::LOGIN_URL) {
            error!("登录已失效");
            return Err(Error::LoginFailed);
        };

        self._doc().await
    }

    #[inline(always)]
    async fn _doc(self) -> R<Html> {
        let text = self.text().await?;
        let doc = Html::parse_document(&text);
        Ok(doc)
    }
}

pub trait UseInputValue {
    fn use_val(&self, selector: &Selector) -> R<&str>;
}

impl UseInputValue for Html {
    fn use_val(&self, selector: &Selector) -> R<&str> {
        let value = self
            .select(selector)
            .next()
            .ok_or(Error::Missing(selector.to_css_string()))?
            .attr("value")
            .ok_or(Error::Missing(format!(
                "{} value",
                selector.to_css_string()
            )))?;
        Ok(value.into())
    }
}

impl Client {
    pub(crate) fn get(&self, url: &str) -> R<RequestBuilder> {
        self.request(Method::GET, url)
    }
    pub(crate) fn post(&self, url: &str) -> R<RequestBuilder> {
        self.request(Method::POST, url)
    }
    pub(crate) fn request(&self, method: Method, url: &str) -> R<RequestBuilder> {
        let url = self.base_url.join(url).map_err(Error::UrlParseError)?;

        #[allow(unused_mut)]
        let mut req = self.client.request(method, url);

        #[cfg(feature = "cookie_override")]
        if let Some(cookie) = &self.cookie_override {
            req = req.header(reqwest::header::COOKIE, cookie);
        }

        Ok(req)
    }
    pub(crate) fn store(&mut self, key: &str, value: &str) {
        self.stores.insert(key.into(), value.into());
    }

    pub(crate) fn use_store(&self, key: &str) -> &String {
        static EMPTY_STRING: String = String::new();
        self.stores.get(key).unwrap_or(&EMPTY_STRING)
    }
}

#[cfg(feature = "cookie_override")]
impl Client {
    pub fn set_cookie_override(&mut self, cookie: String) {
        self.cookie_override = Some(cookie);
    }

    pub fn clear_cookie_override(&mut self) {
        self.cookie_override = None;
    }
}

#[cfg(feature = "reqwest_cookie_store")]
impl Client {
    pub fn cookies(&self) -> Option<String> {
        let store = self.cookie_store.read().ok()?;
        let res = store
            .iter_any()
            .filter_map(|x| match x.name() {
                "JSESSIONID" => Some(format!("{}={}", x.name(), x.value())),
                "X-LB" => Some(format!("{}={}", x.name(), x.value())),
                _ => None,
            })
            .collect::<Vec<String>>();
        match res.len() {
            0 => match self.cookie_override {
                None => None,
                Some(ref cookie) => Some(cookie.clone()),
            },
            _ => Some(res.join("; ")),
        }
    }
}
