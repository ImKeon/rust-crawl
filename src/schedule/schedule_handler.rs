use actix_web::{web, Error, HttpResponse};
use actix_web::web::Data;
use crate::configure::db_configure::DbPool;
use crate::configure::model::PageInfo;
use crate::schedule::schedule_service;

pub async fn get_schedule(
    page_info: web::Query<PageInfo>,
    pool: Data<DbPool>
) -> Result<HttpResponse, Error> {
    print!("]-----] schedule_handler::get_schedule [-----[");
    let schedules = web::block(move || {
        let mut conn = pool.get()?;
        schedule_service::find_all_schedules(&mut conn, page_info.page, page_info.size)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(schedules) = schedules {
        Ok(HttpResponse::Ok().json(schedules))
    } else {
        let res = HttpResponse::NotFound().body(format!("No schedule found"));
        Ok(res)
    }
}