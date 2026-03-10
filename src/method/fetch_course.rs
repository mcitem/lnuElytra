use serde::{Deserialize, Serialize};

use crate::{
    Client,
    course::{Course, Jxb},
    error::{Error, R},
    utils::macros::{debug, info, trace, warn},
};

impl Client {
    /// 查询教学班参数，需要精确的搜索
    pub async fn fetch_courses(&self, q: &str) -> R<Course> {
        info!("查询课程，q: {}", q);

        #[derive(Serialize, Debug)]
        struct PartDisplayRequestData<'a> {
            #[serde(rename = "filter_list[0]")]
            filter_list: &'a str,
            xbm: &'a str,
            ccdm: &'a str,
            kklxdm: &'a str,
            xkxnm: &'a str, //选课学年：如 “2024”
            xkxqm: &'a str, //选课学期名，如第二学期为 “2”
            jg_id: &'a str, // 二级学院 id
            xsbj: &'a str,  //学生班级(推测)
            mzm: &'a str,
            xz: &'a str,      //可能是学制
            bh_id: &'a str,   //学生班级号id,值为完整学号去除后两位
            xqh_id: &'a str,  //学区号id(推测)
            zyfx_id: &'a str, //与专业有关
            xslbdm: &'a str,
            kspage: &'a str,
            jspage: &'a str,
        }

        let part_display_data = PartDisplayRequestData {
            filter_list: q.into(),
            xbm: self.use_store("xbm"),
            ccdm: self.use_store("ccdm"),
            kklxdm: self.use_store("firstKklxdm"),
            xkxnm: self.use_store("xkxnm"),
            xkxqm: self.use_store("xkxqm"),
            jg_id: self.use_store("jg_id_1"),
            xsbj: self.use_store("xsbj"),
            mzm: self.use_store("mzm"),
            xz: self.use_store("xz"),
            bh_id: self.use_store("bh_id"),
            xqh_id: self.use_store("xqh_id"),
            zyfx_id: self.use_store("zyfx_id"),
            xslbdm: self.use_store("xslbdm"),
            kspage: "1",
            jspage: "10",
        };

        #[derive(Deserialize, Debug)]
        struct PartDisplayResponseData {
            #[serde(rename = "tmpList")]
            tmp_list: Vec<PartDisplayResponseInnerData>,
        }

        impl PartDisplayResponseData {
            fn use_first_kch_id(&self) -> R<&String> {
                Ok(&self
                    .tmp_list
                    .get(0)
                    .ok_or(Error::JxbNotFound("~.tmp_list.0"))?
                    .kch_id)
            }
        }

        #[derive(Deserialize, Debug)]
        struct PartDisplayResponseInnerData {
            // jxb_id: String, //教学班id
            // jxbmc: String,  //教学班名称 (2024-2025-2)-40190004-25
            // kcmc: String,   //课程名称
            kch_id: String, //课程号
        }

        trace!("part_display request");

        let part_display_res = self
            .post(&Client::SELECT_COURSE_PART_DISPLAY_URL)
            .form(&part_display_data)
            .send()
            .await?
            .json::<PartDisplayResponseData>()
            .await?;

        debug!("part_display {:#?}", part_display_res);

        #[derive(Serialize, Debug)]
        struct QueryDoWithCouresIdRequestData<'a> {
            #[serde(rename = "filter_list[0]")]
            filter_list: &'a str,
            xkxqm: &'a str,
            xkxnm: &'a str,
            xkkz_id: &'a str,
            bklx_id: &'a str,
            kch_id: &'a str,  // 课程号
            njdm_id: &'a str, //年级名称
            xsbj: &'a str,
            xz: &'a str, //学制
            mzm: &'a str,
            kklxdm: &'a str,
            bh_id: &'a str,
            xqh_id: &'a str,
            xslbdm: &'a str,
            zyfx_id: &'a str,
            jg_id: &'a str,
            ccdm: &'a str,
            xbm: &'a str,
        }

        let query_do_data = QueryDoWithCouresIdRequestData {
            filter_list: q.into(),
            xkxqm: self.use_store("xkxqm"),
            xkxnm: self.use_store("xkxnm"),
            xkkz_id: self.use_store("firstXkkzId"),
            bklx_id: self.use_store("bklx_id"),
            kch_id: &part_display_res.use_first_kch_id()?,
            njdm_id: self.use_store("njdm_id"),
            xsbj: self.use_store("xsbj"),
            xz: self.use_store("xz"),
            mzm: self.use_store("mzm"),
            kklxdm: self.use_store("firstKklxdm"),
            bh_id: self.use_store("bh_id"),
            xqh_id: self.use_store("xqh_id"),
            xslbdm: self.use_store("xslbdm"),
            zyfx_id: self.use_store("zyfx_id"),
            jg_id: self.use_store("jg_id_1"),
            ccdm: self.use_store("ccdm"),
            xbm: self.use_store("xbm"),
        };

        // #[allow(unused)]
        #[derive(Deserialize, Debug)]
        struct SelectCourseQueryDoWithCourseIdResponseInnerData {
            //返回值是一个列表
            do_jxb_id: String, //执行id
            jsxx: String,      //教师信息
            jxb_id: String,    //教学班id
            // jxdd: String,      //教学地点
            sksj: String, //上课时间  星期四第9-10节{9-16周}
                          // xqumc: String,     //校区名称
        }

        trace!("query_do request");

        let query_do_res = self
            .post(&Client::SELECT_COURSE_QUERY_DO_WITH_COURSE_ID_URL)
            .form(&query_do_data)
            .send()
            .await?
            .json::<Vec<SelectCourseQueryDoWithCourseIdResponseInnerData>>()
            .await?;

        debug!("query_do {:#?}", query_do_res);

        let mut returndta = Course {
            xkkz_id: self
                .stores
                .get("firstXkkzId")
                .ok_or(Error::Missing("[firstXkkzId] when fetch_courses".into()))?
                .into(),

            kch_id: part_display_res.use_first_kch_id()?.into(),
            jxb: vec![],
        };

        for item in query_do_res {
            returndta.jxb.push(Jxb {
                jxb_id: item.jxb_id,
                do_id: item.do_jxb_id,
                jsxx: item.jsxx,
                sksj: item.sksj,
            });
        }

        if returndta.jxb.is_empty() {
            warn!("{} 查询教学班为空", q);
        }

        info!("获取课程信息成功");

        debug!("课程信息 {:#?}", returndta);

        Ok(returndta)
    }
}
