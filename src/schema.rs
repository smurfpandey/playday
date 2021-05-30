table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        last_login -> Nullable<Timestamp>,
    }
}
