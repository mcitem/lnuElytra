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
        #[pyo3(signature = (base=None))]
        fn new(base: Option<String>) -> PyResult<Self> {
            Ok(Self(match base {
                None => blocking::Client::new(),
                Some(base) => {
                    let base: Result<_, PyErr> =
                        blocking::Client::new_with_base(&base).map_err(Into::into);
                    base?
                }
            }))
        }

        pub fn login(&mut self, username: &str, password: &str) -> R<String> {
            self.0.login(username, password)
        }

        pub fn check_login(&self) -> R<String> {
            self.0.check_login()
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

        pub fn ver(&self) -> R<Option<String>> {
            self.0.ver()
        }

        #[cfg(feature = "reqwest_cookie_store")]
        pub fn insert_cookie(&mut self, cookie: String) -> R {
            self.0.insert_cookie(&cookie)
        }

        #[cfg(feature = "reqwest_cookie_store")]
        pub fn clear_cookie(&mut self) {
            self.0.clear_cookie();
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

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        pyo3_tracing_subscriber::add_submodule("lnu_elytra", "tracing", m.py(), m)?;
        Ok(())
    }

    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[cfg_attr(test, pyo3_stub_gen::derive::gen_stub_pyclass)]
    #[pyclass]
    pub struct AsyncClient(Arc<RwLock<crate::Client>>);

    #[cfg_attr(test, pyo3_stub_gen::derive::gen_stub_pymethods)]
    #[pymethods]
    impl AsyncClient {
        #[new]
        #[pyo3(signature = (base=None))]
        fn new(base: Option<String>) -> PyResult<Self> {
            Ok(Self(Arc::new(RwLock::new(match base {
                None => crate::Client::new(),
                Some(base) => {
                    let base: PyResult<url::Url> = base
                        .parse()
                        .map_err(crate::Error::UrlParseError)
                        .map_err(Into::into);

                    crate::Client::new_with_base(base?)
                }
            }))))
        }
    }

    #[pymethods]
    impl AsyncClient {
        fn login<'a>(
            &self,
            py: Python<'a>,
            username: String,
            password: String,
        ) -> PyResult<Bound<'a, PyAny>> {
            let client = self.0.clone();
            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                let mut client = client.write().await;
                Ok(client
                    .login(&username, &password)
                    .await
                    .map_err::<PyErr, _>(Into::into)?)
            })
        }

        fn check_login<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
            let client = self.0.clone();
            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                let client = client.read().await;
                Ok(client.check_login().await.map_err::<PyErr, _>(Into::into)?)
            })
        }

        fn init<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
            let client = self.0.clone();
            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                let mut client = client.write().await;
                Ok(client.init().await.map_err::<PyErr, _>(Into::into)?)
            })
        }

        fn fetch_courses<'a>(&self, py: Python<'a>, q: &str) -> PyResult<Bound<'a, PyAny>> {
            let client = self.0.clone();
            let q = q.to_string();
            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                let client = client.read().await;
                Ok(client
                    .fetch_courses(&q)
                    .await
                    .map_err::<PyErr, _>(Into::into)?)
            })
        }

        fn select_course<'a>(
            &self,
            py: Python<'a>,
            course_id: String,
            course_do_id: String,
        ) -> PyResult<Bound<'a, PyAny>> {
            let client = self.0.clone();
            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                let client = client.read().await;
                Ok(client
                    .select_course(&course_id, &course_do_id)
                    .await
                    .map_err::<PyErr, _>(Into::into)?)
            })
        }

        fn ver<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
            let client = self.0.clone();
            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                let client = client.read().await;
                Ok(client.ver().await.map_err::<PyErr, _>(Into::into)?)
            })
        }

        #[cfg(feature = "reqwest_cookie_store")]
        fn insert_cookie<'a>(&self, py: Python<'a>, cookie: String) -> PyResult<Bound<'a, PyAny>> {
            let client = self.0.clone();
            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                let client = client.read().await;
                Ok(client
                    .insert_cookie(&cookie)
                    .map_err::<PyErr, _>(Into::into)?)
            })
        }

        #[cfg(feature = "reqwest_cookie_store")]
        fn clear_cookie<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
            let client = self.0.clone();
            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                let client = client.read().await;
                client.clear_cookie();
                Ok(())
            })
        }
    }

    #[cfg(test)]
    pyo3_stub_gen::inventory::submit! {
        pyo3_stub_gen::derive::gen_methods_from_python! {
            r#"
            class AsyncClient:
                def login(self, username: builtins.str, password: builtins.str) -> typing.Awaitable[builtins.str]: ...
                def check_login(self) -> typing.Awaitable[builtins.str]: ...
                def init(self) -> typing.Awaitable[None]: ...
                def fetch_courses(self, q: builtins.str) -> typing.Awaitable[Course]: ...
                def select_course(self, course_id: builtins.str, course_do_id: builtins.str) -> typing.Awaitable[SelectCourseResponse]: ...
                def ver(self) -> typing.Awaitable[typing.Optional[builtins.str]]: ...
            "#
        }
    }

    #[cfg(all(test, feature = "reqwest_cookie_store"))]
    pyo3_stub_gen::inventory::submit! {
        pyo3_stub_gen::derive::gen_methods_from_python! {
            r#"
            class AsyncClient:
                def insert_cookie(self, cookie: builtins.str) -> typing.Awaitable[None]: ...
                def clear_cookie(self) -> typing.Awaitable[None]: ...
            "#
        }
    }

    #[pymethods]
    impl Course {
        fn async_try_select_0<'a>(
            &self,
            py: Python<'a>,
            client: &AsyncClient,
        ) -> PyResult<Bound<'a, PyAny>> {
            let course = self.clone();
            let client = client.0.clone();
            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                let client = client.read().await;
                course
                    .try_select_0(&client)
                    .await
                    .map_err::<PyErr, _>(Into::into)?;
                Ok(())
            })
        }

        fn async_try_select_by_time<'a>(
            &self,
            py: Python<'a>,
            client: &AsyncClient,
            q: String,
        ) -> PyResult<Bound<'a, PyAny>> {
            let course = self.clone();
            let client = client.0.clone();
            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                let client = client.read().await;
                course
                    .try_select_by_time(&client, &q)
                    .await
                    .map_err::<PyErr, _>(Into::into)?;
                Ok(())
            })
        }
    }

    #[cfg(test)]
    pyo3_stub_gen::inventory::submit! {
        pyo3_stub_gen::derive::gen_methods_from_python! {
            r#"
            class Course:
                def async_try_select_0(self, client: AsyncClient) -> typing.Awaitable[SelectCourseResponse]: ...
                def async_try_select_by_time(self, client: AsyncClient, q: builtins.str) -> typing.Awaitable[SelectCourseResponse]: ...
            "#
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

    pyo3_tracing_subscriber_build::write_stub_files(
        "lnu_elytra",
        "tracing",
        &stub.python_root.join("lnu_elytra/tracing"),
    )?;

    Ok(())
}
