use dotenvy;

pub struct Config {
    pub database_url: String,
}

impl Default for Config {
    fn default() -> Self {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| panic!("DATABASE_URL must be set"));

        Self { database_url }
    }
}