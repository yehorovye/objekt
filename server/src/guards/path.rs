use actix_web::{Error, FromRequest, HttpRequest, dev::Payload};
use futures::future::{Ready, ready};

#[derive(Debug, Clone)]
pub struct SanitizedKey(pub String);

impl FromRequest for SanitizedKey {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let key_result = req.match_info().get("key").map(|s| s.replace("/", ":"));
        match key_result {
            Some(sanitized) => ready(Ok(SanitizedKey(sanitized))),
            None => ready(Ok(SanitizedKey(String::new()))),
        }
    }
}
