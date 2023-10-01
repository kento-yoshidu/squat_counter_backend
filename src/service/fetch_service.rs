use crate::repository::fetch_repository;
use crate::model::user::User;
use crate::model::count::Count;

use aws_sdk_dynamodb::model::AttributeValue;
use actix_web::{
    get, web,
    Responder,
    Result
};
use serde::Serialize;

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
            if let (Some(AttributeValue::S(id)),
                    Some(AttributeValue::S(date)),
                    Some(AttributeValue::S(count)),
                    Some(AttributeValue::S(user_name)),
                ) =
                (item.get("id"), item.get("date"), item.get("count"), item.get("user_name"))
            {
                counts.push(Count::new(id, date, count, &user_name))
            }
        }
    }

    Ok(web::Json(counts))
}