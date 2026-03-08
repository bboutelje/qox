use pyo3::prelude::*;

use crate::core::{compounding::PyCompounding, day_count::PyDayCountConvention, frequency::PyFrequency, rate::PyInterestRate};
pub mod core;



#[pymodule]
fn qox(m: &Bound<'_, PyModule>) -> PyResult<()> {

    m.add_class::<PyCompounding>()?;
    m.add_class::<PyDayCountConvention>()?;
    m.add_class::<PyFrequency>()?;
    m.add_class::<PyInterestRate>()?;
    Ok(())
}

