use chrono::NaiveDate;


#[derive(Debug, Clone, Copy)]
pub enum Tenor {
    Days(i32),
    Weeks(i32),
    Months(i32),
    Years(i32),
}

impl Tenor {
    pub fn advance(&self, from: NaiveDate) -> NaiveDate {
        match *self {
            Tenor::Days(n) => from + chrono::Duration::days(n as i64),
            Tenor::Weeks(n) => from + chrono::Duration::weeks(n as i64),
            Tenor::Months(n) => from + chrono::Months::new(n as u32),
            Tenor::Years(n) => from + chrono::Months::new((n * 12) as u32),
        }
    }
}