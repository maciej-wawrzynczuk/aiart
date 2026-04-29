mod perceptron;
mod svg;

use crate::svg::Svg;
use axum::{
    Router,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
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

async fn get_img1() -> Result<impl IntoResponse, AppError> {
    let s: Vec<f32> = (0..100).map(|_| random_range(0.0..1.0)).collect();
    let svg = Svg::new4(640, 480, &s)?.to_string();

    Ok(([(header::CONTENT_TYPE, "image/svg+xml")], svg))
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}
