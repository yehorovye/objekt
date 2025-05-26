use actix_web::{HttpResponse, Responder, get, web::Data};

use crate::{AppState, guards::path::SanitizedKey, providers::CacheProvider};

macros_utils::routes! {
    route route_list
}

/// If the cache route ends in `/` returns a list of entries starting by the key
#[get("/{key:.*}/")]
pub async fn route_list(key: SanitizedKey, state: Data<AppState>) -> impl Responder {
    let cache = state.provider.clone();

    let list = cache.list().await;
    let mapped_list = list.iter().map(|v| &v.0).collect::<Vec<_>>();

    if key.0.is_empty() {
        HttpResponse::Ok().json(mapped_list)
    } else {
        let filtered_list = mapped_list
            .iter()
            .filter(|v| v.starts_with(&key.0))
            .collect::<Vec<_>>();

        HttpResponse::Ok().json(filtered_list)
    }
}
