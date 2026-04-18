mod floats;
mod svg;

use crate::floats::Line;
use crate::svg::Svg;
use anyhow::Result;

fn main() -> Result<()> {
    let mut s = Svg::new(640, 480);
    for _ in 1..1000 {
            let l: Line<f32> = Line::new_random();
            s.add_float_line(l)?;
     }
    
    println!("{s}");
    Ok(())
}
