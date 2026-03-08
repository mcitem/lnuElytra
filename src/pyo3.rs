use pyo3::prelude::*;

#[pymodule]
pub mod lnu_elytra {
    use crate::{Course, SelectCourseResponse, blocking, error::R};
    use pyo3::{exceptions::PyException, prelude::*};

    impl Into<PyErr> for crate::Error {
        fn into(self) -> PyErr {
            PyErr::new::<PyException, _>(format!("{:?}", self))
        }
    }

    #[cfg_attr(test, pyo3_stub_gen::derive::gen_stub_pyclass)]
    #[pyclass]
    pub struct Client(blocking::Client);

    #[cfg_attr(test, pyo3_stub_gen::derive::gen_stub_pymethods)]
    #[pymethods]
    impl Client {
        #[new]
        fn new() -> Self {
            Self(blocking::Client::new())
        }

        pub fn login(&mut self, username: &str, password: &str) -> R {
            self.0.login(username, password)
        }

        pub fn init(&mut self) -> R {
            self.0.init()
        }

        pub fn fetch_course(&self, q: &str) -> R<Course> {
            self.0.fetch_course(q)
        }

        pub fn select_course(
            &self,
            course_id: &str,
            course_do_id: &str,
        ) -> R<SelectCourseResponse> {
            self.0.select_course(course_id, course_do_id)
        }

        #[cfg(feature = "cookie_override")]
        pub fn set_cookie_override(&mut self, cookie: String) {
            self.0.set_cookie_override(cookie);
        }

        #[cfg(feature = "cookie_override")]
        pub fn clear_cookie_override(&mut self) {
            self.0.clear_cookie_override();
        }
    }

    #[cfg_attr(test, pyo3_stub_gen::derive::gen_stub_pymethods)]
    #[pymethods]
    impl Course {
        #[pyo3(name = "try_select_0")]
        pub fn try_select_0_py(&self, client: &Client) -> R<SelectCourseResponse> {
            self.try_select_0_blocking(&client.0)
        }

        #[pyo3(name = "try_select_by_time")]
        pub fn try_select_by_time_py(&self, client: &Client, q: &str) -> R<SelectCourseResponse> {
            self.try_select_by_time_blocking(&client.0, q)
        }
    }

    #[cfg(test)]
    pyo3_stub_gen::define_stub_info_gatherer!(stub_info);
}

#[test]
#[cfg(test)]
#[cfg(feature = "__pyo3")]
fn gen_stub_info() -> pyo3_stub_gen::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().filter_or("RUST_LOG", "info")).init();
    let stub = lnu_elytra::stub_info()?;
    stub.generate()?;
    Ok(())
}
