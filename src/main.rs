mod schema;
mod models;
mod routes;
mod db;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    info!("Starting server. . .");

    let pool = db::create_db_pool();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .route("/users", web::post().to(routes::user_routes::create_user))
                    .route("/users", web::get().to(routes::user_routes::get_users))
                    .route("/users/{id}", web::get().to(routes::user_routes::get_user))
                    .route("/users/{id}", web::put().to(routes::user_routes::update_user))
                    .route("/users/{id}", web::delete().to(routes::user_routes::delete_user))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}