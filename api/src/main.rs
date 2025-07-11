use poem::{get, handler, listener::TcpListener, post, web::{Json, Path}, IntoResponse, Route, Server};

use store::Store;

use crate::{request_inputs::CreateWebsiteInput, request_outputs::CreateWebsiteOutput};

pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("hello")
}

#[handler]
fn create_website(Json(data) : Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    // make db call
    let url = data.url;

    let store = Store {};
    let id = store.create_website();

    let response = CreateWebsiteOutput{
        id
    };

    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}