use sqlx::PgPool;

pub struct LeetCodeConnection {
    pub leetcode_username: String,
    pub discord_id: i64,
}

impl LeetCodeConnection {
    pub async fn insert(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"INSERT INTO leetcode_connections (leetcode_username, discord_id) VALUES ($1, $2)"#,
            self.leetcode_username,
            self.discord_id,
        ).execute(pool).await?;
        
        Ok(())
    }

    pub async fn find_with_discord_id(pool: &PgPool, discord_id: i64) -> Result<Self, sqlx::Error> {
        let leetcode_username = sqlx::query_scalar(
            r#"SELECT leetcode_username FROM leetcode_connections WHERE discord_id=$1"#,
        )
        .bind(discord_id)
        .fetch_one(pool).await?;

        Ok(Self {
            leetcode_username,
            discord_id,
        })
    }
}