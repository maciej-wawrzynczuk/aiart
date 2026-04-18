use rand::{distr::uniform::SampleUniform, random_range};

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub struct Line<T> {
    pub start: Point<T>,
    pub end: Point<T>,
}

impl<T: num_traits::Float + SampleUniform> Point<T> {
    fn new_random() -> Self {
        Self {
            x: random_range(T::zero()..T::one()),
            y: random_range(T::zero()..T::one()),
        }
    }
}

impl<T: num_traits::Float + SampleUniform> Line<T> {
    pub fn new_random() -> Self {
        Self {
            start: Point::new_random(),
            end: Point::new_random(),
        }
    }
}
