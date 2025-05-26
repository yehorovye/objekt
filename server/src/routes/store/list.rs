use actix_web::{
    HttpResponse, Responder, get,
    web::{Data, Path},
};

use crate::{AppState, providers::CacheProvider, utils::sanitize_path_keys};

macros_utils::routes! {
    route route_list
}

/// If the cache route ends in `/` returns a list of entries starting by the key
#[get("/{key:.*}/")]
pub async fn route_list(key: Path<String>, state: Data<AppState>) -> impl Responder {
    let cache = state.provider.clone();
    let sanitized_key = sanitize_path_keys(key.to_owned());

    let list = cache.list().await;
    let mapped_list = list.iter().map(|v| &v.0).collect::<Vec<_>>();

    if sanitized_key.is_empty() {
        HttpResponse::Ok().json(mapped_list)
    } else {
        let filtered_list = mapped_list
            .iter()
            .filter(|v| v.starts_with(&sanitized_key))
            .collect::<Vec<_>>();

        HttpResponse::Ok().json(filtered_list)
    }
}
