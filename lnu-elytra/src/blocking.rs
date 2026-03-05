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

    pub fn login(&mut self, username: &str, password: &str) -> R {
        self.runtime.block_on(self.client.login(username, password))
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
