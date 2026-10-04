use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

use crate::{api::handlers, app_state::AppState, config::Config};

pub async fn run() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::from_env();
    let bind_addr = format!("{}:{}", config.server_host, config.server_port);

    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .map_err(|err| std::io::Error::other(format!("database connection failed: {err}")))?;

    let app_state = web::Data::new(AppState { db: db_pool });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(handlers::health)
            .service(handlers::health_db)
    })
    .bind(bind_addr)?
    .run()
    .await
}