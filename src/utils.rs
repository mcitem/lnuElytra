use base64::{Engine, prelude::BASE64_STANDARD};
use reqwest::{Method, RequestBuilder, Response};
use rsa::{BoxedUint, Pkcs1v15Encrypt, RsaPublicKey};
use scraper::{Html, Selector, selector::ToCss};
use serde::{Deserialize, de::DeserializeOwned};
use std::{borrow::Cow, collections::HashMap};

use crate::{
    Client, def,
    error::{Error, R},
    utils::macros::{debug, error},
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
        let n_bytes = BASE64_STANDARD.decode(self.modulus)?;
        let e_bytes = BASE64_STANDARD.decode(self.exponent)?;
        let n = BoxedUint::from_be_slice_vartime(&n_bytes);
        let e = BoxedUint::from_be_slice_vartime(&e_bytes);
        let public_key = RsaPublicKey::new(n, e)?;

        Ok(public_key)
    }
}

pub trait EncPwd {
    fn enc_pwd(&self, pwd: &str) -> R<String>;
}

impl EncPwd for RsaPublicKey {
    fn enc_pwd(&self, pwd: &str) -> R<String> {
        let encropy_pwd = self.encrypt(&mut rand::rng(), Pkcs1v15Encrypt, pwd.as_bytes())?;
        Ok(BASE64_STANDARD.encode(encropy_pwd))
    }
}

pub trait ToJson {
    async fn jsonr<T: DeserializeOwned>(self) -> R<T>;
}

impl ToJson for Response {
    async fn jsonr<T: DeserializeOwned>(self) -> R<T> {
        if self.url().path().contains(def::LOGIN_URL) {
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
        if self.url().path().contains(def::LOGIN_URL) {
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
        Ok(value)
    }
}

pub trait UseVer {
    fn use_ver(&self) -> Option<String>;
}

impl UseVer for Html {
    fn use_ver(&self) -> Option<String> {
        let ver = self
            .select(&def::S_SCRIPT)
            .filter_map(|s| s.attr("src"))
            .filter_map(|s| s.split_once('?').map(|(_, q)| q))
            .flat_map(|q| url::form_urlencoded::parse(q.as_bytes()))
            .filter(|(k, _)| k == "ver")
            .fold(HashMap::<Cow<str>, usize>::new(), |mut m, (_, v)| {
                *m.entry(v).or_default() += 1;
                m
            })
            .into_iter()
            .max_by_key(|&(_, c)| c)
            .map(|(v, _)| v.into_owned());

        #[cfg(feature = "tracing")]
        if let Some(ref ver) = ver {
            crate::utils::macros::info!("ver: {}", ver);
        };

        ver
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
        let url = self.base_url.join(url)?;

        #[cfg(feature = "converter")]
        let url = (self.converter)(url)?;

        debug!("request: {} {}", method, url);

        #[allow(unused_mut)]
        let mut req = self.client.request(method, url);

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

#[cfg(feature = "reqwest_cookie_store")]
impl Client {
    pub fn cookies(&self) -> Option<String> {
        let store = self.cookie_store.read().ok()?;
        let res = store
            .iter_any()
            .filter_map(|x| match x.name() {
                "JSESSIONID" => Some(x),
                "X-LB" => Some(x),
                "zstack_cookie" => Some(x),
                "route" => Some(x),
                "wengine_vpn_ticketcsvpn_lingnan_edu_cn" => Some(x),
                _ => None,
            })
            .map(|x| format!("{}={}", x.name(), x.value()))
            .collect::<Vec<String>>();
        match res.len() {
            0 => None,
            _ => Some(res.join("; ")),
        }
    }

    pub fn insert_cookie(&self, cookie: &str) -> R {
        use crate::utils::macros::trace;

        let mut store = self.cookie_store.write().unwrap();

        let url = &self.base_url;

        #[cfg(feature = "converter")]
        let url = (self.converter)(url.join("/")?)?.join("/")?;

        trace!("url: {url}, insert cookie: {cookie}");

        store.parse(cookie, &url)?;

        Ok(())
    }

    pub fn clear_cookie(&self) {
        let mut store = self.cookie_store.write().unwrap();
        store.clear();
    }
}
