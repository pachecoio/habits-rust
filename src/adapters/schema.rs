// @generated automatically by Diesel CLI.

diesel::table! {
    habit_events (id) {
        id -> Nullable<Integer>,
        habit_id -> Integer,
        user_id -> Integer,
        CREATED_AT -> Timestamp,
    }
}

diesel::table! {
    habits (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
        last_name -> Text,
    }
}

diesel::joinable!(habit_events -> habits (habit_id));
diesel::joinable!(habit_events -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    habit_events,
    habits,
    users,
);
