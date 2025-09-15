use axum::{Json, extract::State};

use crate::server::events::repository;

use super::models;

pub async fn create(
    State(repository): State<repository::Events>,
    Json(event): Json<models::Event>,
) {
    match repository.create(&event).await {
        () => log::info!("OK"),
    }
}

pub async fn create_many(
    State(repository): State<repository::Events>,
    Json(events): Json<Vec<models::Event>>,
) {
    match repository.create_many(&events).await {
        () => log::info!("OK"),
    }
}
