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
    }
}

diesel::joinable!(reviews -> places (place_id));

diesel::allow_tables_to_appear_in_same_query!(places, reviews,);
