use actix_web::{Responder, HttpResponse, get};

#[get("ping")]
pub async fn ping() -> impl Responder {
    return HttpResponse::Ok().body("Pong");
}
