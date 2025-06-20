use actix_web::{HttpResponse, Responder, get, web::Data};
use serde_json::json;

use crate::{AppState, guards::path::SanitizedKey, providers::CacheProvider};

macros_utils::routes! {
    route route_metadata,
}

#[get("/{key:.*}$")]
pub async fn route_metadata(key: SanitizedKey, state: Data<AppState>) -> impl Responder {
    match state.provider.metadata(key.0).await {
        Some(metadata) => HttpResponse::Ok().json(metadata),
        None => HttpResponse::NotFound().json(json!({
            "ok": false,
            "message": "This entry does not exist",
            "data": {}
        })),
    }
}
