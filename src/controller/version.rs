use actix_web::{HttpRequest, Responder};

pub fn index(_req: &HttpRequest) -> impl Responder {
    "version: 1.0"
}
