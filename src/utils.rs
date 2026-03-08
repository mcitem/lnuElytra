use base64::{Engine, prelude::BASE64_STANDARD};
use reqwest::{Method, RequestBuilder, Response};
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use scraper::{Html, Selector, selector::ToCss};
use serde::Deserialize;

use crate::{
    Client,
    error::{Error, R},
};

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

pub trait ToHtml {
    async fn doc(self) -> R<Html>;
}

impl ToHtml for Response {
    async fn doc(self) -> R<Html> {
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
    pub(crate) fn get(&self, url: &str) -> RequestBuilder {
        self.request(Method::GET, url)
    }
    pub(crate) fn post(&self, url: &str) -> RequestBuilder {
        self.request(Method::POST, url)
    }
    pub(crate) fn request(&self, method: Method, url: &str) -> RequestBuilder {
        #[allow(unused_mut)]
        let mut req = self
            .client
            .request(method, self.base_url.join(url).unwrap());

        #[cfg(feature = "cookie_override")]
        if let Some(cookie) = &self.cookie_override {
            req = req.header(reqwest::header::COOKIE, cookie);
        }

        req
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
