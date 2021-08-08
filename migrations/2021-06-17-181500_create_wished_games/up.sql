-- Your SQL goes here
CREATE TABLE wished_games (
    id UUID PRIMARY KEY,
    title VARCHAR NOT NULL,
    igdb_id INTEGER NOT NULL,
    igdb_info JSONB NOT NULL,
    added_on TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    user_id UUID NOT NULL,

    UNIQUE(igdb_id, user_id),
    CONSTRAINT fk_users
        FOREIGN KEY(user_id)
            REFERENCES users(id)
            ON DELETE CASCADE
);
