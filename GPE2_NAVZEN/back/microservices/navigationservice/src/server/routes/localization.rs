use actix_web::{web, HttpResponse, Responder};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(localize));
}

async fn localize() -> impl Responder {
    HttpResponse::Ok().body("Localization API")
}
