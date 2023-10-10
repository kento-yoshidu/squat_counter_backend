use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Count {
    pub date: String,
    pub count: String,
    pub user_name: String,
}

impl Count {
    pub fn new(date: &String, count: &String, user_name: &String) -> Count {
        return Count {
            date: date.to_string(),
            count: count.to_string(),
            user_name: user_name.to_string(),
        };
    }
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct CountRequest {
    pub count: usize,
    pub user_name: String,
}
