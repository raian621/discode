use std::{collections::HashSet, fs, path::Path};
use sqlx::postgres::PgPool;

#[derive(Debug)]
pub enum MigrationError {
    Sqlx(sqlx::Error),
    Io(std::io::Error),
    Basic(String)
}

#[derive(Debug)]
struct Migration {
    name: String
}

pub async fn apply_migrations(pool: &PgPool) -> Result<(), MigrationError> {
    let query_result: Result<Vec<Migration>, sqlx::Error> = sqlx::query_as!(Migration, "SELECT name FROM migrations").fetch_all(pool).await;
    let applied_migrations = match query_result {
        Ok(migrations) => HashSet::<String>::from_iter(migrations.into_iter().map(|m| m.name)),
        Err(why) => return Err(MigrationError::Sqlx(why))
    };
    let mut migrations = match fs::read_dir("migrations") {
        Ok(entries) => entries.into_iter().filter(|entry_result| {
            match entry_result {
                Ok(_) => true,
                Err(_) => false
            }
        })
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<String>>(),
        Err(why) => return Err(MigrationError::Io(why))
    };
    migrations.sort();

    for migration in migrations.iter() {
        if !applied_migrations.contains(migration) {
            let sql_query = match fs::read_to_string(Path::new("migrations").join(migration)) {
                Ok(content) => content,
                Err(why) => return Err(MigrationError::Io(why))
            };
            apply_migration(pool, sql_query, migration).await?
        } else {
            println!("Skipping applied migration `{}`", migration);
        }
    }

    Ok(())
}

async fn apply_migration(pool: &PgPool, query: String, migration: &String) -> Result<(), MigrationError> {
    tracing::info!("Applying migration {}", migration);
    let mut conn = match pool.begin().await {
        Ok(conn) => conn,
        Err(why) => return Err(MigrationError::Sqlx(why))
    };
    match sqlx::query(&query).execute(&mut *conn).await {
        Ok(_) => (),
        Err(why) => return Err(MigrationError::Sqlx(why))
    };
    match sqlx::query!(r#"INSERT INTO migrations (name) VALUES ($1)"#, migration).execute(&mut *conn).await {
        Ok(_) => (),
        Err(why) => return Err(MigrationError::Sqlx(why))
    }
    match conn.commit().await {
        Ok(_) => Ok(()),
        Err(why) => Err(MigrationError::Sqlx(why))
    }
}