use serde::Serialize;

use crate::{
    Client,
    error::{Error, R},
    utils::{ToHtml, macros::info},
};

impl Client {
    /// 获取选课基本参数，每次选课只需要执行一次
    pub async fn init(&mut self) -> R {
        info!("正在获取选课基本参数...");

        let index_doc = self
            .get(&Client::SELECT_COURSE_HTML_URL)
            .send()
            .await?
            .doc()
            .await?;

        for item in index_doc.select(&Client::S_INPUT_HIDDENT) {
            let name = item.attr("name").unwrap_or("");
            let value = item.attr("value").unwrap_or("");
            self.store(name, value);
        }

        #[derive(Serialize, Debug)]
        struct DisplayRequestData<'a> {
            xkkz_id: &'a str, // N253512
            xszxzt: &'a str,  // 1
            kspage: &'a str,  // 1
        }

        let display_data = DisplayRequestData {
            xkkz_id: self.stores.get("firstXkkzId").ok_or(Error::NotyetStarted)?,
            xszxzt: "1".into(),
            kspage: "1".into(),
        };

        let display_doc = self
            .post(&Client::SELECT_COURSE_DISPLAY_URL)
            .form(&display_data)
            .send()
            .await?
            .doc()
            .await?;

        for item in display_doc.select(&Client::S_INPUT_HIDDENT) {
            let name = item.attr("name").unwrap_or("");
            let value = item.attr("value").unwrap_or("");
            self.store(name, value);
        }

        info!("选课基本参数获取成功");

        Ok(())
    }
}
