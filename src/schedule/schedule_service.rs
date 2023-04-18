use diesel::MysqlConnection;
use crate::configure::db_configure::DbError;
use crate::schedule::schedule::ScheduleRes;

pub fn find_all_schedules(
    conn: &mut MysqlConnection,
    page: i64,
    size: i64
) -> Result<Option<Vec<ScheduleRes>>, DbError> {
    Ok(Some(Vec::new()))
}