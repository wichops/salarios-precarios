use crate::components::reviews::reviews_table;
use crate::models::*;
use crate::prelude::*;
use crate::schema::*;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = reviews)]
pub struct NewReview {
    pub place_id: i32,
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

pub async fn render_reviews(
    Extension(user): Extension<User>,
    State(state): State<Context>,
) -> Result<Html<String>, (StatusCode, String)> {
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

    // let (all_places, reviews) = conn
    //     .interact(
    //         |conn| -> Result<(Vec<Place>, Vec<Review>), diesel::result::Error> {
    //             let all_places = places::table.select(Place::as_select()).load(conn)?;

    //             let reviews = Review::belonging_to(&all_places)
    //                 .select(Review::as_select())
    //                 .load(conn)?;

    //             Ok((all_places, reviews))
    //         },
    //     )
    //     .await
    //     .map_err(internal_error)?
    //     .map_err(internal_error)?;

    // let reviews = reviews
    //     .grouped_by(&all_places)
    //     .into_iter()
    //     .zip(all_places)
    //     .map(|(reviews, place)| (place, reviews))
    //     .collect::<Vec<(Place, Vec<Review>)>>();

    println!("USER!?!? {user:?}");
    Ok(Html(reviews_table(reviews_with_place)))
}
