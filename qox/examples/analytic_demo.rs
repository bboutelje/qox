use qox::methods::analytic::black_scholes::black_scholes_theta;

pub fn main() {
    let s: f64 = 95.0;
    let k: f64 = 100.0;
    let t: f64 = 1.0;
    let r: f64 = 0.05;
    let sigma: f64 = 0.2;

    let price: f64 = black_scholes_theta::<f64>(s, k, t, r, sigma, false);
    println!("{}", price);
}
