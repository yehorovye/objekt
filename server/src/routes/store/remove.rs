use actix_web::{
    HttpRequest, HttpResponse, Responder, delete,
    http::header,
    web::{Data, Path},
};
use serde_json::json;

use crate::{AppState, providers::CacheProvider, utils::sanitize_path_keys};

macros_utils::routes! {
    route route_remove
}

#[delete("/{key:.*}")]
pub async fn route_remove(
    key: Path<String>,
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

            if user.is_some() {
                if !cache.has_key(sanitized_key.clone()).await {
                    return HttpResponse::NotFound().json(json!({
                        "ok": false,
                        "message": "this entry does not exist",
                        "data": {}
                    }));
                }

                let _ = cache.remove(sanitized_key).await;

                return HttpResponse::Ok().json(json!({
                    "ok": true,
                    "message": "deleted cache entry",
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
