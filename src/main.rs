mod perceptron;
mod svg;

use crate::svg::Svg;
use anyhow::Result;
use rand::random_range;

fn main() -> Result<()> {
    let s: Vec<f32> = (0..100).map(|_| random_range(0.0..1.0)).collect();
    let s = Svg::new4(640, 480, &s)?;

    println!("{s}");
    Ok(())
}
