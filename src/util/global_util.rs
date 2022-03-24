use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub async fn rand_hex_str() -> String {
    let rand_string: Vec<u8> = thread_rng().sample_iter(&Alphanumeric).take(20).collect();

    String::from_utf8(rand_string).unwrap()
}
