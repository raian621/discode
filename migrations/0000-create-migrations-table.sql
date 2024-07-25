CREATE TABLE migrations (
    id   SERIAL       PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL
);

INSERT INTO migrations (name) VALUES ('0000-create-migrations-table.sql');