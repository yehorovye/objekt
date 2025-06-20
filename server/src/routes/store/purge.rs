use actix_web::{HttpResponse, Responder, delete, web::Data};
use serde_json::json;

use crate::{AppState, guards::auth::AuthUser, providers::CacheProvider};

macros_utils::routes! {
    route route_purge
}

#[delete("/!")]
pub async fn route_purge(state: Data<AppState>, user: AuthUser) -> impl Responder {
    let cache = state.provider.clone();
    cache.purge(user.0.name).await;

    HttpResponse::Ok().json(json!({
        "ok": true,
        "message": "purged all entries owned by this user",
        "data": {}
    }))
}
