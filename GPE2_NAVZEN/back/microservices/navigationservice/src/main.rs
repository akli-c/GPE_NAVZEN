// src/main.rs
mod config;
mod data;
mod navigation;
mod server;
mod services;
mod utils;

use actix_web::{App, HttpServer};
use server::routes::configure;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Charger les variables d'environnement
    dotenv::dotenv().ok();
    env_logger::init(); // Active les logs

    // Lire le PORT depuis .env, sinon utiliser 8080 par défaut
    let port: u16 = env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().expect("Port invalide");

    println!("🚀 Démarrage du serveur sur http://127.0.0.1:{}", port);

    HttpServer::new(|| {
        App::new()
            .configure(configure) // Charge toutes les routes
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
