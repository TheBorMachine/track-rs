use std::env;

#[derive(Debug)]
pub struct Clickhouse {
    addr: String,
    user: String,
    password: String,
    database: String,
}

#[derive(Debug)]
pub struct Config {
    pub port: String,
    pub clickhouse: Clickhouse,
}

impl Config {
    pub fn new() -> Self {
        let clickhouse = Clickhouse {
            addr: Config::value("clickhouse_server", "localhost:8192".to_string()),
            user: Config::value("clickhouse_user", "click".to_string()),
            password: Config::value("clickhouse_password", "click".to_string()),
            database: Config::value("clickhouse_database", "default".to_string()),
        };
        Config {
            port: Config::value("server_addr", "localhost:8080".to_string()),
            clickhouse,
        }
    }

    fn value(key: &str, default: String) -> String {
        match env::var(key.to_uppercase()) {
            Ok(v) => v,
            Err(_) => default,
        }
    }
}
