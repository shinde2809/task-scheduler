use actix_web::{HttpResponse, Responder, get, web};

use crate::app_state::AppState;

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[get("/health/db")]
pub async fn health_db(state: web::Data<AppState>) -> impl Responder {
    match sqlx::query_scalar::<_, i64>("SELECT 1")
        .fetch_one(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().body("db ok"),
        Err(err) => HttpResponse::ServiceUnavailable().body(format!("db error: {err}")),
    }
}


