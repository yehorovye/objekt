use actix_web::{HttpResponse, Responder, get, web::Data};
use serde_json::json;

use crate::{
    AppState, guards::path::SanitizedKey, providers::CacheProvider, structs::metadata::Metadata,
};

macros_utils::routes! {
    route route_metadata,
}

#[get("/{key:.*}$")]
pub async fn route_metadata(key: SanitizedKey, state: Data<AppState>) -> impl Responder {
    let cache = state.provider.clone();

    let entry: Option<Metadata> = cache.metadata(key.0).await;

    if let Some(entry) = entry {
        return HttpResponse::Ok().json(entry);
    }

    HttpResponse::NotFound().json(json!({
        "ok": false,
        "message": "this entry does not exist",
        "data": {}
    }))
}
