use std::sync::{Arc, Mutex};

use jsonwebtoken::{encode, EncodingKey, Header};
use poem::{handler, http::StatusCode, web::{Data, Json}};

use store::{store::Store};

use serde::{Serialize, Deserialize};

use crate::{request_inputs::{SignInInput, SignUpInput}, request_outputs::{SignInOutput, SignUpOutput}};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize
}

#[handler]
pub fn user_signup(
    Json(data) : Json<SignUpInput>, 
    Data(s) : Data<&Arc<Mutex<Store>>>
) -> Result<Json<SignUpOutput>, poem::Error> {
    let username = data.username;
    let password = data.password;

    let mut locked_s = s.lock().unwrap();

    let response_id = locked_s.sign_up(username, password).map_err(|_| poem::Error::from_status(StatusCode::CONFLICT));

    match response_id {
        Ok(id) => {
            let response = SignUpOutput {
                id
            };
        
            Ok(Json(response))
        },
        Err(e) => Err(e)
    }
    
}

#[handler]
 pub fn user_signin(
    Json(data) : Json<SignInInput>, 
    Data(s) : Data<&Arc<Mutex<Store>>>
) -> Result<Json<SignInOutput>, poem::Error> {
    let username = data.username;
    let password = data.password;

    let mut locked_s = s.lock().unwrap();

    let result_id = locked_s.sign_in(username, password);

    match result_id {
        Ok(id) => {
            let user_claim =  Claims {
                sub: id,
                exp: 60*60*24
            };

            let token = encode(&Header::default(), &user_claim, &EncodingKey::from_secret("secret".as_ref()))
                .map_err(|_| poem::Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

            let response = SignInOutput {
                jwt: token
            };
        
            Ok(Json(response))
        },
        Err(_) => Err(poem::Error::from_status(StatusCode::UNAUTHORIZED))
    }

    
}