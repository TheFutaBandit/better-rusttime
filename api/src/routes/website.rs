use std::sync::{Arc, Mutex};

use poem::{handler, web::{Data, Json, Path}};



use store::{store::Store};

use crate::{request_inputs::{CreateWebsiteInput}, request_outputs::{CreateWebsiteOutput, GetWebsiteOutput}};



#[handler]
pub fn get_website(Path(website_id): Path<String>, Data(s) : Data<&Arc<Mutex<Store>>>) -> Json<GetWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();

    let website_response = locked_s.get_website(website_id).unwrap();

    let response = GetWebsiteOutput {
        website: website_response.time_added.to_string()
    };

    Json(response)

    
}

#[handler]
pub fn create_website(Json(data) : Json<CreateWebsiteInput>, Data(s) : Data<&Arc<Mutex<Store>>>) -> Json<CreateWebsiteOutput> {
    // make db call
    let url = data.url;

    let mut locked_s = s.lock().unwrap();

    let website_response = locked_s.create_website(url, String::from("e7f40797-70ec-4182-828c-320a78a5daed")).unwrap();

    let response = CreateWebsiteOutput{
        id: website_response.id
    };

    Json(response)
}