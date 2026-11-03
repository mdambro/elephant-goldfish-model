CREATE SEQUENCE IF NOT EXISTS users_id_seq;

CREATE TABLE IF NOT EXISTS users (
    id    BIGINT PRIMARY KEY DEFAULT nextval('users_id_seq'),
    email TEXT   NOT NULL UNIQUE
);
