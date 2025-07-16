use std::sync::{Arc, Mutex};

use poem::{handler, web::{Data, Json}};

use store::{store::Store};

use crate::{request_inputs::{SignInInput, SignUpInput}, request_outputs::{SignInOutput, SignUpOutput}};

#[handler]
pub fn user_signup(Json(data) : Json<SignUpInput>, Data(s) : Data<&Arc<Mutex<Store>>>) -> Json<SignUpOutput> {
    let username = data.username;
    let password = data.password;

    let mut locked_s = s.lock().unwrap();

    let id = locked_s.sign_up(username, password).unwrap_or_else(|_| "Error signing up".to_string());

    let response = SignUpOutput {
        id
    };

    Json(response)

}

#[handler]
 pub fn user_signin(Json(data) : Json<SignInInput>, Data(s) : Data<&Arc<Mutex<Store>>>) -> Json<SignInOutput> {
    let username = data.username;
    let password = data.password;

    let mut locked_s = s.lock().unwrap();

    let _flag = locked_s.sign_in(username, password).unwrap();

    let response = SignInOutput {
        jwt: String::from("username")
    };

    Json(response)
}