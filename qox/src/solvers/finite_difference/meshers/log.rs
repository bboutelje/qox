use crate::{solvers::finite_difference::meshers::Mesher1d, types::Real};

pub struct LogMesher1d<T: Real, M: Mesher1d<T>> {
    underlying: M,
    exp_centers: Vec<T>,
}

impl<T: Real, M: Mesher1d<T>> LogMesher1d<T, M> {
    pub fn new(mesher: M) -> Self {
        let exp_centers = mesher.centers().iter().map(|x| x.exp()).collect();

        Self {
            underlying: mesher,
            exp_centers,
        }
    }
}

impl<T: Real, M: Mesher1d<T>> Mesher1d<T> for LogMesher1d<T, M> {
    fn size(&self) -> usize {
        self.underlying.size()
    }
    fn centers(&self) -> &[T] {
        self.underlying.centers()
    }
    fn h_plus(&self) -> &[T] {
        self.underlying.h_plus()
    }
    fn h_minus(&self) -> &[T] {
        self.underlying.h_minus()
    }

    fn location(&self, index: usize) -> T {
        self.exp_centers[index]
    }
}
