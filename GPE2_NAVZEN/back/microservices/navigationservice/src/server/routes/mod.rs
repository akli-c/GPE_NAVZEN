pub mod navigation;
pub mod localization;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("").configure(navigation::configure));
}
