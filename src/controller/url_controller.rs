use crate::dao::url_dao::{delete_by_id, insert_url, select_by_name, select_by_target};
use crate::pojo::app_state::AppState;
use crate::pojo::msg::Msg;
use crate::pojo::user::{InsertUrl, Url};
use crate::util::global_util::rand_hex_str;
use actix_web::web::Path;
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::Duration;
use chrono::Utc;

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

        let rand_url_name = rand_hex_str().await;

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
        if let Ok(v) = select_by_name(&data.db_pool, &name).await {
            return HttpResponse::TemporaryRedirect()
                .insert_header(("location", v.url_target))
                .finish();
        }
    }

    HttpResponse::Ok().body(serde_json::to_string(&msg).unwrap())
}
