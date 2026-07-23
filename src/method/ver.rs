use crate::{
    Client,
    error::R,
    utils::{ToHtml, UseVer},
};

impl Client {
    pub async fn ver(&self) -> R<Option<String>> {
        Ok(self
            .get(&Client::LOGIN_URL)?
            .send()
            .await?
            ._doc()
            .await?
            .use_ver())
    }
}
