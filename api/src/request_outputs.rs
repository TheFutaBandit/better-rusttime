use serde::{Serialize, Deserialize};
use store::models::website::Website;

#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteOutput {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignUpOutput {
    pub id: String
}

#[derive(Serialize, Deserialize)]
pub struct SignInOutput {
    pub jwt: String
}

#[derive(Serialize, Deserialize)]
pub struct getWebsiteOutput {
    pub website: String
}

