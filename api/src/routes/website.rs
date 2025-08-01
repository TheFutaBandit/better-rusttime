use std::sync::{Arc, Mutex};

use poem::{handler, web::{Data, Json, Path}};



use store::{store::Store};

use crate::{auth_middleware::UserId, request_inputs::CreateWebsiteInput, request_outputs::{CreateWebsiteOutput, GetWebsiteOutput}};



#[handler]
pub fn get_website(
    Path(website_id): Path<String>, 
    Data(s) : Data<&Arc<Mutex<Store>>>,
    UserId(user_id) : UserId
) -> Json<GetWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();

    let website_response = locked_s.get_website(website_id, user_id).unwrap();

    let response = GetWebsiteOutput {
        website_response: website_response.id
    };

    Json(response)

    
}

#[handler]
pub fn create_website(
    Json(data) : Json<CreateWebsiteInput>, 
    Data(s) : Data<&Arc<Mutex<Store>>>,
    UserId(user_id): UserId
) -> Json<CreateWebsiteOutput> {
    // make db call
    let url = data.url;

    let mut locked_s = s.lock().unwrap();

    let website_response = locked_s.create_website(url, user_id).unwrap();

    let response = CreateWebsiteOutput{
        id: website_response.id
    };

    Json(response)
}