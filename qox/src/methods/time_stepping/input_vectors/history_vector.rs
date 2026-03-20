use crate::{methods::time_stepping::input_vectors::InputVector, types::Real};

pub struct HistoryVector<T> {
    pub items: Vec<T>, // Flat vector of size r * n
    pub r: usize,
    pub n: usize,
    pub current_time: T,
}

impl<T: Real> InputVector<T> for HistoryVector<T> {
    fn r(&self) -> usize {
        self.r
    }
    fn n(&self) -> usize {
        self.n
    }

    fn get_current_time(&self) -> T {
        self.current_time
    }
    fn set_current_time(&mut self, time: T) {
        self.current_time = time;
    }

    fn get_items(&self) -> &[T] {
        &self.items
    }
    fn get_items_mut(&mut self) -> &mut [T] {
        &mut self.items
    }
}
