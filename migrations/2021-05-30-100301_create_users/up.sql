-- Your SQL goes here
CREATE TABLE users (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE  NOT NULL,
    last_login TIMESTAMP WITHOUT TIME ZONE NOT NULL,

    UNIQUE(email)
);
