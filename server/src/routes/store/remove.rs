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
    let cache = &state.provider;

    match cache.remove(key.0).await {
        Some(_) => HttpResponse::Ok().json(json!({
            "ok": true,
            "message": "Deleted cache entry",
            "data": {}
        })),
        None => HttpResponse::NotFound().json(json!({
            "ok": false,
            "message": "This entry does not exist",
            "data": {}
        })),
    }
}
