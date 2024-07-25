CREATE TABLE leetcode_connections (
    id                SERIAL       PRIMARY KEY,
    leetcode_username VARCHAR(255) UNIQUE NOT NULL,
    discord_id        BIGINT       UNIQUE NOT NULL
);
