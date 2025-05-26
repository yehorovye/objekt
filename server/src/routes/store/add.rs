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
    let cache = state.provider.clone();

    if cache.has_key(key.0.clone()).await {
        return HttpResponse::BadRequest().json(json!({
            "ok": false,
            "message": "this entry already exists",
            "data": {}
        }));
    }

    cache.add(key.0, value.0, user.0.name).await;

    HttpResponse::Created().json(json!({
        "ok": true,
        "message": "created cache entry",
        "data": {}
    }))
}
