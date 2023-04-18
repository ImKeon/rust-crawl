use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::league_rank;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, PartialEq, Insertable, Identifiable)]
#[diesel(primary_key(id))]
#[diesel(table_name = league_rank)]
pub struct LeagueRank {
    pub id: Option<i64>,
    pub created_at: Option<i64>,
    pub created_id: Option<i64>,
    pub updated_at: Option<i64>,
    pub updated_id: Option<i64>,
    pub issued_date: Option<i64>,
    pub subject_rank: Option<i32>,
    pub rank_title: Option<String>,
    pub subject_name: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankReq {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankRes {

}