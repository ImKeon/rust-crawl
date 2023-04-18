use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::schedule;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, PartialEq, Insertable, Identifiable)]
#[diesel(primary_key(id))]
#[diesel(table_name = schedule)]
pub struct Schedule {
    pub id: Option<i64>,
    pub created_at: Option<i64>,
    pub created_id: Option<i64>,
    pub updated_at: Option<i64>,
    pub updated_id: Option<i64>,
    pub game_schedule_day: Option<i64>,
    pub home_team: Option<String>,
    pub away_team: Option<String>,
    pub stadium: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleReq {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleRes {

}