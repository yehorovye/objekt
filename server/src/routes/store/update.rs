use actix_web::{
    HttpResponse, Responder, patch,
    web::{Data, Json},
};
use serde_json::{Value, json};

use crate::{
    AppState,
    guards::{auth::AuthUser, path::SanitizedKey},
    providers::CacheProvider,
};

macros_utils::routes! {
    route route_upsert
}

#[patch("/{key:.*}")]
pub async fn route_upsert(
    key: SanitizedKey,
    value: Json<Value>,
    state: Data<AppState>,
    user: AuthUser,
) -> impl Responder {
    let cache = state.provider.clone();
    let username = user.0.name;

    return match cache.update(key.0, value.0, username).await {
        Some(value) => HttpResponse::Ok().json(json!({
            "ok": true,
            "message": "updated entry",
            "data": value
        })),
        None => HttpResponse::BadRequest().json(json!({
            "ok": false,
            "message": "entry does not exist",
            "data": {}
        })),
    };
}
