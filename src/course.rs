use serde::Serialize;

use crate::method::SelectCourseResponse;
use crate::{
    Client,
    error::{Error, R},
};

#[derive(Debug, Serialize)]
pub struct Jxb {
    pub jxb_id: String,
    pub do_id: String,
    pub jsxx: String,
    pub sksj: String,
}

#[derive(Debug, Serialize)]
pub struct Course {
    pub xkkz_id: String,
    pub kch_id: String,
    pub jxb: Vec<Jxb>,
}

impl Course {
    pub async fn try_select_0(&self, i: &Client) -> R<SelectCourseResponse> {
        let do_id = &self
            .jxb
            .get(0)
            .ok_or(Error::JxbNotFound("try_select_0"))?
            .do_id;

        i.select_course(&self.kch_id, do_id).await
    }
}

impl Course {
    // 星期四第9-10节{9-16周}
    // 16周
    pub async fn try_select_by_time(&self, i: &Client, q: &str) -> R<SelectCourseResponse> {
        let coures_id = &self.kch_id;

        let do_id = &self
            .jxb
            .iter()
            .filter(|x| x.sksj.contains(q))
            .collect::<Vec<&Jxb>>();

        let do_id = &do_id
            .get(0)
            .ok_or(Error::JxbNotFound("try_select_by_time"))?
            .do_id;

        i.select_course(coures_id, do_id).await
    }
}
