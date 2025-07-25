use serde::{Serialize, Deserialize};

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
    pub token: String
}

#[derive(Serialize, Deserialize)]
pub struct GetWebsiteOutput {
    pub website_response: String
}

