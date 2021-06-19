-- Your SQL goes here
CREATE TABLE wished_games (
    id UUID PRIMARY KEY,
    title VARCHAR NOT NULL,
    igdb_info JSONB NOT NULL,
    added_on TIMESTAMP WITHOUT TIME ZONE  NOT NULL
);
