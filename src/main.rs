use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use sqlx::{mysql::MySqlPool, Pool};

use controller::url_controller::{api_add_url, api_redierct};
use pojo::app_state::AppState;

pub mod controller;
pub mod dao;
pub mod pojo;
pub mod util;

async fn make_db_pool() -> MySqlPool {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    Pool::connect(&database_url).await.unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let url = std::env::var("URL").unwrap();
    let port: u16 = std::env::var("PORT").unwrap().parse::<u16>().unwrap();

    let db_pool = make_db_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db_pool: db_pool.clone(),
            }))
            .wrap(Cors::permissive())
            .service(api_add_url)
            .service(api_redierct)
    })
    .bind((url, port))?
    .run()
    .await
}
