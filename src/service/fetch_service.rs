use crate::repository::fetch_repository;
use crate::model::user::User;
use crate::model::count::Count;

use aws_sdk_dynamodb::types::AttributeValue;
use actix_web::{
    get, web,
    Responder,
    Result
};
use serde::Serialize;
use chrono::{Local, Datelike};

#[derive(Debug, Serialize)]
pub struct ApiResponseBody {
    pub success: bool,
    pub errcode: u16,
    pub message: String,
}

#[get("/fetch/user")]
pub async fn fetch_user() -> Result<impl Responder> {
    let result = fetch_repository::fetch_user().await;

    let mut users: Vec<User> = Vec::new();

    for output in result.into_iter() {
        for item in output.items.unwrap_or_default() {
            if let (Some(AttributeValue::S(id)),
                    Some(AttributeValue::S(name))) =
                (item.get("id"), item.get("name"))
            {
                users.push(User::new(id, name))
            }
        }
    }

    Ok(web::Json(users))
}

#[get("/fetch/count")]
pub async fn fetch_count() -> Result<impl Responder> {
    let resp = fetch_repository::fetch_count().await;

    let mut counts: Vec<Count> = Vec::new();

    for scan_output in resp.into_iter() {
        for item in scan_output.items.unwrap_or_default() {
            if let (Some(AttributeValue::S(date)),
                    Some(AttributeValue::S(count)),
                    Some(AttributeValue::S(user_name)),
                ) =
                (item.get("date"), item.get("count"), item.get("user_name"))
            {
                counts.push(Count::new(date, count, &user_name))
            }
        }
    }

    Ok(web::Json(counts))
}

/*
#[get("/fetch/today")]
pub async fn fetch_today() -> Result<impl Responder> {
    let current_date = Local::now();
    let year = current_date.year();
    let month = current_date.month();
    let day = current_date.day();
    let date = format!("{}-{:02}-{:02}", year, month, day);

    let resp = fetch_repository::fetch_today(&date).await;

    let mut cnt = Count {
        date: " ".to_string(),
        count: " ".to_string(),
        user_name: " ".to_string(),
    };

    for scan_output in resp.into_iter() {
        for item in scan_output.items.unwrap_or_default() {
            if let (Some(AttributeValue::S(date)),
                    Some(AttributeValue::S(count)),
                    Some(AttributeValue::S(user_name)),
            ) =
            (item.get("date"), item.get("count"), item.get("user_name"))
        {
            cnt.date = date.to_string();
            cnt.count = count.to_string();
            cnt.user_name = user_name.to_string();
        }
        }
    }

    Ok(web::Json(cnt))
}
*/
