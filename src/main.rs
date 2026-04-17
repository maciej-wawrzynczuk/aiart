use rand::{distr::uniform::SampleUniform, random_range};
use std::fmt;



#[derive(Debug)]
struct Line<T> {
    x0: T,
    y0: T,
    x1: T,
    y1: T
}


fn main() {
    let l: Line<f32> = Line::new_random();
    println!("{l}")
}

impl<T: num_traits::Float + SampleUniform> Line<T> {
    fn new_random() -> Self {
        Self {
            x0: random_range(T::zero()..T::one()),
            y0: random_range(T::zero()..T::one()),
            x1: random_range(T::zero()..T::one()),
            y1: random_range(T::zero()..T::one()),
        }
    } 
}

impl<T: fmt::Display> fmt::Display for Line<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{}->{},{}", self.x0, self.y0, self.x1, self.y1)
    }
}