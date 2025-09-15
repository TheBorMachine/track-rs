mod events;

use tokio::net::TcpListener;

pub async fn run() {
    let client = match mongodb::Client::with_uri_str("mongodb://localhost:27017").await {
        Ok(client) => client,
        Err(error) => panic!("{}", error),
    };

    let router = axum::Router::new()
        .merge(events::router(client))
        .into_make_service();

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
