mod models;
mod repository;
mod service;

use axum::routing;

pub fn router(client: mongodb::Client) -> axum::Router {
    let repository = repository::Events::new(client);

    axum::Router::new()
        .route("/event", routing::post(service::create))
        .route("/events", routing::post(service::create_many))
        .with_state(repository)
}
