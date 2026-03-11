use pyo3::prelude::*;
use chrono::{DateTime, Utc};
use qox::evaluators::black_scholes::finite_difference::VanillaPayoff;
use qox::instruments::stock_option::StockOption;
use qox::traits::instrument::OptionType;
use crate::market::market_frame::PyOptionMarketFrame;
use qox::traits::instrument::OptionInstrument;

#[pyclass(name = "StockOption")]
#[derive(Clone)]
pub struct PyStockOption {
    pub inner: StockOption,
}

#[pymethods]
impl PyStockOption {
    #[new]
    pub fn new(strike: f64, expiry: DateTime<Utc>, option_type_str: &str) -> PyResult<Self> {
        let option_type = match option_type_str.to_lowercase().as_str() {
            "call" => OptionType::Call,
            "put" => OptionType::Put,
            _ => return Err(pyo3::exceptions::PyValueError::new_err("Invalid option type")),
        };

        Ok(PyStockOption {
            inner: StockOption::new(strike, expiry, option_type),
        })
    }

    #[getter]
    pub fn strike(&self) -> f64 {
        self.inner.strike
    }

    #[getter]
    pub fn years_to_expiry(&self) -> f64 {
        <StockOption as OptionInstrument<f64, VanillaPayoff>>::years_to_expiry(self.inner)
    }

    pub fn evaluate(&self, market_frame: &PyOptionMarketFrame) -> f64 {
        self.inner.evaluate(&market_frame.inner)
    }
}
