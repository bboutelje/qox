use pyo3::prelude::*;
use qox::market::market_frame::OptionMarketFrame;

use crate::market::{rate_curve::{PyRateCurve, RateCurveEnum}, vol_surface::{PyVolSurface, VolSurfaceEnum}};

#[pyclass(name = "OptionMarketFrame")]
pub struct PyOptionMarketFrame {
    pub inner: OptionMarketFrame<f64, RateCurveEnum, VolSurfaceEnum>,
}

#[pymethods]
impl PyOptionMarketFrame {
    #[new]
    pub fn new(
        spot_price: f64, 
        rate_curve: Bound<'_, PyRateCurve>,
        vol_surface: Bound<'_, PyVolSurface>
    ) -> Self {
        Self {
            inner: OptionMarketFrame::new(
                spot_price,
                rate_curve.borrow().inner.clone(),
                vol_surface.borrow().inner.clone(),
            ),
        }
    }

    #[getter]
    pub fn spot_price(&self) -> f64 {
        self.inner.spot_price
    }
}