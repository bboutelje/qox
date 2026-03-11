use chrono::NaiveDate;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use qox::{core::{period::DefaultPeriodCalculator, tenor::Tenor}, market::vol_surface::{FlatVolSurface, InterpolatedVolSurface}};
use qox::traits::vol_surface::VolSurface;
use crate::core::tenor::PyTenor;

#[derive(Clone)]
pub enum VolSurfaceEnum {
    Flat(FlatVolSurface<f64>),
    Interpolated(InterpolatedVolSurface<f64>),
}

#[pyclass(name = "VolSurface")]
#[derive(Clone)]
pub struct PyVolSurface {
    pub inner: VolSurfaceEnum,
}

#[pymethods]
impl PyVolSurface {
    #[staticmethod]
    pub fn flat(vol: f64) -> Self {
        Self { inner: VolSurfaceEnum::Flat(FlatVolSurface::new(vol)) }
    }

    #[staticmethod]
    pub fn interpolated(
        reference_date: NaiveDate,
        tenors: Vec<PyTenor>,
        vols: Vec<f64>,
    ) -> PyResult<Self> {
        let rust_tenors: Vec<Tenor> = tenors.into_iter().map(|t| t.inner).collect();
        let calculator = DefaultPeriodCalculator;

        let surface = InterpolatedVolSurface::new(reference_date, rust_tenors, vols, &calculator)
            .map_err(|e| PyValueError::new_err(format!("Surface Error: {:?}", e)))?;

        Ok(Self { inner: VolSurfaceEnum::Interpolated(surface) })
    }

    pub fn volatility(&self, strike: f64, t: f64) -> f64 {
        match &self.inner {
            VolSurfaceEnum::Flat(s) => s.volatility(strike, t),
            VolSurfaceEnum::Interpolated(s) => s.volatility(strike, t),
        }
    }
}

impl VolSurface<f64> for VolSurfaceEnum {
    fn volatility(&self, strike: f64, t: f64) -> f64 {
        match self {
            VolSurfaceEnum::Flat(s) => s.volatility(strike, t),
            VolSurfaceEnum::Interpolated(s) => s.volatility(strike, t),
        }
    }
}