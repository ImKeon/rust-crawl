use actix_web::{web, HttpResponse, Error};
use crate::configure::db_configure::DbPool;
use crate::configure::model::PageInfo;
use crate::league_rank::league_rank_service;

pub async fn get_rank(
    pool: web::Data<DbPool>,
    page_info: web::Query<PageInfo>,
) -> Result<HttpResponse, Error> {
    print!("]-----] rank_handler::get_rank [------[");
    let ranks = web::block(move || {
        let mut conn = pool.get()?;
        league_rank_service::find_all_ranks(&mut conn, page_info.page, page_info.size)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(ranks) = ranks {
        Ok(HttpResponse::Ok().json(ranks))
    } else {
        let res = HttpResponse::NotFound().body(format!("]-----] Rank Not Found [-----["));
        Ok(res)
    }
}