use actix_web::web;
use crate::{league_rank, schedule};

pub fn router_configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(league_rank::league_rank_router::rank_route);
    cfg.configure(schedule::schedule_router::schedule_route);
}