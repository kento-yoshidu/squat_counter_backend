use dotenv::dotenv;
use std::env;

use aws_sdk_dynamodb::{Client, model::AttributeValue};

#[allow(unused)]
pub async fn add_user() -> Result<aws_sdk_dynamodb::output::PutItemOutput, aws_sdk_dynamodb::SdkError<aws_sdk_dynamodb::error::PutItemError>> {
    dotenv().ok();

    let table_name = env::var("TABLE_NAME_USER").unwrap();

    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let user_id = AttributeValue::N("6".to_string());
    let user_name = AttributeValue::S("foobar".to_string());

    let request = client
        .put_item()
        .table_name(table_name)
        .item("Id", user_id)
        .item("Name", user_name);

    let resp = request.send().await;

    resp
}

pub async fn add_count(id: &String, date: &String, count: &String, user_name: &String) -> Result<aws_sdk_dynamodb::output::PutItemOutput, aws_sdk_dynamodb::SdkError<aws_sdk_dynamodb::error::PutItemError>> {
    dotenv().ok();

    let table_name = env::var("TABLE_NAME_COUNT").unwrap();

    println!("teble_name = {}", table_name);

    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let id = AttributeValue::S(id.to_string());
    let date = AttributeValue::S(date.to_string());
    let count = AttributeValue::S(count.to_string());
    let user_name = AttributeValue::S(user_name.to_string());

    let request = client
        .put_item()
        .table_name(table_name)
        .item("id", id)
        .item("date", date)
        .item("count", count)
        .item("user_name", user_name);

    let resp = request.send().await;

    resp
}
/*
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error};

pub async fn add_item(client: &Client, item: Item, table: &String) -> Result<ItemOut, Error> {
    let user_av = AttributeValue::S(item.username);
    let type_av = AttributeValue::S(item.p_type);
    let age_av = AttributeValue::S(item.age);
    let first_av = AttributeValue::S(item.first);
    let last_av = AttributeValue::S(item.last);

    let request = client
        .put_item()
        .table_name(table)
        .item("username", user_av)
        .item("account_type", type_av)
        .item("age", age_av)
        .item("first_name", first_av)
        .item("last_name", last_av);

    println!("Executing request [{request:?}] to add item...");

    let resp = request.send().await?;

    let attributes = resp.attributes().unwrap();

    let username = attributes.get("username").cloned();
    let first_name = attributes.get("first_name").cloned();
    let last_name = attributes.get("last_name").cloned();
    let age = attributes.get("age").cloned();
    let p_type = attributes.get("p_type").cloned();

    println!(
        "Added user {:?}, {:?} {:?}, age {:?} as {:?} user",
        username, first_name, last_name, age, p_type
    );

    Ok(ItemOut {
        p_type,
        age,
        username,
        first_name,
        last_name,
    })
}
*/
