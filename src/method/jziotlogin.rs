use crate::{
    Client,
    error::R,
    utils::{ToHtml, UseInputValue},
};

impl Client {
    pub async fn jziotlogin(&mut self) -> R<String> {
        self.get(&Client::JZIOTLOGIN_URL)?
            .send()
            .await?
            ._doc()
            .await?
            .use_val(&Client::S_SESSION_USER_KEY)
            .map(|s| s.to_owned())
            .map_err(|_| crate::error::Error::LoginFailed)
    }
}
