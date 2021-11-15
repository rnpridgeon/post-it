use std::net::TcpListener;

use axum::{
    routing::get,
    Router,
};

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, Error>;

pub async fn run(listener: TcpListener) -> Result<()> {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
