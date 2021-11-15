
use std::{env, net::TcpListener};

#[tokio::main]
async fn main() -> post_it::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        panic!("Usage: post-it address");
    }

    let listener = TcpListener::bind(&args[1])?;

    post_it::run(listener).await?;
    
    Ok(())
}