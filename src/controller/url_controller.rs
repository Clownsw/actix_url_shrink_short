#![allow(unused_assignments)]

use std::sync::Arc;

use crate::dao::url_dao::{
    delete_by_id, insert_url, select_by_name, select_by_target, select_count_by_name,
};

use crate::pojo::app_state::AppState;
use crate::pojo::msg::Msg;
use crate::pojo::user::{InsertUrl, Url};
use crate::util::global_util::{get_redis_string_by_key, rand_hex_str, set_redis_string};
use crate::URL_TIME;
use actix_web::web::Path;
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::Duration;
use chrono::Utc;
use log::info;
use sqlx::Pool;

#[post("/api/add")]
async fn api_add_url(body: String, data: web::Data<AppState>) -> impl Responder {
    let mut msg: Msg = Msg::new();

    let mut url: Url = serde_json::from_str(body.as_str()).unwrap();
    url.url = url.url.trim().to_string();

    let parse_result = url::Url::parse(url.url.as_str());

    // 如果解析错误说明传入的不是一个URL
    if let Err(_) = parse_result {
        msg.code = 500;
        msg.message = String::from("not found url!");
    } else {
        // 查询一下这个URL是否已存在
        let select_result = select_by_target(&data.db_pool, &url.url).await;

        if let Ok(v) = select_result {
            let now = Utc::now();

            // 走到这里说明它存在, 如果它还未过期, 我们直接将它返回即可
            if v.url_time.timestamp_millis() > now.timestamp_millis() {
                msg.code = 200;
                msg.message = String::from(v.url_name);
                return HttpResponse::Ok().body(serde_json::to_string(&msg).unwrap());
            } else {
                // 到这里说明虽然存在但是已经过期了, 那么我们将它删除, 并重新添加
                delete_by_id(&data.db_pool, v.url_id).await;
            }
        }

        // 创建弱引用
        let _db_pool: Arc<Pool<_>> = Arc::new(data.db_pool.clone());
        let _db_pool_weak = Arc::downgrade(&_db_pool);

        let mut rand_url_name = String::new();

        // 创建一个十六进制随机字符串
        // 并对其循环检查是否存在, 如果存在则重新生成, 反之可使用跳出循环
        loop {
            rand_url_name = rand_hex_str().await;

            if select_count_by_name(&_db_pool_weak.upgrade().unwrap().as_ref(), &rand_url_name)
                .await
                <= 0
            {
                break;
            }
        }

        msg.message = String::from(rand_url_name.as_str());

        insert_url(
            &data.db_pool,
            InsertUrl {
                url_name: rand_url_name,
                url_target: url.url,
                url_time: Utc::now().checked_add_signed(Duration::days(1)).unwrap(),
            },
        )
        .await;

        msg.code = 200;
    }

    HttpResponse::Ok().body(serde_json::to_string(&msg).unwrap())
}

#[get("/api/t/{name}")]
async fn api_redierct(path: Path<String>, data: web::Data<AppState>) -> impl Responder {
    let name = path.into_inner();
    let msg = Msg {
        code: 500,
        message: String::from("The short link has expired!"),
    };

    if !name.is_empty() {
        let mut async_conn = data.redis_client.get_async_connection().await.unwrap();

        // 尝试去redis中查询
        if let Ok(v) = get_redis_string_by_key(&mut async_conn, &name).await {
            info!("从缓存中读出 {}", name);

            return HttpResponse::TemporaryRedirect()
                .insert_header(("location", v))
                .finish();
        } else {
            // 如果缓存查不到, 尝试去数据库中查
            if let Ok(v) = select_by_name(&data.db_pool, &name).await {
                // 判断一下是否过期, 如果过期返回错误提示并删除该记录
                if v.url_time.timestamp_millis() > Utc::now().timestamp_millis() {
                    let url_time_ref = URL_TIME.lock().unwrap();

                    // 如果没过期则, 将这个存入redis进行缓存
                    match set_redis_string(
                        &mut async_conn,
                        &name,
                        &v.url_target,
                        *url_time_ref as usize,
                    )
                    .await
                    {
                        Ok(_) => {
                            info!("缓存 {} 成功!", name.clone())
                        }
                        Err(_) => {
                            info!("缓存 {} 失败!", name.clone())
                        }
                    };

                    return HttpResponse::TemporaryRedirect()
                        .insert_header(("location", v.url_target))
                        .finish();
                } else {
                    delete_by_id(&data.db_pool, v.url_id).await;
                }
            }
        }
    }

    HttpResponse::Ok().body(serde_json::to_string(&msg).unwrap())
}
