mod perceptron;
mod svg;

use crate::svg::Svg;
use axum::{Router, http::header, response::IntoResponse, routing::get};
use rand::random_range;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/ping", get(|| async { "Pong" }))
        .route("/img1.svg", get(get_img1));
    let listener = TcpListener::bind("0.0.0.0:30001").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_img1() -> impl IntoResponse {
    let s: Vec<f32> = (0..100).map(|_| random_range(0.0..1.0)).collect();
    let svg = Svg::new4(640, 480, &s).unwrap().to_string();

    ([(header::CONTENT_TYPE, "image/svg+xml")], svg)
}
