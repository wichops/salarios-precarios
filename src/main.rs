pub mod components;
pub mod models;
pub mod schema;

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use diesel::prelude::*;
use serde::Deserialize;
use tower_http::services::ServeDir;

use crate::models::*;
use crate::schema::*;

#[tokio::main]
async fn main() {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:monsterfoot@localhost/service_life".to_string());

    // setup connection pool
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    let app = Router::new()
        .nest_service("/public", ServeDir::new("public"))
        .route("/", get(list_reviews))
        .route("/reviews", get(reviews))
        .route("/places", get(list_places).post(create_place))
        .route("/create", post(create_review))
        .with_state(pool);

    println!("Starting server at 0.0.0.0:3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = reviews)]
struct NewReview {
    place_id: i32,
    weekly_salary: f32,
    shift_days_count: i32,
    shift_duration: i32,
}

async fn reviews(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Html<String>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

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

    Ok(Html(components::reviews::reviews(reviews_with_place)))
}

async fn list_reviews(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<Review>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let res = conn
        .interact(|conn| reviews::table.select(Review::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(res))
}

async fn create_review(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(new_review): Json<NewReview>,
) -> Result<Json<Review>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

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

#[derive(Deserialize, Insertable)]
#[diesel(table_name = places)]
struct NewPlace {
    name: String,
    address: String,
}

async fn create_place(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(new_place): Json<NewPlace>,
) -> Result<Json<Place>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(places::table)
                .values(new_place)
                .returning(Place::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(res))
}

async fn list_places(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<Place>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let res = conn
        .interact(|conn| places::table.select(Place::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(res))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
