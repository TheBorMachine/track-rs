use super::models::{Event};
use crate::app::App;

use axum::{Json, extract::State, http::StatusCode};

use std::sync::Arc;

pub async fn events(State(state): State<Arc<App>>, Json(data): Json<Vec<Event>>) -> StatusCode {
    if let Err(_) = state.chanel.send(data).await {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    StatusCode::CREATED
}
