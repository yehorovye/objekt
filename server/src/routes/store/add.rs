use actix_web::{
    HttpResponse, Responder, put,
    web::{Data, Json},
};
use serde_json::{Value, json};

use crate::{
    AppState,
    guards::{auth::AuthUser, path::SanitizedKey},
    providers::CacheProvider,
};

macros_utils::routes! {
    route route_add
}

#[put("/{key:.*}")]
pub async fn route_add(
    key: SanitizedKey,
    value: Json<Value>,
    state: Data<AppState>,
    user: AuthUser,
) -> impl Responder {
    match state
        .provider
        .add(key.0.clone(), value.into_inner(), user.0.name)
        .await
    {
        Some(_) => HttpResponse::Created().json(json!({
            "ok": true,
            "message": "Created cache entry",
            "data": {}
        })),
        None => HttpResponse::BadRequest().json(json!({
            "ok": false,
            "message": "This entry already exists",
            "data": {}
        })),
    }
}
