use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sqlx::PgPool;

pub struct ConnectionToken {
    pub discord_id: i64,
    pub token: String,
}

impl ConnectionToken {
    pub fn new(discord_id: i64) -> Self {
        Self {
            discord_id,
            token: generate_random_token(8)
        }
    }

    pub async fn insert(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"INSERT INTO connect_tokens (discord_id, token) VALUES ($1, $2)"#,
            self.discord_id,
            self.token,
        ).execute(pool).await?;

        Ok(())
    }

    pub async fn delete(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"DELETE FROM connect_tokens WHERE discord_id=$1"#,
            self.discord_id,
        ).execute(pool).await?;

        Ok(())
    }
}

pub async fn get_connection_token_by_id(pool: &PgPool, discord_id: i64) -> ConnectionToken {
    let result = sqlx::query_as!(
        ConnectionToken,
        r#"SELECT discord_id, token FROM connect_tokens WHERE discord_id=$1"#,
        discord_id
    ).fetch_one(pool).await;

    match result {
        Err(_why) => {
            let connection_token = ConnectionToken::new(discord_id);
            connection_token.insert(pool).await.unwrap();
            connection_token
        },
        Ok(connection_token) => connection_token
    }
}

fn generate_random_token(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_random_token() {
        let random_token = generate_random_token(8);
        assert_eq!(random_token.len(), 8);
    }

    #[test]
    fn test_new_connect_token() {
        let discord_id = 12344567;
        let connect_token = ConnectionToken::new(discord_id);

        assert_eq!(connect_token.discord_id, discord_id);
        assert_eq!(connect_token.token.len(), 8);
    }
}