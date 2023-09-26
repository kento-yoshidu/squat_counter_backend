use crate::repo::add_item;

use dotenv::dotenv;
use std::env;

use aws_sdk_dynamodb::{Client, model::AttributeValue};

#[allow(unused)]
use actix_web::{
    get, post, web,
    web::{Data, Json, Path},
    Responder, HttpResponse,
    Result
};

#[allow(unused)]
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(Serialize, FromRow, Debug)]
struct User {
    Id: String,
    Name: String,
}

#[derive(Debug, Serialize)]
#[allow(unused)]
pub enum ApiErrorResponse {
    AuthenticationFailure,
    NeedsAuthentication,
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

    println!("response = {:?}", resp);

    for item in resp.items.unwrap_or_default() {
        let mut users: Vec<User> = Vec::new();

        if let (Some(AttributeValue::N(id)), Some(AttributeValue::S(name))) =
            (item.get("Id"), item.get("Name"))
        {
            println!("title: '{}', content: '{}'", id, name)
        }
    }

    let mut users: Vec<User> = Vec::new();

            /*
    for item in resp.items {
        for i in item {
            println!("{:?}", i.get("Name"));
        }

            let user = User {
                Id: i.get("Id").unwrap(),
                Name: i.get("Name").unwrap(),
            }
    }
            */

    //let user = User { id: 1, name: String::from("hoge") };

    Ok(web::Json(users))
}

#[post("/add")]
pub async fn add() -> Result<impl Responder> {
    let resp = add_item::add().await;

    println!("resop = {:?}", resp);

    Ok(web::Json( "status" ))
}
