use actix_web::web;

use super::league_rank_handler;

pub fn rank_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/league_rank")
            .service(web::resource("")
            .route(web::get().to(league_rank_handler::get_rank))
        )
    );
}