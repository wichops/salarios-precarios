// @generated automatically by Diesel CLI.

diesel::table! {
    places (id) {
        id -> Int4,
        #[max_length = 80]
        name -> Varchar,
        address -> Nullable<Text>,
        maps_url -> Nullable<Text>,
    }
}

diesel::table! {
    reviews (id) {
        id -> Int4,
        weekly_salary -> Float4,
        weekly_tips -> Nullable<Float4>,
        shift_days_count -> Int4,
        shift_duration -> Int4,
        social_security -> Nullable<Bool>,
        place_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        session_token -> Nullable<Text>,
        access_token -> Nullable<Text>,
        user_id -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Text,
    }
}

diesel::joinable!(reviews -> places (place_id));
diesel::joinable!(reviews -> users (user_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(places, reviews, sessions, users,);
