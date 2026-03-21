use crate::types::Real;

pub fn black_scholes<T: Real>(s: T, k: T, t: T, r: T, sigma: T, is_call: bool) -> T {
    let zero = T::zero();
    let half = T::from_f64(0.5);
    let one = T::one();

    if t <= zero {
        return if is_call { s.max(k) - k } else { k.max(s) - s };
    }

    let sigma_sq_half = half * sigma.powi(2);
    let numerator = (s / k).ln() + (r + sigma_sq_half) * t;
    let denominator = sigma * t.sqrt();

    let d1 = numerator / denominator;
    let d2 = d1 - denominator;

    let ert = (-r * t).exp();

    if is_call {
        s * d1.norm_cdf() - k * ert * d2.norm_cdf()
    } else {
        let nd1 = (-d1).norm_cdf();
        let nd2 = (-d2).norm_cdf();
        k * ert * nd2 - s * nd1
    }
}

pub fn black_scholes_theta<T: Real>(s: T, k: T, t: T, r: T, sigma: T, is_call: bool) -> T {
    let zero = T::zero();
    let half = T::from_f64(0.5);
    let one = T::one();
    let two = T::from_f64(2.0);
    let pi = T::from_f64(std::f64::consts::PI);

    if t <= zero {
        return zero;
    }

    let sigma_sq = sigma.powi(2);
    let sqrt_t = t.sqrt();

    let d1 = ((s / k).ln() + (r + half * sigma_sq) * t) / (sigma * sqrt_t);
    let d2 = d1 - sigma * sqrt_t;

    let nd1_prime = (-(d1.powi(2)) / two).exp() / (two * pi).sqrt();
    let ert = (-r * t).exp();

    let term1 = -(s * nd1_prime * sigma) / (two * sqrt_t);
    let term2 = r * k * ert * d2.norm_cdf();
    let term3 = r * k * ert * (-d2).norm_cdf();

    if is_call {
        term1 - term2
    } else {
        term1 + term3
    }
}
