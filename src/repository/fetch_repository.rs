use dotenv::dotenv;
use std::env;

use aws_sdk_dynamodb::{Client, Error, types::AttributeValue};
use aws_sdk_dynamodb::operation::scan::ScanOutput;

pub async fn fetch_user() -> Result<ScanOutput, Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    dotenv().ok();

    let table_name = env::var("TABLE_NAME_USER").unwrap();

    let resp = client
        .scan()
        .table_name(table_name)
        .send()
        .await
        .unwrap();

    Ok(resp)
}

pub async fn fetch_count() -> Result<ScanOutput, Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    dotenv().ok();

    let table_name = env::var("TABLE_NAME_COUNT").unwrap();

    let resp = client
        .scan()
        .table_name(table_name)
        .send()
        .await
        .unwrap();

    Ok(resp)
}

/*
pub async fn fetch_today(date: &String) -> Result<aws_sdk_dynamodb::output::ScanOutput, SdkError<aws_sdk_dynamodb::error::ScanError>> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    dotenv().ok();

    let table_name = env::var("TABLE_NAME_COUNT").unwrap();

    let resp = client
        .scan()
        .table_name(table_name)
        .filter_expression("#filter_key = :val".to_string())
        .expression_attribute_names(
            "#filter_key".to_string(),
            "date".to_string())
        .expression_attribute_values(
            ":val".to_string(),
            AttributeValue::S(date.to_string())
        )
        .send()
        .await
        .unwrap();

    Ok(resp)
}
*/
