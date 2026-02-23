use crate::traits::real::Real;


pub trait Instrument<T: Real> {

}

pub trait OptionInstrument<T: Real>: Instrument<T> {
    fn strike(&self) -> T;
    fn is_call(&self) -> bool;
    fn time_to_expiry(&self) -> T;
}

pub trait FutureInstrument<T: Real>: Instrument<T> {
    fn delivery_time(&self) -> T;
    fn forward_price(&self) -> T;
}