mod floats;
mod svg;

use anyhow::Result;
use crate::floats::Line;
use crate::svg::{CanvasSize, SVGLine};


fn main() -> Result<()> {
    let l: Line<f32> = Line::new_random();
    let c = CanvasSize { x: 640, y: 480 };
    let cl = SVGLine::from_line(l, c)?;
    println!("{cl}");
    Ok(())
}
