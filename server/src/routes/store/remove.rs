use actix_web::{HttpResponse, Responder, delete, web::Data};
use serde_json::json;

use crate::{
    AppState,
    guards::{auth::AuthUser, path::SanitizedKey},
    providers::CacheProvider,
};

macros_utils::routes! {
    route route_remove
}

#[delete("/{key:.*}")]
pub async fn route_remove(key: SanitizedKey, state: Data<AppState>, _: AuthUser) -> impl Responder {
    let cache = state.provider.clone();

    if !cache.has_key(key.0.clone()).await {
        return HttpResponse::NotFound().json(json!({
            "ok": false,
            "message": "this entry does not exist",
            "data": {}
        }));
    }

    let _ = cache.remove(key.0).await;

    HttpResponse::Ok().json(json!({
        "ok": true,
        "message": "deleted cache entry",
        "data": {}
    }))
}
