use anyhow::Result;
use clickhouse::Client;

pub async fn create_client() -> Result<Client> {
    let client = Client::default()
        .with_url("http://localhost:18123")
        .with_user("click")
        .with_password("click")
        .with_database("tracking")
        .with_option("async_insert", "1")
        .with_option("wait_for_async_insert", "0");

    let result: u8 = client.query("SELECT 1").fetch_one().await?;

    println!("Query result: {}", result);

    Ok(client)
}
