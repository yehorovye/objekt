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
    route route_upsert
}

/// If the cache route ends in `!` it will be upsert
#[put("/{key:.*}!")]
pub async fn route_upsert(
    key: SanitizedKey,
    value: Json<Value>,
    state: Data<AppState>,
    user: AuthUser,
) -> impl Responder {
    let cache = state.provider.clone();
    let username = user.0.name;

    cache.upsert(key.0, value.0, username).await;

    HttpResponse::Created().json(json!({
        "ok": true,
        "message": "updated/created cache entry",
        "data": {}
    }))
}
