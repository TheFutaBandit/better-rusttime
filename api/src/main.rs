use poem::{get, handler, listener::TcpListener, post, web::{Json, Path}, Error, IntoResponse, Route, Server};

use store::{models::website, store::Store};

use crate::{request_inputs::{CreateWebsiteInput, SignInInput, SignUpInput}, request_outputs::{getWebsiteOutput, CreateWebsiteOutput, SignInOutput, SignUpOutput}};

pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn user_signup(Json(data) : Json<SignUpInput>) -> Json<SignUpOutput> {
    let username = data.username;
    let password = data.password;

    let mut store = Store::default().unwrap();

    let id = store.sign_up(username, password).unwrap_or_else(|_| "Error signing up".to_string());

    let response = SignUpOutput {
        id
    };

    Json(response)

}

#[handler]
fn user_signin(Json(data) : Json<SignInInput>) -> Json<SignInOutput> {
    let username = data.username;
    let password = data.password;

    let mut store = Store::default().unwrap();

    let flag = store.sign_in(username, password).unwrap();

    let response = SignInOutput {
        jwt: String::from("username")
    };

    Json(response)
}

#[handler]
fn get_website(Path(website_id): Path<String>) -> Json<getWebsiteOutput> {
    let mut store = Store::default().unwrap();

    let website_response = store.get_website(website_id).unwrap();

    let response = getWebsiteOutput {
        website: website_response.time_added.to_string()
    };

    Json(response)

    
}

#[handler]
fn create_website(Json(data) : Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    // make db call
    let url = data.url;

    let mut store = Store::default().unwrap();

    let website_response = store.create_website(url, String::from("e7f40797-70ec-4182-828c-320a78a5daed")).unwrap();

    let response = CreateWebsiteOutput{
        id: website_response.id
    };

    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/user/sign_up", post(user_signup))
        .at("/user/sign_in", post(user_signin));
    
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}