use chrono::NaiveDate;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use qox::core::{period::DefaultPeriodCalculator, rate::InterestRate, tenor::Tenor};
use qox::market::rate_curve::{ContinuousRateCurve, FlatRateCurve, InterpolatedRateCurve};
use qox::traits::rate_curve::RateCurve;

use crate::core::day_count::PyDayCountConvention;
use crate::core::{rate::PyInterestRate, tenor::PyTenor};

#[derive(Clone)]
pub enum RateCurveEnum {
    Flat(FlatRateCurve<'static, f64>),
    Interpolated(InterpolatedRateCurve<'static, f64>),
    Continuous(ContinuousRateCurve<'static, f64>),
}

impl RateCurve<f64> for RateCurveEnum {
    fn zero_rate(&self, t: f64) -> f64 {
        match self {
            RateCurveEnum::Flat(c) => c.zero_rate(t),
            RateCurveEnum::Interpolated(c) => c.zero_rate(t),
            RateCurveEnum::Continuous(c) => c.zero_rate(t),
        }
    }

    fn discount_factor(&self, t: f64) -> f64 {
        match self {
            RateCurveEnum::Flat(c) => c.discount_factor(t),
            RateCurveEnum::Interpolated(c) => c.discount_factor(t),
            RateCurveEnum::Continuous(c) => c.discount_factor(t),
        }
    }
}

#[pyclass(name = "RateCurve")]
#[derive(Clone)]
pub struct PyRateCurve {
    pub inner: RateCurveEnum,
}

#[pymethods]
impl PyRateCurve {
    #[staticmethod]
    pub fn flat(rate: &Bound<'_, PyInterestRate>) -> Self {
        let inner_rate = rate.borrow().inner.clone();
        Self {
            inner: RateCurveEnum::Flat(FlatRateCurve::new(inner_rate)),
        }
    }

    #[staticmethod]
    pub fn continuous(value: f64, day_count_convention: PyDayCountConvention) -> Self {
        Self {
            inner: RateCurveEnum::Continuous(ContinuousRateCurve::new(
                value,
                day_count_convention.into(),
            )),
        }
    }

    #[staticmethod]
    pub fn interpolated(
        reference_date: NaiveDate,
        tenors: Vec<PyTenor>,
        // Similarly update the rates argument
        rates: Vec<PyRef<'_, PyInterestRate>>,
    ) -> PyResult<Self> {
        let rust_tenors: Vec<Tenor> = tenors.into_iter().map(|t| t.inner).collect();
        // Access inner via the PyRef
        let rust_rates: Vec<InterestRate<'static, f64>> =
            rates.into_iter().map(|r| r.inner.clone()).collect();

        let calculator = DefaultPeriodCalculator;

        let curve =
            InterpolatedRateCurve::new(reference_date, rust_tenors, rust_rates, &calculator)
                .map_err(|e| PyValueError::new_err(format!("Curve Error: {:?}", e)))?;

        Ok(Self {
            inner: RateCurveEnum::Interpolated(curve),
        })
    }

    pub fn zero_rate(&self, t: f64) -> f64 {
        self.inner.zero_rate(t)
    }

    pub fn discount_factor(&self, t: f64) -> f64 {
        self.inner.discount_factor(t)
    }
}
