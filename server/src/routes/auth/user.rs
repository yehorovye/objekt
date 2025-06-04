use actix_web::{
    HttpResponse, Responder, post,
    web::{Data, Json, Path},
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    AppState, providers::CacheProvider, routes::auth::generate_user_token, structs::user::User,
};

macros_utils::routes! {
    route route_user,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserPayload {
    password: String,
}

#[post("/{user}")]
pub async fn route_user(
    user: Path<String>,
    payload: Json<CreateUserPayload>,
    state: Data<AppState>,
) -> impl Responder {
    let username = user.into_inner();
    let password = payload.password.clone();

    let users = state.users.clone();
    let password_hash = generate_user_token(&username, &password);

    if users.list().await.iter().any(|v| v.1.name == username) {
        return HttpResponse::BadRequest().json(json!({
            "ok": false,
            "message": "user already exists"
        }));
    }

    users
        .add(
            username.clone(),
            User {
                name: username,
                password_hash: password_hash.clone(),
            },
            String::from("system"),
        )
        .await;

    HttpResponse::Created().json(json!({
        "ok": true,
        "message": "created user",
        "data": {
            "token": password_hash
        }
    }))
}
