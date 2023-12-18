/*
use crate::model::count::Count;
use crate::repository::update_repository;

use actix_web::{
    post,
    Responder,Result,
    web
};
use aws_sdk_dynamodb::model::AttributeValue;

#[post("/update/count")]
pub async fn update_count() -> Result<impl Responder> {
    let res = update_repository::update_count().await;

    let mut cnt = Count {
        date: " ".to_string(),
        count: " ".to_string(),
        user_name: " ".to_string(),
    };

    for output in res.into_iter() {
        if let Some(item) = output.attributes {
            if let (Some(AttributeValue::S(date)),
                    Some(AttributeValue::S(count)),
                    Some(AttributeValue::S(user_name))
                ) =
                (item.get("date"), item.get("count"), item.get("user_name"))
            {
                cnt.date = date.to_string();
                cnt.count = count.to_string();
                cnt.user_name = user_name.to_string();
            }
        }
    }

    Ok(web::Json(cnt))
}
*/
