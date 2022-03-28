use redis::Client;
use sqlx::mysql::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: MySqlPool,
    pub redis_client: Client,
}
