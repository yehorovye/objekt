use actix_web::{HttpResponse, Responder, get, web::Data};
use serde_json::json;

use crate::{AppState, guards::path::SanitizedKey, providers::CacheProvider};

macros_utils::routes! {
    route route_entry
}

#[get("/{key:.*}")]
pub async fn route_entry(key: SanitizedKey, state: Data<AppState>) -> impl Responder {
    match state.provider.entry(key.0).await {
        Some(entry) => HttpResponse::Ok().json(entry),
        None => HttpResponse::NotFound().json(json!({
            "ok": false,
            "message": "This entry does not exist",
            "data": {}
        })),
    }
}
