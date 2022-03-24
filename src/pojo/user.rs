#![allow(non_snake_case)]

use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Url {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertUrl {
    pub url_name: String,
    pub url_target: String,
    pub url_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SelectUrl {
    pub url_id: i64,
    pub url_name: String,
    pub url_target: String,
    pub url_time: DateTime<Utc>,
}