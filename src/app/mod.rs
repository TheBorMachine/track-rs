mod config;
pub use config::Config;

mod event;
use event::{Event, add_events, events};

mod storage;
pub use storage::create_client;
use tokio::{
    sync::mpsc,
    time::{Duration, sleep},
};

use std::{sync::Arc, time::Instant};

use axum::{Router, http::header, routing::post};
use clickhouse::{Client, error::Error};
use tower_http::cors::CorsLayer;

const BUFFER_CAPACITY: usize = 50_000;
const BUFFER_FLUSH_INTERVAL: u64 = 5;

struct App {
    // client: Client,
    chanel: mpsc::Sender<Vec<Event>>,
}

pub async fn create_app(client: Client) -> Result<Router, Error> {
    let (chanel, mut event_rx) = mpsc::channel(BUFFER_CAPACITY);

    let app = Arc::new(App { chanel });

    let router = Router::new()
        .route("/track", post(events))
        .layer(
            CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_headers([header::CONTENT_TYPE])
                .allow_methods([axum::http::method::Method::POST]),
        )
        .with_state(app);

    tokio::spawn(async move {
        let mut buffer: Vec<Event> = Vec::with_capacity(BUFFER_CAPACITY);
        let mut last_flush = Instant::now();

        loop {
            tokio::select! {
                events = event_rx.recv() => {
                    if let Some(events) = events {
                        buffer.extend(events);
                    }
                }

                // TODO: timer for checking reset conditions
                _ = sleep(Duration::from_millis(100)) => {}
            }

            if buffer.len() >= 10_000
                || last_flush.elapsed() > Duration::from_secs(BUFFER_FLUSH_INTERVAL)
            {
                if !buffer.is_empty() {
                    let chunk = std::mem::replace(&mut buffer, Vec::with_capacity(BUFFER_CAPACITY));
                    _ = add_events(chunk, &client).await;
                    last_flush = Instant::now();
                }
            }
        }
    });

    Ok(router)
}
