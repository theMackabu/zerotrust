use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

table! {
    login_history (id) {
        id -> Integer,
        user_id -> Integer,
        login_timestamp -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        login_session -> Varchar,
        providers -> Varchar,
        services -> Varchar,
    }
}

joinable!(login_history -> users (user_id));
allow_tables_to_appear_in_same_query!(login_history, users);
