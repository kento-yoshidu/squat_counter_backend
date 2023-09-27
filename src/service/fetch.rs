use dotenv::dotenv;
use std::env;

use aws_sdk_dynamodb::{Client, model::AttributeValue};

use actix_web::{
    get, web,
    Responder,
    Result
};

use serde::Serialize;
use sqlx::{self, FromRow};

#[derive(Serialize, FromRow, Debug)]
struct User {
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
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    dotenv().ok();

    let table_name = env::var("TABLE_NAME").unwrap();

    let resp = client.scan().table_name(table_name).send().await.unwrap();

    let mut users: Vec<User> = Vec::new();

    for item in resp.items.unwrap_or_default() {
        if let (Some(AttributeValue::N(id)), Some(AttributeValue::S(name))) =
            (item.get("Id"), item.get("Name"))
        {
            users.push(User::new(id, name))
        }
    }

    Ok(web::Json(users))
}
