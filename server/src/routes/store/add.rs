use actix_web::{
    HttpRequest, HttpResponse, Responder,
    http::header,
    put,
    web::{Data, Json, Path},
};
use serde_json::{Value, json};

use crate::{AppState, providers::CacheProvider};

macros_utils::routes! {
    route route_add
}

#[put("/{key:.*}")]
pub async fn route_add(
    key: Path<String>,
    value: Json<Value>,
    state: Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    if let Some(issuer) = req.headers().get(header::AUTHORIZATION) {
        let users = state.users.lock().await;

        let hash = issuer.to_str().ok().map(|s| s.to_string());

        if let Some(token) = hash {
            let cache = state.provider.clone();

            let user = users.values().find(|v| v.password_hash == token);

            if let Some(usr) = user {
                if cache.has_key(key.to_owned()).await {
                    return HttpResponse::BadRequest().json(json!({
                        "ok": false,
                        "message": "this entry already exists",
                        "data": {}
                    }));
                }

                let username = usr.clone().name;

                let _ = cache
                    .add(key.to_owned().replace("/", ":"), value.0, username)
                    .await;

                return HttpResponse::Created().json(json!({
                    "ok": true,
                    "message": "created cache entry",
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
