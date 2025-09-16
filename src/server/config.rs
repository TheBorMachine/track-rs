pub struct Config {
    addr: String,
    mongo: String,
}

impl Config {
    pub fn addr(&self) -> String {
        self.addr.clone()
    }

    pub fn mongo_uri(&self) -> String {
        self.mongo.clone()
    }
}

impl Default for Config {
    fn default() -> Self {
        let addr = match std::env::var("SERVER_ADDR") {
            Ok(value) => value,
            Err(_) => String::from("127.0.0.1:8000"),
        };

        let mongo = match std::env::var("MONGO_URI") {
            Ok(value) => value,
            Err(_) => String::from("mongodb://localhost:27017"),
        };

        Self { addr, mongo }
    }
}
