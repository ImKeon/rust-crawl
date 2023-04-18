use actix_web::web;
use crate::schedule::schedule_handler;

pub fn schedule_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/schedule")
            .service(web::resource("")
                .route(web::get().to(schedule_handler::get_schedule))
            )
    );
}