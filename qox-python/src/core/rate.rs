use pyo3::prelude::*;
use qox::core::{period::DayCountConvention, rate::{Compounding, Frequency, InterestRate}};
use qox::core::rate::Discountable;
use crate::core::{compounding::PyCompounding, day_count::PyDayCountConvention, frequency::PyFrequency};

#[pyclass(name = "InterestRate")]
pub struct PyInterestRate {
    inner: InterestRate<'static, f64>,
}

#[pymethods]
impl PyInterestRate {
    #[new]
    fn new(
        rate: f64,
        dcc: PyDayCountConvention,
        compounding: PyCompounding,
        frequency: PyFrequency,
    ) -> Self {
        // Map your Python wrappers into the Core types
        let core_dcc: DayCountConvention = dcc.into();
        let core_compounding: Compounding = compounding.into();
        let core_frequency: Frequency = frequency.into();

        Self {
            inner: InterestRate::new(rate, core_dcc, core_compounding, core_frequency),
        }
    }

    fn discount_factor(&self, t: f64) -> f64 {
        self.inner.discount_factor(&t)
    }

    // Property to inspect the rate
    // #[getter]
    // fn rate(&self) -> f64 {
    //     self.inner.implied_rate()
    // }
}