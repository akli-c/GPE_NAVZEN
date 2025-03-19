use dotenv::dotenv;
use std::env;

pub struct Config {
    pub port: u16,
    pub db_url: String,
    pub pg_url: String,
    pub map_file: String,
    pub surface_info_file: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();  // Charge les variables d'environnement depuis .env

        Self {
            port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap(),
            db_url: format!(
                "mysql://{}:{}@{}:{}/{}",
                env::var("DB_USER").unwrap(),
                env::var("DB_PASSWORD").unwrap(),
                env::var("DB_HOST").unwrap(),
                env::var("DB_PORT").unwrap(),
                env::var("DB_NAME").unwrap()
            ),
            pg_url: format!(
                "postgres://{}:{}@{}:{}/{}",
                env::var("PG_USER").unwrap(),
                env::var("PG_PASSWORD").unwrap(),
                env::var("PG_HOST").unwrap(),
                env::var("PG_PORT").unwrap(),
                env::var("PG_NAME").unwrap()
            ),
            map_file: env::var("MAP_FILE_PATH").unwrap(),
            surface_info_file: env::var("SURFACE_INFO_PATH").unwrap(),
        }
    }
}
