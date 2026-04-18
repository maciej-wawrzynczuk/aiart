use core::fmt;

use anyhow::{Context, Result};
use num_traits::{Float, NumCast};

use crate::floats::Line;

pub struct Svg {
    c: CanvasSize,
    lines: Vec<SVGLine>,
}

impl Svg {
    pub fn new(x: u16, y:u16) -> Self {
        Self {
            c: CanvasSize { x, y },
            lines: Vec::new()
        }
    }

    pub fn add_line(&mut self, l: SVGLine) {
        self.lines.push(l);
    }
}

impl fmt::Display for Svg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
            self.c.x, self.c.y
        )?;
        for l in &self.lines {
            writeln!(f, "{l}")?
        }
        writeln!(f, "</svg>")
    }
}

pub struct SVGLine {
    x1: u16,
    x2: u16,
    y1: u16,
    y2: u16,
}

pub struct CanvasSize {
    pub x: u16,
    pub y: u16,
}

impl SVGLine {
    pub fn new(x1: u16, x2: u16, y1: u16, y2: u16) -> Self {
        Self {x1, x2, y1, y2}
    }
    pub fn from_line<T: Float>(l: Line<T>, c: CanvasSize) -> Result<Self> {
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
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\" stroke-width=\"2\"/>",
            self.x1, self.y1, self.x2, self.y2
        )
    }
}

fn f2canvas<F: Float, I: NumCast>(f: F, i: I) -> Result<I> {
    let i_as_f: F = NumCast::from(i).context("Cast error")?;
    NumCast::from((f * i_as_f).round()).context("Cast error")
}


#[cfg(test)]
mod test{
    use crate::svg::{Svg, SVGLine, f2canvas};
    use indoc::indoc;

    #[test]
    fn f2c_boundaries() {
        assert_eq!(f2canvas(0.0, 100).unwrap(), 0);
        assert_eq!(f2canvas(1.0, 100).unwrap(), 100);
    }

    #[test]
    fn f2c_mid() {
        assert_eq!(f2canvas(0.5, 100).unwrap(), 50);
    }

    #[test]
    fn display_line() {
        let l = SVGLine::new(0, 1, 2, 3);
        let expected = "<line x1=\"0\" y1=\"2\" x2=\"1\" y2=\"3\" stroke=\"black\" stroke-width=\"2\"/>"
            .to_string();
        let result = l.to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn svg_empty() {
        let s = Svg::new(10, 20);
        let expected = indoc! {"
            <svg viewBox=\"0 0 10 20\" xmlns=\"http://www.w3.org/2000/svg\">
            </svg>
        "};
        assert_eq!(s.to_string(), expected.to_string());
    }

        #[test]
    fn svg_one() {
        let mut s = Svg::new(10, 20);
        let l = SVGLine::new(12, 56, 34, 67);
        s.add_line(l);
        let expected = indoc! {"
            <svg viewBox=\"0 0 10 20\" xmlns=\"http://www.w3.org/2000/svg\">
            <line x1=\"12\" y1=\"34\" x2=\"56\" y2=\"67\" stroke=\"black\" stroke-width=\"2\"/>
            </svg>
        "};
        assert_eq!(s.to_string(), expected.to_string());
    }
}
