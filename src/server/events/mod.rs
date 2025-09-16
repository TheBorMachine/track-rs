mod models;
mod repository;
mod service;

use axum::routing;
use tokio::{
    sync::mpsc,
    time::{Duration, Instant, sleep},
};

const BUFFER_FLUSH_INTERVAL: u64 = 5;
const BUFFER_CAPACITY: usize = 50_000;

pub fn router(client: mongodb::Client) -> axum::Router {
    let (sender, mut reciever) = mpsc::channel::<models::Event>(BUFFER_CAPACITY);

    let repository = repository::Events::new(client);

    tokio::spawn(async move {
        let mut buffer: Vec<models::Event> = Vec::with_capacity(BUFFER_CAPACITY);
        let mut last_flush = Instant::now();

        loop {
            tokio::select! {
                events = reciever.recv() => {
                    if let Some(events) = events {
                        buffer.push(events);
                    }
                }

                _ = sleep(Duration::from_millis(100)) => {}
            }

            if buffer.len() >= 10_000
                || last_flush.elapsed() > Duration::from_secs(BUFFER_FLUSH_INTERVAL)
            {
                if !buffer.is_empty() {
                    let chunk = std::mem::replace(&mut buffer, Vec::with_capacity(BUFFER_CAPACITY));
                    _ = repository.create_many(&chunk).await;
                    last_flush = Instant::now();
                }
            }
        }
    });

    axum::Router::new()
        .route("/event", routing::post(service::create))
        .route("/events", routing::post(service::create_many))
        .with_state(sender)
}
