-- Your SQL goes here
CREATE TABLE game_stores (
    id UUID PRIMARY KEY,
    store_name VARCHAR NOT NULL,
    store_token JSONB NOT NULL,
    added_on TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    updated_on TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    user_id UUID NOT NULL,

    CONSTRAINT fk_users
        FOREIGN KEY(user_id)
            REFERENCES users(id)
            ON DELETE CASCADE
);
