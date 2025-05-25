use actix_web::{HttpResponse, Responder, get};

macros_utils::routes! {
    route route_root,
}

#[get("/")]
pub async fn route_root() -> impl Responder {
    HttpResponse::Ok().body("Ok!")
}
