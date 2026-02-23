#[derive(Debug, Default)]
pub struct Greeks {
    pub price: f64,
    pub delta: Option<f64>,
    pub gamma: Option<f64>,
    pub vega: Option<f64>,
    pub theta: Option<f64>,
}

pub enum GreekRequest {
    Price,
    FirstOrder
}

pub mod black;

