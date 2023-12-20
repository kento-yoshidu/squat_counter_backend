use crate::model::user::User;
use crate::model::count::{Count, self};

use chrono::format::Item;
use dotenv::dotenv;
use std::env;

use aws_sdk_dynamodb::{Client, types::AttributeValue, Error};
use aws_sdk_dynamodb::operation::put_item::PutItemOutput;

/*
pub async fn add_user(user: &User) -> Result<aws_sdk_dynamodb::output::PutItemOutput, aws_sdk_dynamodb::SdkError<aws_sdk_dynamodb::error::PutItemError>> {
    dotenv().ok();

    let table_name = env::var("TABLE_NAME_USER").unwrap();

    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let id = AttributeValue::S(user.id.to_string());
    let name = AttributeValue::S(user.name.to_string());

    let request = client
        .put_item()
        .table_name(table_name)
        .item("id", id)
        .item("name", name);

    let resp = request.send().await;

    resp
}
*/

#[derive(Debug, PartialEq)]
pub struct ItemOut {
    pub date: Option<AttributeValue>,
    pub count: Option<AttributeValue>,
    pub username: Option<AttributeValue>,
}

pub async fn add_count(count: &Count) -> Result<(), Error> {
    dotenv().ok();

    let table_name = env::var("TABLE_NAME_COUNT").unwrap();

    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let username = &count.user_name;

    let date = AttributeValue::S(count.date.to_string());
    let count = AttributeValue::S(count.count.to_string());
    let user_name = AttributeValue::S(username.to_string());

    let request = client
        .put_item()
        .table_name(table_name)
        .item("date", date)
        .item("count", count)
        .item("user_name", user_name);

    let resp = request.send().await?;

    /*
    let attributes = resp.attributes().unwrap();

    let date = attributes.get("date").cloned();
    let count = attributes.get("count").cloned();
    let username = attributes.get("user_name").cloned();
    */

    Ok(())
}
