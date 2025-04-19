// @generated automatically by Diesel CLI.

diesel::table! {
    profile_activities (viewer_id) {
        viewer_id -> Uuid,
        activity_count -> Int4,
    }
}

diesel::table! {
    profile_likes (viewer_id, profile_id) {
        viewer_id -> Uuid,
        profile_id -> Uuid,
    }
}

diesel::table! {
    profile_superlikes (viewer_id, profile_id) {
        viewer_id -> Uuid,
        profile_id -> Uuid,
    }
}

diesel::table! {
    profile_views (viewer_id, profile_id) {
        viewer_id -> Uuid,
        profile_id -> Uuid,
    }
}

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
        location -> Text,
        gender -> Text,
        interests -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    profile_activities,
    profile_likes,
    profile_superlikes,
    profile_views,
    profiles,
);
