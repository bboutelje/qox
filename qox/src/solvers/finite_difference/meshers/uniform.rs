use crate::{
    solvers::finite_difference::meshers::Mesher1d, traits::transform::Transform, types::Real,
};

pub struct UniformMesher1d<T: Real, Tr: Transform<T>> {
    pub transform: Tr,
    pub centers: Vec<T>,
    pub h_plus: Vec<T>,
    pub h_minus: Vec<T>,
}

impl<T: Real, Tr: Transform<T>> UniformMesher1d<T, Tr> {
    pub fn new(start: T, end: T, size: usize, transform: Tr) -> Self {
        let n_minus_1 = T::from_f64((size - 1) as f64);
        let dx = (end - start) / n_minus_1;

        let centers: Vec<T> = (0..size)
            .map(|i| start + (T::from_f64(i as f64) * dx))
            .collect();

        let (h_plus, h_minus) = Self::build_distances(&centers);

        Self {
            transform,
            centers,
            h_plus,
            h_minus,
        }
    }

    fn build_distances(centers: &[T]) -> (Vec<T>, Vec<T>) {
        let n = centers.len();
        let mut hp = vec![T::zero(); n];
        let mut hm = vec![T::zero(); n];
        for (i, window) in centers.windows(2).enumerate() {
            let diff = window[1] - window[0];
            hp[i] = diff;
            hm[i + 1] = diff;
        }
        (hp, hm)
    }
}

impl<T: Real, Tr: Transform<T>> Mesher1d<T> for UniformMesher1d<T, Tr> {
    fn size(&self) -> usize {
        self.centers.len()
    }
    fn centers(&self) -> &[T] {
        &self.centers
    }
    fn h_plus(&self) -> &[T] {
        &self.h_plus
    }
    fn h_minus(&self) -> &[T] {
        &self.h_minus
    }

    // Restore the mapping here
    fn location(&self, index: usize) -> T {
        self.transform.to_physical(self.centers[index])
    }
}
