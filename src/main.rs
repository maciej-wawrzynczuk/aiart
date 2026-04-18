mod floats;
mod svg;

use crate::floats::Line;
use crate::svg::{CanvasSize, SVGLine, Svg};
use anyhow::Result;

fn main() -> Result<()> {
    let l: Line<f32> = Line::new_random();
    let c = CanvasSize { x: 640, y: 480 };
    let cl = SVGLine::from_line(l, c)?;
    let mut s = Svg::new(640, 480);
    s.add_line(cl);
    println!("{s}");
    Ok(())
}
