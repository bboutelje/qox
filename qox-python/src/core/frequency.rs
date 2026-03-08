use pyo3::prelude::*;
use qox::core::rate::Frequency as CoreFrequency;

#[pyclass(name="Frequency")]
#[derive(Debug, Clone, Copy)]
pub enum PyFrequency {
    Annual,
    SemiAnnual,
    Quarterly,
    Monthly,
    Once,
    Infinite,
}

impl From<PyFrequency> for CoreFrequency {
    fn from(py_f: PyFrequency) -> Self {
        match py_f {
            PyFrequency::Annual => CoreFrequency::Annual,
            PyFrequency::SemiAnnual => CoreFrequency::SemiAnnual,
            PyFrequency::Quarterly => CoreFrequency::Quarterly,
            PyFrequency::Monthly => CoreFrequency::Monthly,
            PyFrequency::Once => CoreFrequency::Once,
            PyFrequency::Infinite => CoreFrequency::Infinite,
        }
    }
}

#[pymethods]
impl PyFrequency {
    // Expose the integer value to Python
    #[getter]
    fn value(&self) -> Option<i32> {
        
        match self {
            PyFrequency::Annual => Some(1),
            PyFrequency::SemiAnnual => Some(2),
            PyFrequency::Quarterly => Some(4),
            PyFrequency::Monthly => Some(12),
            _ => None,
        }
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }

    // #[staticmethod]
    // fn from_int(value: i32) -> Option<PyFrequency> {
    //     match value {
    //         1 => Some(PyFrequency::Annual),
    //         2 => Some(PyFrequency::SemiAnnual),
    //         4 => Some(PyFrequency::Quarterly),
    //         12 => Some(PyFrequency::Monthly),
    //         0 => Some(PyFrequency::Once),
    //         -1 => Some(PyFrequency::Infinite),
    //         _ => None,
    //     }
    // }
}