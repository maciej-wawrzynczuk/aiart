use rand::random_range;
use std::fmt;

#[derive(Debug)]
struct SimpleLine {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32
}

fn main() {
    let l = SimpleLine::new_random();
    println!("{l}")
}

impl SimpleLine {
    fn new_random() -> Self {
        Self {
            x0: random_range(0.0..1.0),
            y0: random_range(0.0..1.0),
            x1: random_range(0.0..1.0),
            y1: random_range(0.0..1.0),
        }
    } 
}

impl fmt::Display for SimpleLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{}->{},{}", self.x0, self.y0, self.x1, self.y1)
    }
}