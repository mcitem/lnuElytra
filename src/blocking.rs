use tokio::runtime::Runtime;

use crate::{Course, SelectCourseResponse, error::R};

pub struct Client {
    client: crate::Client,
    runtime: Runtime,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: crate::Client::new(),
            runtime: Runtime::new().unwrap(),
        }
    }

    pub fn new_with_base(backend: &str) -> R<Self> {
        Ok(Self {
            client: crate::Client::new_with_base(backend.parse()?),
            runtime: Runtime::new().unwrap(),
        })
    }

    pub fn login(&mut self, username: &str, password: &str) -> R<String> {
        self.runtime.block_on(self.client.login(username, password))
    }

    pub fn check_login(&self) -> R<String> {
        self.runtime.block_on(self.client.check_login())
    }

    pub fn init(&mut self) -> R {
        self.runtime.block_on(self.client.init())
    }

    pub fn fetch_course(&self, q: &str) -> R<Course> {
        self.runtime.block_on(self.client.fetch_courses(q))
    }

    pub fn select_course(&self, course_id: &str, course_do_id: &str) -> R<SelectCourseResponse> {
        self.runtime
            .block_on(self.client.select_course(course_id, course_do_id))
    }

    pub fn ver(&self) -> R<Option<String>> {
        self.runtime.block_on(self.client.ver())
    }
}

#[cfg(feature = "reqwest_cookie_store")]
impl Client {
    pub fn insert_cookie(&self, cookie: &str) -> R {
        self.client.insert_cookie(cookie)
    }

    pub fn clear_cookie(&self) {
        self.client.clear_cookie();
    }
}

impl Course {
    pub fn try_select_0_blocking(&self, client: &Client) -> R<SelectCourseResponse> {
        client.runtime.block_on(self.try_select_0(&client.client))
    }

    pub fn try_select_by_time_blocking(&self, client: &Client, q: &str) -> R<SelectCourseResponse> {
        client
            .runtime
            .block_on(self.try_select_by_time(&client.client, q))
    }
}
