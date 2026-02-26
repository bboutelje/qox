use crate::traits::real::Real;


pub trait Instrument {

}

pub trait OptionInstrument<T: Real>: Instrument {
    fn strike(&self) -> f64;
    fn is_call(&self) -> bool;
    fn time_to_expiry(&self) -> T;
}

pub trait FutureInstrument<T: Real>: Instrument {
    fn delivery_time(&self) -> T;
    fn forward_price(&self) -> T;
}