use pyo3::prelude::*;
use qox::core::rate::Compounding;

#[pyclass(name="Compounding")]
#[derive(Debug, Clone, Copy)]
pub enum PyCompounding {
    Simple,
    Compounded,
    Continuous,
    SimpleThenCompounded,
}

impl From<PyCompounding> for Compounding {

    fn from(py_c: PyCompounding) -> Self {
        match py_c {
            PyCompounding::Simple => Compounding::Simple,
            PyCompounding::Compounded => Compounding::Compounded,
            PyCompounding::Continuous => Compounding::Continuous,
            PyCompounding::SimpleThenCompounded => Compounding::SimpleThenCompounded,
        }
    }
}

#[pymethods]
impl PyCompounding {
    // This allows Python to show meaningful representations
    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}