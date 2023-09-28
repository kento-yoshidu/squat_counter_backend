use crate::repository::fetch_item;

use actix_web::{
    get, web,
    Responder,
    Result
};

use serde::Serialize;
use sqlx::{self, FromRow};

#[derive(Serialize, FromRow, Debug)]
pub struct User {
    id: String,
    name: String,
}

impl User {
    fn new(id: &String, name: &String) -> User {
        return User {
            id: id.to_string(),
            name: name.to_string(),
        };
    }
}

#[derive(Debug, Serialize)]
pub struct ApiResponseBody {
    pub success: bool,
    pub errcode: u16,
    pub message: String,
}

#[get("/fetch")]
pub async fn fetch() -> Result<impl Responder> {
    fetch_item::fetch_item().await;

    /*
    for item in resp.items.unwrap_or_default() {
        if let (Some(AttributeValue::N(id)), Some(AttributeValue::S(name))) =
            (item.get("Id"), item.get("Name"))
        {
            users.push(User::new(id, name))
        }
    }
    */

    let mut users: Vec<User> = Vec::new();

    Ok(web::Json(users))
}
