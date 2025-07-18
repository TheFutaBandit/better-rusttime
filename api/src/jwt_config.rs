use dotenvy;

pub struct jwt_config {
    pub secret: String
}

impl Default for jwt_config {
    fn default() -> Self {
        dotenvy::dotenv().ok();

        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| panic!("JWT_SECRET NOT RECEIVED"));

        Self {
            secret
        }
    }
}
