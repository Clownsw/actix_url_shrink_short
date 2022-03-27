use std::fs::File;
use std::io::BufReader;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use sqlx::{mysql::MySqlPool, Pool};

use controller::url_controller::{api_add_url, api_redierct};
use pojo::app_state::AppState;

use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

pub mod controller;
pub mod dao;
pub mod pojo;
pub mod util;

async fn make_db_pool() -> MySqlPool {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    Pool::connect(&database_url).await.unwrap()
}

async fn load_rustls_config() -> Option<ServerConfig> {
    let cert_name = std::env::var("CERT").unwrap();
    let key_name = std::env::var("KEY").unwrap();

    if cert_name.is_empty() || key_name.is_empty() {
        return None;
    }

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let cert_file = &mut BufReader::new(File::open(cert_name).unwrap());
    let key_file = &mut BufReader::new(File::open(key_name).unwrap());

    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();

    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    Some(config.with_single_cert(cert_chain, keys.remove(0)).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let url = std::env::var("URL").unwrap();
    let port: u16 = std::env::var("PORT").unwrap().parse::<u16>().unwrap();

    let tls_config = load_rustls_config().await;
    let db_pool = make_db_pool().await;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db_pool: db_pool.clone(),
            }))
            .wrap(Cors::permissive())
            .service(api_add_url)
            .service(api_redierct)
    });

    if let Some(v) = tls_config {
        server
            .bind_rustls(format!("{}:{}", url, port), v)
            .unwrap()
            .run()
            .await
    } else {
        server.bind((url, port)).unwrap().run().await
    }
}
