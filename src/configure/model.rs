use serde::{Deserialize};

#[derive(Deserialize)]
pub struct PageInfo {
    pub page: i64,
    pub size: i64,
}