use crate::{
    Client, Error, def,
    error::R,
    utils::{ToHtml, UseInputValue},
};

impl Client {
    pub async fn check_login(&self) -> R<String> {
        self.get(def::LOGIN_URL)?
            .send()
            .await?
            ._doc()
            .await?
            .use_val(&def::S_SESSION_USER_KEY)
            .map(|s| s.to_owned())
            .map_err(|_| Error::LoginFailed)
    }
}
