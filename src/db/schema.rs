// @generated automatically by Diesel CLI.

diesel::table! {
    task (id) {
        id -> Integer,
        title -> Text,
        created_at -> Timestamp,
        completed -> Bool,
    }
}
