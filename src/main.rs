mod floats;
mod svg;

use crate::floats::Line;
use crate::svg::Svg;
use anyhow::Result;

fn main() -> Result<()> {
    let mut s = Svg::new(640, 480);
    let ls: Vec<Line<f32>> = (0..100)
        .map(|_| Line::new_random())
        .collect();

    s.add_float_lines(&ls)?;

    println!("{s}");
    Ok(())
}
