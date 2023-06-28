pub mod models;
pub mod schema;

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use models::*;
use schema::*;

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
        .route("/", get(list_reviews))
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
    weekly_salary: f32,
    shift_days_count: i32,
    shift_duration: i32,
}

#[derive(Debug, Serialize)]
struct TestResp {
    name: String,
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

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
