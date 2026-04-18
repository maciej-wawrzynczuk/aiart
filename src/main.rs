mod floats;
use crate::floats::Line;



fn main() {
    let l: Line<f32> = Line::new_random();
    println!("{l}")
}
