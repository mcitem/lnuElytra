use crate::{
    Client, def,
    error::R,
    utils::{ToHtml, UseVer},
};

impl Client {
    pub async fn ver(&self) -> R<Option<String>> {
        Ok(self
            .get(def::LOGIN_URL)?
            .send()
            .await?
            ._doc()
            .await?
            .use_ver())
    }
}
