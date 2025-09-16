use axum::{Json, extract::State, http::StatusCode};
use tokio::sync::mpsc;

use super::models;

pub async fn create(
    State(sender): State<mpsc::Sender<models::Event>>,
    Json(event): Json<models::Event>,
) -> StatusCode {
    match sender.send(event).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn create_many(
    State(sender): State<mpsc::Sender<models::Event>>,
    Json(events): Json<Vec<models::Event>>,
) -> StatusCode {
    for event in events {
        _ = match sender.send(event).await {
            Ok(_) => StatusCode::CREATED,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    StatusCode::CREATED
}
