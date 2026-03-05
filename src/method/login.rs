use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use crate::{
    Client,
    error::{Error, R},
    utils::{EncPwd, PublicKey, ToHtml, UseInputValue},
};

impl Client {
    /// 登录
    pub async fn login(&mut self, username: &str, password: &str) -> R {
        let doc = self.get(&Client::LOGIN_URL).send().await?.doc().await?;
        let csrftoken = doc.use_val(&Client::S_CSRFTOKEN)?;

        let mm = self
            .get(&Client::PUBLIC_KEY_URL)
            .send()
            .await?
            .json::<PublicKey>()
            .await?
            .into_rsa_key()?
            .enc_pwd(password)?;

        #[derive(Serialize, Debug)]
        struct LoginData<'a> {
            csrftoken: &'a str,
            yhm: &'a str,
            mm: &'a str,
            language: &'a str,
        }

        let login_data = LoginData {
            csrftoken: csrftoken,
            yhm: username,
            mm: &mm,
            language: "zh_CN",
        };

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

        let login_res = self
            .post(&Client::LOGIN_URL)
            .query(&[("time", timestamp)])
            .form(&login_data)
            .send()
            .await?;

        let doc = login_res.doc().await?;

        let u = doc.use_val(&Client::S_SESSION_USER_KEY)?;

        if u != username {
            return Err(Error::LoginFailed);
        }

        Ok(())
    }
}
