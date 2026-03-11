use pyo3::prelude::*;
use chrono::NaiveDate;
use qox::core::tenor::Tenor;

#[pyclass(name = "Tenor")]
#[derive(Clone)]
pub struct PyTenor {
    pub inner: Tenor,
}

#[pymethods]
impl PyTenor {
    #[staticmethod]
    fn days(n: i32) -> Self { Self { inner: Tenor::Days(n) } }
    
    #[staticmethod]
    fn weeks(n: i32) -> Self { Self { inner: Tenor::Weeks(n) } }
    
    #[staticmethod]
    fn months(n: i32) -> Self { Self { inner: Tenor::Months(n) } }
    
    #[staticmethod]
    fn years(n: i32) -> Self { Self { inner: Tenor::Years(n) } }

    fn advance(&self, from: NaiveDate) -> NaiveDate {
        self.inner.advance(from)
    }
}


