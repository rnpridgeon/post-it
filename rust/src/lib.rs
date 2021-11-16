use std::net::TcpListener;

use axum::{
    AddExtensionLayer, 
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, Router}};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};
use tower_http::cors::CorsLayer;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    content: String
}

impl Message {
    pub fn new(msg_content: &str) -> Self {
        Self {content: msg_content.to_string()}
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub trait StateStore {
    fn create(&mut self, payload: String) -> Result<()>;
    fn list(&self) -> Result<Vec<String>>;
}

pub type InMemoryStateStore = Vec<String>;

impl StateStore for InMemoryStateStore {
    #[allow(clippy::unit_arg)]
    fn create(&mut self, payload: String) -> Result<()> {
        Ok(self.push(payload))
    }

    fn list(&self) -> Result<Vec<String>> {
        Ok(self.clone())
    }
}

enum Event {
    CreateMessage {
        payload: String,
        response_channel: oneshot::Sender<Result<()>>,
    },
    ListMessage {
        response_channel: oneshot::Sender<Result<Vec<String>>>,
    },
}

async fn create_message(Extension(store): Extension<mpsc::Sender<Event>>, Json(msg): Json<Message>) -> impl IntoResponse {
    let (tx, rx) = oneshot::channel();

    if msg.content.trim().is_empty() {
        return StatusCode::BAD_REQUEST
    }

    let event = Event::CreateMessage {
        payload: msg.content,
        response_channel: tx,
    };
    
    let _ = store.send(event).await;

    match rx.await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    }
}

async fn list_message(Extension(store): Extension<mpsc::Sender<Event>>) -> impl IntoResponse {
    let (tx, rx) = oneshot::channel();

    // An error occured placing an event on the events channel
    match store.send(Event::ListMessage{response_channel: tx}).await {
        Ok(_) => (),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(vec!())),
    };

    match rx.await {
        Ok(msgs) => (StatusCode::OK, Json(msgs.unwrap())),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec!()))
    }
}

pub async fn run(listener: TcpListener) -> Result<()> {
    // State Store event queue
    let (tx, mut rx) = mpsc::channel(100);    
    let _state_manager = tokio::spawn(async move {
        let mut store = InMemoryStateStore::new();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                Event::CreateMessage {
                    payload,
                    response_channel,
                } => {
                    let _ = response_channel.send(store.create(payload));
                }
                Event::ListMessage { response_channel } => {
                    let _ = response_channel.send(store.list());
                }
            }
        }
        rx.close()
    });

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/message", post(create_message).get(list_message))
        .layer(CorsLayer::permissive())
        .layer(AddExtensionLayer::new(tx));

    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
