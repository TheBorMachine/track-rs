mod app;

use app::{create_app, create_client};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let config = app::Config::new();

    let client = create_client().await.unwrap();

    if let Ok(app) = create_app(client).await {
        let addr = "localhost:8000";
        let listener = TcpListener::bind(addr).await.unwrap();
        println!("Сервер запущен на {}", addr);

        let router = app.into_make_service();

        axum::serve(listener, router).await.unwrap();
    }
}
