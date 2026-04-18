mod floats;
use core::fmt;

use num_traits::{Float, NumCast};
use anyhow::{Context, Result};

use crate::floats::Line;

#[derive(Debug)]
struct SVGLine {
    x1: u16,
    x2: u16,
    y1: u16,
    y2: u16,
}

struct CanvasSize {
    x: u16,
    y: u16,
}

impl SVGLine {
    fn from_line<T: Float>(l: Line<T>, c: CanvasSize) -> Result<Self> {
        Ok(Self {
            x1: f2canvas(l.start.x, c.x)?,
            y1: f2canvas(l.start.y, c.y)?,
            x2: f2canvas(l.end.x, c.x)?,
            y2: f2canvas(l.end.y, c.y)?,
        })
    }
}

impl fmt::Display for SVGLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"currentColor\" stroke-width=\"2\"/>",
            self.x1, self.y1, self.x2, self.y2
            )
    }
}

fn f2canvas<F: Float, I: NumCast>(f: F, i: I) -> Result<I> {
        let i_as_f: F = NumCast::from(i).context("Cast error")?;
        NumCast::from((f* i_as_f).round()).context("Cast error")
}

fn main() -> Result<()> {
    let l: Line<f32> = Line::new_random();
    let c = CanvasSize { x: 640, y: 480 };
    let cl = SVGLine::from_line(l, c)?;
    println!("{cl}");
    Ok(())
}
