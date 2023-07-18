use crate::models::*;
use crate::prelude::*;
use crate::schema::*;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = reviews)]
pub struct NewReview {
    pub place_id: i32,
    pub user_id: i32,
    pub weekly_salary: f32,
    pub shift_days_count: i32,
    pub shift_duration: i32,
}

pub async fn create_review(
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
