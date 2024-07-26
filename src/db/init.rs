use std::env;

use sqlx::postgres::PgPool;

struct DbConfig {
    host: String,
    port: i32,
    user: String,
    password: String,
    db_name: String,
    // use_tls: bool,
}

pub async fn init() -> Result<PgPool, sqlx::Error> {
    let db_url = build_database_url(DbConfig {
        host: env::var("DB_HOST").unwrap_or("localhost".to_string()),
        port: match env::var("DB_PORT") {
            Ok(port_str) => port_str.parse().unwrap(),
            Err(_) => 5433,
        },
        user: env::var("DB_USER").unwrap(), 
        password: env::var("DB_PASSWORD").unwrap(),
        db_name: env::var("DB_NAME").unwrap()
    });

    tracing::info!("Connecting to PostgreSQL database...");
    let pool = PgPool::connect(db_url.as_str()).await?;
    tracing::info!("Connection successful!");

    Ok(pool)
}

fn build_database_url(config: DbConfig) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user,
        config.password,
        config.host,
        config.port,
        config.db_name
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build_database_url() {
        let url = build_database_url(DbConfig{
            host: "localhost".to_string(),
            port: 5433,
            user: "postgres_user".to_string(),
            password: "postgres_password".to_string(),
            db_name: "postgres_db".to_string()
        });

        assert_eq!("postgres://postgres_user:postgres_password@localhost:5433/postgres_db", url);
    }
}