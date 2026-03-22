use crate::{methods::time_stepping::input_vectors::InputVector, types::Real};

pub struct NordsieckVector<T> {
    pub items: Vec<T>,
    pub r: usize,
    pub n: usize,
    pub current_time: T,
}

impl<T: Real> NordsieckVector<T> {
    pub(crate) fn new(r: usize, nodes: usize, time: T) -> Self {
        Self {
            items: vec![time; r * nodes],
            r,
            n: nodes,
            current_time: time,
        }
    }
}

impl<T: Real> InputVector<T> for NordsieckVector<T> {
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
