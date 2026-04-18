mod floats;
use crate::floats::Line;

struct SVGLIne {
    x1: u16,
    x2: u16,
    y1: u16,
    y2: u16,
}

struct CanvasSize {
    x: u16,
    y: u16,
}

impl SVGLIne {
    fn from_line<T>(l: Line<T>, c: CanvasSize) -> Self {
        Self {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        }
    }
}

fn main() {
    let l: Line<f32> = Line::new_random();
    println!("{l}")
}
