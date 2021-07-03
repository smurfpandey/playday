-- Your SQL goes here
ALTER TABLE wished_games
ADD COLUMN pc_release_date BIGINT DEFAULT 0 NOT NULL;
