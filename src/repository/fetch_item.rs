use dotenv::dotenv;
use std::env;

use aws_sdk_dynamodb::{Client, SdkError};

pub async fn fetch_user() -> Result<aws_sdk_dynamodb::output::ScanOutput, SdkError<aws_sdk_dynamodb::error::ScanError>> {
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

pub async fn fetch_count() -> Result<aws_sdk_dynamodb::output::ScanOutput, SdkError<aws_sdk_dynamodb::error::ScanError>> {
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
