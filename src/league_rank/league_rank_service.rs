use diesel::MysqlConnection;
use crate::configure::db_configure::DbError;
use crate::league_rank::league_rank::RankRes;

pub fn find_all_ranks(
    conn: &mut MysqlConnection,
    page: i64,
    size: i64
) -> Result<Option<Vec<RankRes>>, DbError> {
    Ok(Some(Vec::new()))
}