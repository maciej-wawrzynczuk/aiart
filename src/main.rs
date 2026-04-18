mod floats;
mod svg;

use crate::floats::Line;
use crate::svg::Svg;
use anyhow::Result;

fn main() -> Result<()> {
    let ls: Vec<Line<f32>> = (0..100).map(|_| Line::new_random()).collect();
    let s = Svg::new(640, 480, &ls)?;

    println!("{s}");
    Ok(())
}
