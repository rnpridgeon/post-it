
use std::net::TcpListener;

async fn setup() -> post_it::Result<String> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;

    tokio::spawn(post_it::run(listener));
    
    Ok(format!("{}", address))
}

#[tokio::test]
async fn test_hello_world() {
    let addr = setup().await.expect("Failed to setup test server");
    let client = reqwest::Client::new();

    let response = client
        // Use the returned application address
        .get(&format!("http://{}/", addr))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(&response.status().is_success());
    assert_eq!(Some(13), response.content_length());
    assert_eq!("Hello, World!", response.text().await.expect("Failed to read response"));
}
