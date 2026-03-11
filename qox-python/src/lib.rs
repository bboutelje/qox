use pyo3::prelude::*;

use crate::{core::{compounding::PyCompounding, day_count::PyDayCountConvention, frequency::PyFrequency, rate::PyInterestRate, tenor::PyTenor}, instruments::stock_option::PyStockOption, market::{market_frame::PyOptionMarketFrame, rate_curve::PyRateCurve, vol_surface::PyVolSurface}};
pub mod core;
pub mod market;
pub mod instruments;


#[pymodule]
fn qox(m: &Bound<'_, PyModule>) -> PyResult<()> {

    m.add_class::<PyCompounding>()?;
    m.add_class::<PyDayCountConvention>()?;
    m.add_class::<PyFrequency>()?;
    m.add_class::<PyInterestRate>()?;
    m.add_class::<PyStockOption>()?;
    m.add_class::<PyOptionMarketFrame>()?;
    m.add_class::<PyTenor>()?;
    m.add_class::<PyRateCurve>()?;
    m.add_class::<PyVolSurface>()?;

    
    
    Ok(())
}

