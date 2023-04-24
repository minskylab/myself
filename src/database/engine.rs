use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn new_postgres_pool(database_url: String) -> PgPool {
    // let database_url = std::env::var("DATABASE_URL").unwrap_or("sqlite://sqlite.db".to_string());
    PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .unwrap()
}
