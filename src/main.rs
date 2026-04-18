use rand::{distr::uniform::SampleUniform, random_range};
use std::fmt;


struct Point<T> {
    x: T,
    y: T
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>
}


fn main() {
    let l: Line<f32> = Line::new_random();
    println!("{l}")
}

impl<T: num_traits::Float + SampleUniform> Point<T> {
    fn new_random() -> Self {
        Self {
            x: random_range(T::zero()..T::one()),
            y: random_range(T::zero()..T::one())
        }
    }
}

impl<T: fmt::Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl<T: num_traits::Float + SampleUniform> Line<T> {
    fn new_random() -> Self {
        Self {
            start: Point::new_random(),
            end: Point::new_random()
        }
    } 
}

impl<T: fmt::Display> fmt::Display for Line<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}->{})", self.start, self.end)
    }
}