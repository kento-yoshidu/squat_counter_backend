use crate::repository::fetch_item;
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
    let result = fetch_item::fetch_user().await;

    let mut users: Vec<User> = Vec::new();

    for output in result.into_iter() {
        for item in output.items.unwrap_or_default() {
            if let (Some(AttributeValue::N(id)), Some(AttributeValue::S(name))) =
                (item.get("Id"), item.get("Name"))
            {
                users.push(User::new(id, name))
            }
        }
    }

    Ok(web::Json(users))
}

#[get("/fetch/count")]
pub async fn fetch_count() -> Result<impl Responder> {
    let resp = fetch_item::fetch_count().await;

    let mut counts: Vec<Count> = Vec::new();

    for scan_output in resp.into_iter() {
        for item in scan_output.items.unwrap_or_default() {
            if let (Some(AttributeValue::S(id)), Some(AttributeValue::S(date)), Some(AttributeValue::S(count))) =
                (item.get("id"), item.get("date"), item.get("count"))
            {
                counts.push(Count::new(id, date, count))
            }
        }
    }

    Ok(web::Json(counts))
}
