// use crate::traits::real::Real;

// pub struct Concentrating1dMesher<T: Real> {
//     centers: Vec<T>,
//     h_plus: Vec<T>,
//     h_minus: Vec<T>,
// }

// impl<T: Real> Concentrating1dMesher<T> {
//     pub fn new(start: T, end: T, size: usize, c_point: T, density: T) -> Self {
//         let mut centers = Vec::with_capacity(size);
        
//         // 1. Calculate the mapping limits using T's math methods
//         let asinh_min = ((start.clone() - c_point.clone()) / density.clone()).asinh();
//         let asinh_max = ((end.clone() - c_point.clone()) / density.clone()).asinh();
        
//         for i in 0..size {
//             let z = T::from_f64(i as f64) / T::from_f64((size - 1) as f64);
//             let transformed = c_point.clone() + density.clone() * (asinh_min.clone() + z * (asinh_max.clone() - asinh_min.clone())).sinh();
//             centers.push(transformed);
//         }

//         // 2. Build h_plus and h_minus from the new centers...
//         let (h_plus, h_minus) = build_distances(&centers);
//         Self { centers, h_plus, h_minus }
//     }
// }