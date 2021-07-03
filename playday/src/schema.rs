table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        last_login -> Timestamp,
    }
}

table! {
    wished_games (id) {
        id -> Uuid,
        title -> Varchar,
        igdb_id -> Int4,
        igdb_info -> Jsonb,
        added_on -> Timestamp,
        user_id -> Uuid,
        pc_release_date -> Int8,
    }
}

joinable!(wished_games -> users (user_id));

allow_tables_to_appear_in_same_query!(
    users,
    wished_games,
);
