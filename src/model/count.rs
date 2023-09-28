use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
pub struct Count {
    id: String,
    date: String,
    count: String,
}

impl Count {
    pub fn new(id: &String, date: &String, count: &String) -> Count {
        return Count {
            id: id.to_string(),
            date: date.to_string(),
            count: count.to_string(),
        };
    }
}
