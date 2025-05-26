use actix_web::{
    HttpRequest, HttpResponse, Responder,
    http::header,
    put,
    web::{Data, Json, Path},
};
use serde_json::{Value, json};

use crate::{AppState, providers::CacheProvider, utils::sanitize_path_keys};

macros_utils::routes! {
    route route_upsert
}

/// If the cache route ends in `!` it will be upsert
#[put("/{key:.*}!")]
pub async fn route_upsert(
    key: Path<String>,
    value: Json<Value>,
    state: Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    if let Some(issuer) = req.headers().get(header::AUTHORIZATION) {
        let users = state.users.lock().await;
        let sanitized_key = sanitize_path_keys(key.to_owned());

        let hash = issuer.to_str().ok().map(|s| s.to_string());

        if let Some(token) = hash {
            let cache = state.provider.clone();

            let user = users.values().find(|v| v.password_hash == token);

            if let Some(usr) = user {
                let username = usr.clone().name;

                let _ = cache.upsert(sanitized_key, value.0, username).await;

                return HttpResponse::Created().json(json!({
                    "ok": true,
                    "message": "updated/created cache entry",
                    "data": {}
                }));
            }
        }
    };

    HttpResponse::Unauthorized().json(json!({
        "ok": false,
        "message": "you lack the permissions needed for this route",
        "data": {}
    }))
}
