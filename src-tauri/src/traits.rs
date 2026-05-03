use rand::rng;
use rand::seq::SliceRandom;
use std::collections::VecDeque;

pub trait Shuffle {
    fn shuffle(&mut self);
}

impl<T> Shuffle for VecDeque<T> {
    fn shuffle(&mut self) {
        let mut rng = rng();
        let slice = self.make_contiguous();

        slice.shuffle(&mut rng);
    }
}
