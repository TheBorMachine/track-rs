mod logger;
mod server;

#[tokio::main]
async fn main() {
    logger::init();
    server::run().await;
}
