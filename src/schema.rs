// @generated automatically by Diesel CLI.

diesel::table! {
    reviews (id) {
        id -> Int4,
        weekly_salary -> Float4,
        weekly_tips -> Nullable<Float4>,
        shift_days_count -> Int4,
        shift_duration -> Int4,
        social_security -> Nullable<Bool>,
    }
}
