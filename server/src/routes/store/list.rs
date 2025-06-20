use actix_web::{HttpResponse, Responder, get, web::Data};

use crate::{AppState, guards::path::SanitizedKey, providers::CacheProvider};

macros_utils::routes! {
    route route_list
}

/// If the cache route ends in `/`, returns a list of entries starting with the key
#[get("/{key:.*}/")]
pub async fn route_list(key: SanitizedKey, state: Data<AppState>) -> impl Responder {
    let list = state.provider.list().await;

    let entries: Vec<&String> = if key.0.is_empty() {
        list.iter().map(|(k, _)| k).collect()
    } else {
        list.iter()
            .filter_map(|(k, _)| k.starts_with(&key.0).then_some(k))
            .collect()
    };

    HttpResponse::Ok().json(entries)
}
