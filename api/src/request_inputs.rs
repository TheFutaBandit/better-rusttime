use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteInput {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignUpInput {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct SignInInput {
    pub username: String,
    pub password: String
}