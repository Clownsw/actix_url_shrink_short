use sqlx::mysql::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: MySqlPool,
}