-- Add migration script here
CREATE TABLE connect_tokens (
    id         SERIAL  PRIMARY KEY,
    discord_id BIGINT  UNIQUE NOT NULL,
    token      CHAR(8) NOT NULL
);