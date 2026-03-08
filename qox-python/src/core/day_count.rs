use pyo3::prelude::*;
use qox::core::period::{DayCountConvention, Thirty360Subtype};


#[pyclass(name="DayCountConvention")]
#[derive(Debug, Clone, Copy)] // Added Copy since it's a simple enum variant
pub enum PyDayCountConvention {
    Actual360,
    Actual365Fixed,
    ActActISDA,
    Thirty360US,
}

impl From<PyDayCountConvention> for DayCountConvention<'static> {
    fn from(py_dcc: PyDayCountConvention) -> Self {
        match py_dcc {
            PyDayCountConvention::Actual360 => DayCountConvention::Actual360,
            PyDayCountConvention::Actual365Fixed => DayCountConvention::Actual365Fixed,
            PyDayCountConvention::ActActISDA => DayCountConvention::ActActISDA,
            PyDayCountConvention::Thirty360US => DayCountConvention::Thirty360(Thirty360Subtype::US),
        }
    }
}

#[pymethods]
impl PyDayCountConvention {
    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}