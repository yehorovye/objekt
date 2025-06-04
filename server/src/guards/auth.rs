use crate::providers::CacheProvider;
use crate::{AppState, structs::user::User};
use actix_web::HttpResponse;
use actix_web::{Error, FromRequest, HttpRequest, dev::Payload, http::header, web::Data};
use futures::executor::block_on;
use futures::future::{Ready, ready};

#[derive(Debug, Clone)]
pub struct AuthUser(pub User);

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let state = match req.app_data::<Data<AppState>>() {
            Some(data) => data.clone(),
            None => return ready(Err(json_unauthorized("missing app state"))),
        };

        let token = match req.headers().get(header::AUTHORIZATION) {
            Some(header_value) => match header_value.to_str() {
                Ok(s) => s.to_string(),
                Err(_) => return ready(Err(json_unauthorized("invalid header format"))),
            },
            None => return ready(Err(json_unauthorized("missing auth header"))),
        };

        let maybe_user = {
            let users = state.users.clone();
            let list = block_on(users.list());

            list.iter().find(|u| u.1.password_hash == token).cloned()
        };

        match maybe_user {
            Some(user) => ready(Ok(AuthUser(user.1))),
            None => ready(Err(json_unauthorized("invalid token"))),
        }
    }
}

fn json_unauthorized(msg: &str) -> Error {
    actix_web::error::InternalError::from_response(
        msg.to_string(),
        HttpResponse::Unauthorized().json(serde_json::json!({
            "ok": false,
            "message": msg,
            "data": {}
        })),
    )
    .into()
}
