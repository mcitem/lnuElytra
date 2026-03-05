use serde::{Deserialize, Serialize};

use crate::{
    Client,
    error::{Error, R},
};

impl Client {
    /// 选课接口
    pub async fn select_course(
        &self,
        course_id: &str,
        course_do_id: &str,
    ) -> R<SelectCourseResponse> {
        let xh = self
            .stores
            .get("xh_id")
            .ok_or(Error::MissingField("xh_id"))?;

        if xh.len() < 8 {
            return Err(Error::InvalidXhId);
        }

        #[derive(Serialize, Debug)]
        struct SelectCouresData<'a> {
            // 选课需要的参数
            jxb_ids: &'a str,
            kch_id: &'a str,
            qz: &'a str, // 0 定值
            njdm_id: &'a str,
            zyh_id: &'a str,
        }

        let res = self
            .post(&Client::SELECT_COURSE_URL)
            .form(&SelectCouresData {
                jxb_ids: course_do_id,
                kch_id: course_id,
                qz: "0",
                njdm_id: &xh[0..4],
                zyh_id: &xh[4..8],
            })
            .send()
            .await?;

        let res = res.json::<SelectCourseResponse>().await?;

        Ok(res)
    }
}

/// { flag: "1", msg: None }
///
/// { flag: "0", msg: Some("对不起，当前未开放选课！") }
///
/// { flag: "0", msg: Some("选课频率过高，请稍后重试！") }
///
/// { flag: "0", msg: Some("一门课程只能选一个教学班，不可再选！") }
///
/// { flag: "0", msg: Some("超过体育分项本学期本专业最高选课门次限制，不可选！") }
#[derive(Deserialize, Debug)]
pub struct SelectCourseResponse {
    flag: String,
    msg: Option<String>,
}

impl SelectCourseResponse {
    pub fn is_success(&self) -> bool {
        self.flag == "1"
    }

    pub fn msg(&self) -> Option<&str> {
        self.msg.as_deref()
    }
}
