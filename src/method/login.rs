use serde::Serialize;

use crate::{
    Client, def,
    error::{Error, R},
    utils::{
        EncPwd, PublicKey, ToHtml, UseInputValue, UseVer,
        macros::{debug, error, info, trace},
    },
};

impl Client {
    /// 登录
    pub async fn login(&mut self, username: &str, password: &str) -> R<String> {
        info!("正在登录...");

        trace!("加载登录页");

        let doc = self.get(def::LOGIN_URL)?.send().await?._doc().await?;

        trace!("解析登录页，获取csrftoken");

        let csrftoken = doc.use_val(&def::S_CSRFTOKEN)?;

        debug!("csrftoken: {}", csrftoken);

        use std::borrow::Cow;
        let mm = if let Ok(mmsfjm) = doc.use_val(&def::S_INPUT_MMSFJM)
            && mmsfjm == "0"
        {
            trace!("mmsfjm == '0'");
            Cow::Borrowed(password)
        } else {
            trace!("获取公钥，使用公钥加密密码");
            Cow::Owned(
                self.get(def::PUBLIC_KEY_URL)?
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
            csrftoken,
            yhm: username,
            mm: &mm,
            language: "zh_CN",
        };

        debug!("登录数据: {:?}", login_data);

        let timestamp = chrono::Utc::now().timestamp_millis();

        trace!("发送登录请求");

        let doc = self
            .post(def::LOGIN_URL)?
            .query(&[("time", timestamp)])
            .form(&login_data)
            .send()
            .await?
            ._doc()
            .await?;

        let u = doc.use_val(&def::S_SESSION_USER_KEY).map_err(|_| {
            error!("登录失败，未找到 SESSION_USER_KEY");
            Error::LoginFailed
        })?;

        debug!("SESSION_USER_KEY: {}", u);

        if u != username {
            error!("登录失败，SESSION_USER_KEY 不匹配");
            return Err(Error::LoginFailed);
        }

        info!("登录成功");

        let _ = doc.use_ver();

        Ok(u.to_owned())
    }
}
