use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use crate::{
    Client,
    error::{Error, R},
    utils::{
        EncPwd, PublicKey, ToHtml, UseInputValue,
        macros::{debug, error, info, trace},
    },
};

impl Client {
    /// 登录
    pub async fn login(&mut self, username: &str, password: &str) -> R {
        info!("正在登录...");

        trace!("加载登录页");

        let doc = self.get(&Client::LOGIN_URL).send().await?.doc().await?;

        trace!("解析登录页，获取csrftoken");

        let csrftoken = doc.use_val(&Client::S_CSRFTOKEN)?;

        debug!("csrftoken: {}", csrftoken);

        use std::borrow::Cow;
        let mm = if let Ok(mmsfjm) = doc.use_val(&Client::S_INPUT_MMSFJM)
            && mmsfjm == "0"
        {
            trace!("mmsfjm == '0'");
            Cow::Borrowed(password)
        } else {
            trace!("获取公钥，使用公钥加密密码");
            Cow::Owned(
                self.get(&Client::PUBLIC_KEY_URL)
                    .send()
                    .await?
                    .json::<PublicKey>()
                    .await?
                    .into_rsa_key()?
                    .enc_pwd(password)?,
            )
        };

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

        debug!("登录数据: {:?}", login_data);

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

        trace!("发送登录请求");

        let login_res = self
            .post(&Client::LOGIN_URL)
            .query(&[("time", timestamp)])
            .form(&login_data)
            .send()
            .await?;

        let doc = login_res.doc().await?;

        let u = doc.use_val(&Client::S_SESSION_USER_KEY).or_else(|_| {
            error!("登录失败，未找到用户名");
            return Err(Error::LoginFailed);
        })?;

        debug!("SESSION_USER_KEY: {}", u);

        if u != username {
            error!("登录失败，用户名不匹配");
            return Err(Error::LoginFailed);
        }

        info!("登录成功");

        Ok(())
    }
}
