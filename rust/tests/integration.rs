use std::net::TcpListener;
use serde_json::json;

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
        .get(&format!("http://{}/", addr))
        .send()
        .await
        .expect("Failed to post message.");


    let status = response.status();
    let response_data = &response.text().await.unwrap();
    
    assert_eq!(status, reqwest::StatusCode::OK);
    assert_eq!("Hello, World!", response_data)
}

#[tokio::test]
async fn test_create_message() {
    let addr = setup().await.expect("Failed to setup test server");
    let client = reqwest::Client::new();

    let msg = post_it::Message::new("This is a message");

    let response = client
        .post(&format!("http://{}/api/message", addr))
        .json(&msg)
        .send()
        .await
        .expect("Failed to post message.");


    assert!(&response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn test_create_message_literal() {
    let addr = setup().await.expect("Failed to setup test server");
    let client = reqwest::Client::new();

    let msg = json!({"content": "this is a message"});

    let response = client
        .post(&format!("http://{}/api/message", addr))
        .json(&msg)
        .send()
        .await
        .expect("Failed to post message.");


    let status = response.status();
    assert_eq!(status, reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_invalid_input() {
    let addr = setup().await.expect("Failed to setup test server");
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("http://{}/api/message", addr))
        .json("")
        .send()
        .await
        .expect("Failed to post message.");

    let status = response.status();

    assert_eq!(status, reqwest::StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_empty_message_content() {
    let addr = setup().await.expect("Failed to setup test server");
    let client = reqwest::Client::new();

    let msg = post_it::Message::new("");

    let response = client
        .post(&format!("http://{}/api/message", addr))
        .json(&msg)
        .send()
        .await
        .expect("Failed to post message.");

    let status = response.status();

    assert_eq!(status, reqwest::StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_whitespace_only_message_content() {
    let addr = setup().await.expect("Failed to setup test server");
    let client = reqwest::Client::new();

    let msg = post_it::Message::new("       ");

    let response = client
        .post(&format!("http://{}/api/message", addr))
        .json(&msg)
        .send()
        .await
        .expect("Failed to post message.");

    let status = response.status();

    assert_eq!(status, reqwest::StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_unsupported_http_method() {
    let addr = setup().await.expect("Failed to setup test server");
    let client = reqwest::Client::new();

    let response = client
        .put(&format!("http://{}/api/message", addr))
        .json("")
        .send()
        .await
        .expect("Failed to post message.");

    let status = response.status();

    assert_eq!(status, reqwest::StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn test_list_message() {
    let addr = setup().await.expect("Failed to setup test server");
    let client = reqwest::Client::new();

    let msg = post_it::Message::new("This is a message");
    let another_msg = post_it::Message::new("This is another message");

    let _ = 
        client
            .post(&format!("http://{}/api/message", addr))
            .json(&msg)
            .send()
            .await
            .expect("Failed to post message.");

    let _ = 
        client
            .post(&format!("http://{}/api/message", addr))
            .json(&another_msg)
            .send()
            .await
            .expect("Failed to post message.");

    let response =  
        client
            .get(&format!("http://{}/api/message", addr))
            .send()
            .await
            .expect("Failed to list messages.");

    let status = response.status();
    let response_data = &response.json::<Vec<String>>().await.unwrap();

    assert_eq!(status, reqwest::StatusCode::OK);
    assert_eq!(2, response_data.len());
    assert!(response_data.contains(&msg.content().to_string()));
    assert!(response_data.contains(&another_msg.content().to_string()));
}

#[tokio::test]
async fn test_lst_message_empty() {
    let addr = setup().await.expect("Failed to setup test server");
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://{}/api/message", addr))
        .send()
        .await
        .expect("Failed to post message.");

    let status = response.status();
    let response_data = &response.json::<Vec<String>>().await.unwrap();

    assert_eq!(status, reqwest::StatusCode::OK);
    assert_eq!(0, response_data.len());
}
