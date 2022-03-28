#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use sqlx::{mysql::MySqlPool, Pool};

use controller::url_controller::{api_add_url, api_redierct};
use pojo::app_state::AppState;

pub mod controller;
pub mod dao;
pub mod pojo;
pub mod util;

lazy_static! {
    pub static ref URL_TIME: Mutex<u32> = Mutex::new(0 as u32);
}

async fn make_db_pool() -> MySqlPool {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    Pool::connect(&database_url).await.unwrap()
}

async fn make_redis_client() -> redis::Client {
    let redis_url = std::env::var("REDIS_URL").unwrap();

    redis::Client::open(redis_url).unwrap()
}

async fn make_redis_url_time() {
    let mut url_time_mut_ref = URL_TIME.lock().unwrap();
    
    *url_time_mut_ref = std::env::var("REDIS_URL_TIME")
        .unwrap()
        .parse::<u32>()
        .unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let url = std::env::var("URL").unwrap();
    let port: u16 = std::env::var("PORT").unwrap().parse::<u16>().unwrap();

    let db_pool = make_db_pool().await;
    let redis_client = make_redis_client().await;

    make_redis_url_time().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db_pool: db_pool.clone(),
                redis_client: redis_client.clone(),
            }))
            .wrap(Cors::permissive())
            .service(api_add_url)
            .service(api_redierct)
    })
    .bind((url, port))?
    .run()
    .await
}
