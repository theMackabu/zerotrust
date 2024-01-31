diesel::table! {
    login_history (id) {
        id -> Integer,
        user_id -> Integer,
        login_timestamp -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        admin -> Bool,
        username -> Text,
        email -> Text,
        password -> Text,
        providers -> Text,
        services -> Text,
        login_session -> Text,
    }
}

diesel::joinable!(login_history -> users (user_id));
diesel::allow_tables_to_appear_in_same_query!(login_history, users,);
