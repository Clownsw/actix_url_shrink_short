use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use redis::aio::Connection;
use redis::{AsyncCommands, RedisError};

pub async fn rand_hex_str() -> String {
    let rand_string: Vec<u8> = thread_rng().sample_iter(&Alphanumeric).take(20).collect();

    String::from_utf8(rand_string).unwrap()
}

pub async fn get_redis_string_by_key(
    conn: &mut Connection,
    key: &String,
) -> Result<String, RedisError> {
    conn.get::<_, String>(key).await
}

pub async fn set_redis_string(
    conn: &mut Connection,
    key: &String,
    value: &String,
    ttl: usize,
) -> Result<(), RedisError> {
    conn.pset_ex::<_, _, ()>(key, value, ttl * 1000).await
}
