use crate::repository::fetch_item;
use crate::model::user::User;
use crate::model::count::Count;

use actix_web::{
    get, web,
    Responder,
    Result
};

use serde::Serialize;
// use sqlx::{self, FromRow};

#[derive(Debug, Serialize)]
pub struct ApiResponseBody {
    pub success: bool,
    pub errcode: u16,
    pub message: String,
}

#[get("/fetch")]
pub async fn fetch_user() -> Result<impl Responder> {
    fetch_item::fetch_user().await;

    /*
    for item in resp.items.unwrap_or_default() {
        if let (Some(AttributeValue::N(id)), Some(AttributeValue::S(name))) =
            (item.get("Id"), item.get("Name"))
        {
            users.push(User::new(id, name))
        }
    }
    */

    let users: Vec<User> = Vec::new();

    Ok(web::Json(users))
}

#[get("/fetch/count")]
pub async fn fetch_count() -> Result<impl Responder> {
    fetch_item::fetch_count().await;

    let counts: Vec<Count> = Vec::new();

    Ok(web::Json(counts))
}
