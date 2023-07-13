use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{places, reviews, sessions, users};

#[derive(Serialize, Selectable, Queryable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = places)]
pub struct Place {
    pub id: i32,
    pub name: String,
    pub address: Option<String>,
    pub maps_url: Option<String>,
}

#[derive(Serialize, Selectable, Queryable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Place))]
#[diesel(table_name = reviews)]
pub struct Review {
    pub id: i32,
    pub place_id: i32,
    pub weekly_salary: f32,
    pub weekly_tips: Option<f32>,
    pub shift_days_count: i32,
    pub shift_duration: i32,
    pub social_security: Option<bool>,
}

#[derive(Deserialize, Serialize, Selectable, Queryable, Identifiable, Debug, PartialEq, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub email: String,
}

#[derive(Serialize, Selectable, Queryable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(User))]
#[diesel(table_name = sessions)]
pub struct Session {
    pub id: i32,
    pub user_id: Option<i32>,
    pub session_token: Option<String>,
    pub access_token: Option<String>,
}
