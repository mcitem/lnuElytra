use serde::{Deserialize, Serialize};

use crate::{
    Client, def,
    error::R,
    utils::{
        ToJson,
        macros::{debug, info, trace, warn},
    },
};

impl Client {
    /// 选课接口
    pub async fn select_course(
        &self,
        course_id: &str,
        course_do_id: &str,
    ) -> R<SelectCourseResponse> {
        info!("执行选课");

        // let xh = self
        //     .stores
        //     .get("xh_id")
        //     .ok_or(Error::MissingField("xh_id"))?;

        // if xh.len() < 8 {
        //     error!("xh_id {} 无法提取选课参数", xh);
        //     return Err(Error::InvalidXhId);
        // }

        #[derive(Serialize, Debug)]
        struct SelectCourseData<'a> {
            // 选课需要的参数
            jxb_ids: &'a str,
            kch_id: &'a str,
            qz: &'a str, // 0 定值
            // <input type="hidden" name="njdm_id" id="njdm_id" value="">
            njdm_id: &'a str,
            // <input type="hidden" name="zyh_id" id="zyh_id" value="">
            zyh_id: &'a str,
        }

        trace!("发送请求选课");

        // debug!("{},{}", self.use_store("njdm_id"), self.use_store("zyh_id"));

        let res = self
            .post(def::SELECT_COURSE_URL)?
            .form(&SelectCourseData {
                jxb_ids: course_do_id,
                kch_id: course_id,
                qz: "0",
                njdm_id: self.use_store("njdm_id"),
                zyh_id: self.use_store("zyh_id"),
            })
            .send()
            .await?;

        let res = res.jsonr::<SelectCourseResponse>().await?;

        if res.is_success() {
            info!("选课成功");
        } else {
            warn!("选课失败: {}", res.msg().unwrap_or("未知错误"));
        }

        debug!("选课结果: {:?}", res);

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
///
/// { flag: "0", msg: Some("超过通识选修课本学期本专业最高选课门次限制，不可选！") }
#[cfg_attr(
    feature = "__pyo3",
    cfg_attr(test, pyo3_stub_gen::derive::gen_stub_pyclass),
    pyo3::pyclass(get_all)
)]
#[derive(Deserialize, Debug)]
pub struct SelectCourseResponse {
    pub flag: String,
    pub msg: Option<String>,
}

impl SelectCourseResponse {
    pub fn is_success(&self) -> bool {
        self.flag == "1"
    }

    pub fn msg(&self) -> Option<&str> {
        self.msg.as_deref()
    }
}
