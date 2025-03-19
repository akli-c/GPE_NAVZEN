use actix_web::{App, HttpServer};
use crate::server::routes;

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
