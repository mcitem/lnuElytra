use crate::{
    Client, Error,
    error::R,
    utils::{ToHtml, UseInputValue},
};

impl Client {
    pub async fn check_login(&self) -> R<String> {
        self.get(&Client::LOGIN_URL)
            .send()
            .await?
            .doc()
            .await?
            .use_val(&Client::S_SESSION_USER_KEY)
            .map(|s| s.to_owned())
            .map_err(|_| Error::LoginFailed)
    }
}
