use std::{time::Duration};
use actix_web::{App, HttpServer, web, middleware, HttpResponse};
use diesel::{MysqlConnection, r2d2};
use diesel::r2d2::ConnectionManager;
use crawl_server::router_configure::router_configure;

mod configure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    let connection_timeout = Duration::new(10, 0);
    let pool = r2d2::Pool::builder()
        .max_size(100)
        .connection_timeout(connection_timeout)
        .build(manager)
        .expect("]-----] Fail to create pool [-----[");
    log::info!("]-----] Starting Server at 127.0.0.1:8080 [-----[");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(router_configure)
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("welcome!") }),
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
