use core::fmt;

use anyhow::{Context, Result};
use num_traits::{Float, NumCast};

pub struct Svg {
    c: CanvasSize,
    lines: SvgLines,
}

pub(crate) struct SvgLines {
    pub(crate) x1: Vec<u16>,
    pub(crate) y1: Vec<u16>,
    pub(crate) x2: Vec<u16>,
    pub(crate) y2: Vec<u16>,
}

impl SvgLines {
    pub(crate) fn new<F: Float>(s: &[F], c: &CanvasSize) -> Result<Self> {
        let size = s.len() / 4;
        let mut result = SvgLines {
            x1: Vec::with_capacity(size),
            x2: Vec::with_capacity(size),
            y1: Vec::with_capacity(size),
            y2: Vec::with_capacity(size),
        };

        for batch in s.chunks_exact(4) {
            result.x1.push(f2canvas(batch[0], c.x)?);
            result.y1.push(f2canvas(batch[1], c.y)?);
            result.x2.push(f2canvas(batch[2], c.x)?);
            result.y2.push(f2canvas(batch[3], c.y)?);
        }

        Ok(result)
    }
}

impl fmt::Display for SvgLines {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for n in 0..self.x1.len() {
            writeln!(
                f,
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\" stroke-width=\"2\"/>",
                self.x1[n], self.y1[n], self.x2[n], self.y2[n]
            )?;
        }
        Ok(())
    }
}

impl Svg {
    pub fn new4<F: Float>(x: u16, y: u16, s: &[F]) -> Result<Self> {
        let c = CanvasSize { x, y };
        let lines = SvgLines::new(s, &c)?;
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
        write!(f, "{}", self.lines)?;
        writeln!(f, "</svg>")
    }
}

pub(crate) struct CanvasSize {
    x: u16,
    y: u16,
}


fn f2canvas<F: Float, I: NumCast>(f: F, i: I) -> Result<I> {
    let i_as_f: F = NumCast::from(i).context("Cast error")?;
    NumCast::from((f * i_as_f).round()).context("Cast error")
}

#[cfg(test)]
mod test {
    use crate::svg::{CanvasSize, Svg, SvgLines, f2canvas};
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
    fn display_lines() {
        let data: Vec<f32> = vec![0.1, 0.2, 0.3, 0.4];
        let c = CanvasSize { x:100, y: 100 };
        let sut = SvgLines::new(&data, &c).unwrap();
        let expected =
            "<line x1=\"10\" y1=\"20\" x2=\"30\" y2=\"40\" stroke=\"black\" stroke-width=\"2\"/>\n"
                .to_string();
        let result = sut.to_string();
        assert_eq!(result, expected);        
    }

    #[test]
    fn diplay_lines_more() {
        let data:  Vec<f32> = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let c = CanvasSize { x: 100, y: 100 };
        let sut = SvgLines::new(&data, &c).unwrap();
        let expected = indoc! {"\
            <line x1=\"10\" y1=\"20\" x2=\"30\" y2=\"40\" stroke=\"black\" stroke-width=\"2\"/>
            <line x1=\"50\" y1=\"60\" x2=\"70\" y2=\"80\" stroke=\"black\" stroke-width=\"2\"/>
        "};

        assert_eq!(sut.to_string(), expected.to_string());
    }

    #[test]
    fn svg_empty() {
        let s = Svg::new4(10, 20, &[] as &[f32]).unwrap();
        let expected = indoc! {"
            <svg viewBox=\"0 0 10 20\" xmlns=\"http://www.w3.org/2000/svg\">
            </svg>
        "};
        assert_eq!(s.to_string(), expected.to_string());
    }

    #[test]
    fn svg_one() {
        let data: Vec<f32> = vec![0.25, 0.25, 0.75, 0.75];
        let s = Svg::new4(100, 100, &data).unwrap();
        let expected = indoc! {"
            <svg viewBox=\"0 0 100 100\" xmlns=\"http://www.w3.org/2000/svg\">
            <line x1=\"25\" y1=\"25\" x2=\"75\" y2=\"75\" stroke=\"black\" stroke-width=\"2\"/>
            </svg>
        "};
        assert_eq!(s.to_string(), expected.to_string());
    }
}
