use std::sync::{Arc, Mutex};

use poem::{get, listener::TcpListener, post, EndpointExt, Route, Server};

use store::{store::Store};

use crate::routes::{user::{user_signin, user_signup}, website::{create_website, get_website}};

pub mod request_inputs;
pub mod request_outputs;

pub mod routes;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    let s: Arc<Mutex<Store>> = Arc::new(Mutex::new(Store::new().unwrap()));

    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/user/sign_up", post(user_signup))
        .at("/user/sign_in", post(user_signin))
        .data(s);
    
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}