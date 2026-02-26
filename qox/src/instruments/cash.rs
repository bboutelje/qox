// use chrono::{DateTime, Utc, NaiveDate};
// use std::sync::Arc;
// use crate::conventions::{BusinessDayConvention, DayCountBasis, DayCountMethod};
// use crate::rate::{Compounding, Frequency, InterestRate};
// use crate::period::{DefaultPeriodCalculator, PeriodCalculator};
// use crate::calendar::Calendar;
// use crate::tenor::Tenor;

// pub struct MarketQuote {
//     pub value: f64,
//     pub timestamp: DateTime<Utc>,
//     pub source: String,
// }

// pub struct CashDeposit<'a> {
//     pub start_date: NaiveDate,
//     pub maturity_date: NaiveDate,
//     pub quote: Arc<MarketQuote>,
//     pub basis: DayCountBasis<'a>, 
// }

// impl<'a> CashDeposit<'a> {
//     pub fn discount_factor(&self, period_calculator: &DefaultPeriodCalculator) -> f64 {
//         let method = DayCountMethod::Standard(self.basis);
//         let tau = period_calculator.year_fraction(self.start_date, self.maturity_date, method).0;
        
//         let rate = InterestRate {
//             value: self.quote.value,
//             method,
//             compounding: Compounding::Simple,
//             frequency: Frequency::Once,
//         };

//         rate.discount_factor(tau)
//     }
// }

// pub struct CashQuote {
//     pub period: Tenor,
//     pub quote: Arc<MarketQuote>,
// }

// impl CashQuote {
//     pub fn resolve<'a>(
//         &self, 
//         evaluation_date: NaiveDate, 
//         spot_lag: i32, 
//         calendar: &'a dyn Calendar,
//         convention: BusinessDayConvention,
//         basis: DayCountBasis<'a>,
//         _is_eom: bool,
//     ) -> CashDeposit<'a> {

//         let spot_date = calendar.advance_business_days(evaluation_date, spot_lag);

//         let maturity_date = calendar.advance_period(
//             spot_date, 
//             self.period, 
//             convention,
//             _is_eom
//         );

//         CashDeposit {
//             start_date: spot_date,
//             maturity_date,
//             quote: Arc::clone(&self.quote),
//             basis: basis,
//         }
//     }
// }