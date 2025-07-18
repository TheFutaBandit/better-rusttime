use jsonwebtoken::{decode, DecodingKey, Validation};
use poem::{http::StatusCode, Error, FromRequest};

use crate::{jwt_config, routes::user::Claims};

pub struct UserId(pub String);

impl<'a> FromRequest<'a> for UserId {
    async fn from_request(
            req: &'a poem::Request,
            _body: &mut poem::RequestBody
        ) -> poem::Result<Self> {
        let token  = req
                .headers()
                .get("Authorization")
                .and_then(|value| value.to_str().ok())
                .ok_or_else(|| Error::from_string("missing token", StatusCode::BAD_REQUEST))?;

        let secret = jwt_config::jwt_config::default();

        let user_id = decode::<Claims>(&token, &DecodingKey::from_secret(&secret.secret.as_ref()), &Validation::default())
            .map_err(|_| poem::Error::from_string("Authentication Failure", StatusCode::FORBIDDEN))?;

        
        Ok(UserId(user_id.claims.sub.to_string()))
    }
}

