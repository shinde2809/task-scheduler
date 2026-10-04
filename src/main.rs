use task_schedular::bootstrap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    bootstrap::startup::run().await
}
