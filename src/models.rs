use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Selectable, Queryable)]
#[diesel(table_name = crate::schema::reviews)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Review {
    pub id: i32,
    pub weekly_salary: f32,
    pub weekly_tips: Option<f32>,
    pub shift_days_count: i32,
    pub shift_duration: i32,
    pub social_security: Option<bool>,
}
