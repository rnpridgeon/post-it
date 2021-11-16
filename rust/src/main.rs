use std::net::TcpListener;

#[tokio::main]
async fn main() -> post_it::Result<()> {
    // Binding to all interfaces to satisfy docker-compose environment
    let listener = TcpListener::bind("0.0.0.0:3001")?;

    post_it::run(listener).await
}
