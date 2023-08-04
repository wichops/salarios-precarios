use axum_sessions::async_session::CookieStore;
use axum_sessions::extractors::ReadableSession;

use crate::models::*;
use crate::prelude::*;
use crate::schema::*;

#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = reviews)]
pub struct NewReview {
    pub place_id: i32,
    pub user_id: Option<i32>,
    pub weekly_salary: f32,
    pub shift_days_count: i32,
    pub shift_duration: i32,
}

pub async fn create_review_json(
    State(state): State<Context>,
    Json(new_review): Json<NewReview>,
) -> Result<Json<Review>, (StatusCode, String)> {
    let conn = state.pool.get().await.map_err(internal_error)?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(reviews::table)
                .values(new_review)
                .returning(Review::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(res))
}

pub async fn create_review(
    State(state): State<Context>,
    Extension(user): Extension<User>,
    Form(mut new_review): Form<NewReview>,
) -> Result<Redirect, (StatusCode, String)> {
    let conn = state.pool.get().await.map_err(internal_error)?;
    new_review.user_id = Some(user.id);

    conn.interact(|conn| {
        diesel::insert_into(reviews::table)
            .values(new_review)
            .returning(Review::as_returning())
            .get_result(conn)
    })
    .await
    .map_err(internal_error)?
    .map_err(internal_error)?;

    Ok(Redirect::to("/reviews"))
}

#[derive(Template)]
#[template(path = "reviews.html")]
pub struct ReviewsTemplate<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
    pub reviews: Vec<(Review, Place)>,
}

pub async fn render_reviews(
    State(state): State<Context>,
) -> Result<ReviewsTemplate<'static>, (StatusCode, String)> {
    let conn = state.pool.get().await.map_err(internal_error)?;

    let reviews_with_place = conn
        .interact(|conn| {
            reviews::table
                .inner_join(places::table)
                .select((Review::as_select(), Place::as_select()))
                .load::<(Review, Place)>(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(ReviewsTemplate {
        title: "Salarios",
        subtitle: "Análisis de los salarios del servicio en México",
        reviews: reviews_with_place,
    })
}

#[derive(Template)]
#[template(path = "new_review.html")]
pub struct NewReviewTemplate {
    pub places: Vec<Place>,
}

pub async fn new_review(
    State(state): State<Context>,
) -> Result<NewReviewTemplate, (StatusCode, String)> {
    let conn = state.pool.get().await.map_err(internal_error)?;
    let places = conn
        .interact(|conn| places::table.select(Place::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(NewReviewTemplate { places })
}
