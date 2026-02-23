// use chrono::{DateTime, Utc};

// /// Trait for anything that can be used to build a yield curve
// pub trait CurveInstrument {
//     fn maturity_date(&self) -> DateTime<Utc>;
//     // You might add methods for par rate, market price, etc.
// }

// pub struct YieldCurve<T: CurveInstrument> {
//     pub evaluation_date: DateTime<Utc>,
//     pub instruments: Vec<T>,
// }

// impl<T: CurveInstrument> YieldCurve<T> {
//     pub fn new(evaluation_date: DateTime<Utc>, mut instruments: Vec<T>) -> Self {
//         // Sort by maturity using the trait method
//         instruments.sort_by_key(|inst| inst.maturity_date());
        
//         Self {
//             evaluation_date,
//             instruments,
//         }
//     }
// }