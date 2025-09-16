mod config;
mod events;

use tokio::net::TcpListener;

pub async fn run() {
    let cfg = config::Config::default();

    let client = match mongodb::Client::with_uri_str(cfg.mongo_uri()).await {
        Ok(client) => client,
        Err(error) => {
            log::error!("Error while connecting to MongoDB: {}", error);
            std::process::exit(1);
        }
    };

    let router = axum::Router::new()
        .merge(events::router(client))
        .into_make_service();

    let listener = TcpListener::bind(cfg.addr()).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
