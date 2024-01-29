use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

table! {
    login_history (id) {
        id -> Int4,
        user_id -> Int4,
        login_timestamp -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        login_session -> Varchar,
        providers -> Json,
    }
}

joinable!(login_history -> users (user_id));
allow_tables_to_appear_in_same_query!(login_history, users);
