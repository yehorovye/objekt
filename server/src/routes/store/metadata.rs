use actix_web::{
    HttpResponse, Responder, get,
    web::{Data, Path},
};
use serde_json::json;

use crate::{
    AppState,
    providers::{CacheProvider, Metadata},
    utils::sanitize_path_keys,
};

macros_utils::routes! {
    route route_metadata
}

#[get("/{key:.*}$")]
pub async fn route_metadata(key: Path<String>, state: Data<AppState>) -> impl Responder {
    let cache = state.provider.clone();
    let sanitized_key = sanitize_path_keys(key.to_owned());

    let entry: Option<Metadata> = cache.metadata(sanitized_key).await;

    if let Some(entry) = entry {
        return HttpResponse::Ok().json(entry);
    }

    HttpResponse::NotFound().json(json!({
        "ok": false,
        "message": "this entry does not exist",
        "data": {}
    }))
}
