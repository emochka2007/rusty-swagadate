// @generated automatically by Diesel CLI.

diesel::table! {
    profiles (id) {
        id -> Uuid,
        user_id -> Int8,
        username -> Text,
        created_at -> Timestamp,
        description -> Text,
        file_ids -> Nullable<Array<Nullable<Text>>>,
        displayed_name -> Text,
        age -> Int4,
    }
}
