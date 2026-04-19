use core::fmt;

use anyhow::{Context, Result, ensure};
use num_traits::{Float, NumCast};

use crate::floats::Line;

pub struct Svg {
    c: CanvasSize,
    lines: Vec<SVGLine>,
}

impl Svg {
    pub fn new<F: Float>(x: u16, y: u16, ls: &[Line<F>]) -> Result<Self> {
        let c = CanvasSize { x, y };
        let lines: Vec<SVGLine> = ls
            .iter()
            .map(|l| SVGLine::from_line(l, &c))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { c, lines })
    }

    pub fn new4<F: Float>(x: u16, y: u16, s: &[F]) -> Result<Self> {
        let c = CanvasSize { x, y };
        let lines: Vec<SVGLine> = s.chunks_exact(4)
            .map(|x| SVGLine::from_4(x, &c))
            .collect::<Result<Vec<_>>>()?;
        
        Ok(Self {c, lines })
    }
}

impl fmt::Display for Svg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
            self.c.x, self.c.y
        )?;
        for l in &self.lines {
            writeln!(f, "{l}")?
        }
        writeln!(f, "</svg>")
    }
}

pub(crate) struct SVGLine {
    pub(crate) x1: u16,
    pub(crate) x2: u16,
    pub(crate) y1: u16,
    pub(crate) y2: u16,
}

struct CanvasSize {
    x: u16,
    y: u16,
}

impl SVGLine {
    fn from_line<T: Float>(l: &Line<T>, c: &CanvasSize) -> Result<Self> {
        Ok(Self {
            x1: f2canvas(l.start.x, c.x)?,
            y1: f2canvas(l.start.y, c.y)?,
            x2: f2canvas(l.end.x, c.x)?,
            y2: f2canvas(l.end.y, c.y)?,
        })
    }

    fn from_4<T: Float>(s: &[T], c: &CanvasSize) -> Result<Self> {
        ensure!(s.len() == 4, "Not enough data");
        let x1 = f2canvas(s[0], c.x)?;
        let y1 = f2canvas(s[1], c.y)?;
        let x2 = f2canvas(s[2], c.x)?;
        let y2 = f2canvas(s[3], c.y)?;

        Ok(Self { x1, y1, x2, y2})
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
mod test {
    use crate::Line;
    use crate::floats::Point;
    use crate::svg::{CanvasSize, SVGLine, Svg, f2canvas};
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
        let l = SVGLine {
            x1: 0,
            y1: 2,
            x2: 1,
            y2: 3,
        };
        let expected =
            "<line x1=\"0\" y1=\"2\" x2=\"1\" y2=\"3\" stroke=\"black\" stroke-width=\"2\"/>"
                .to_string();
        let result = l.to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn line_from_4() {
        let c = CanvasSize { x: 100, y: 100};
        let data: Vec<f32> = vec! [0.1, 0.2, 0.3, 0.4];
        let sut = SVGLine::from_4(&data, &c).unwrap();
        
        assert_eq!(sut.x1, 10);
        assert_eq!(sut.y1, 20);
        assert_eq!(sut.x2, 30);
        assert_eq!(sut.y2, 40);
    }

    #[test]
    fn svg_empty() {
        let s = Svg::new(10, 20, &[] as &[Line<f32>]).unwrap();
        let expected = indoc! {"
            <svg viewBox=\"0 0 10 20\" xmlns=\"http://www.w3.org/2000/svg\">
            </svg>
        "};
        assert_eq!(s.to_string(), expected.to_string());
    }

    #[test]
    fn svg_one() {
        let ls: Vec<Line<f32>> = vec![Line {
            start: Point { x: 0.25, y: 0.25 },
            end: Point { x: 0.75, y: 0.75 },
        }];
        let s = Svg::new(100, 100, &ls).unwrap();
        let expected = indoc! {"
            <svg viewBox=\"0 0 100 100\" xmlns=\"http://www.w3.org/2000/svg\">
            <line x1=\"25\" y1=\"25\" x2=\"75\" y2=\"75\" stroke=\"black\" stroke-width=\"2\"/>
            </svg>
        "};
        assert_eq!(s.to_string(), expected.to_string());
    }
}
