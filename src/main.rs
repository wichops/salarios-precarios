pub mod components;
pub mod models;
pub mod schema;

use std::{error::Error, sync::Arc};

use axum::{
    extract::{Extension, Query, State},
    http::{HeaderMap, Request, StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse, Json, Redirect, Response},
    routing::{get, post},
    RequestPartsExt, Router,
};
use axum_sessions::{
    async_session::MemoryStore,
    extractors::{ReadableSession, WritableSession},
    SessionLayer,
};
use dotenvy::dotenv;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

mod prelude {
    use deadpool_diesel::{Manager, Pool};
    use diesel::prelude::*;

    pub type Database = Pool<Manager<PgConnection>>;
}

use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use prelude::*;

#[derive(Clone)]
struct Context {
    pool: Database,
    client: BasicClient,
}

type MaybeUser = Option<User>;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().expect(".env file not found");

    let db_url = std::env::var("DATABASE_URL")?;

    // setup connection pool
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();
    let store = MemoryStore::new();
    let secret = thread_rng().gen::<[u8; 64]>();
    let session_layer =
        SessionLayer::new(store, &secret).with_same_site_policy(axum_sessions::SameSite::Lax);

    let client_id = ClientId::new(std::env::var("AUTH0_CLIENT_ID")?);
    let client_secret = ClientSecret::new(std::env::var("AUTH0_CLIENT_SECRET")?);
    let auth_url = AuthUrl::new(std::env::var("AUTH0_URL")? + "/authorize")?;
    let token_url = TokenUrl::new(std::env::var("AUTH0_URL")? + "/oauth/token")?;

    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:3000/callback".to_string())
                .expect("Invalid redirect URL"),
        );

    let context = Context { pool, client };

    let app = Router::new()
        .nest_service("/public", ServeDir::new("public"))
        .route("/", get(list_reviews))
        .route("/reviews", get(reviews))
        .route("/places", get(list_places).post(create_place))
        .route("/create", post(create_review))
        .route("/protected", get(protected))
        .layer(Extension(MaybeUser::default()))
        .layer(middleware::from_fn_with_state(context.clone(), auth))
        .route("/login", get(login))
        .route("/sign_in", get(sign_in))
        .route("/callback", get(callback))
        .layer(session_layer)
        .with_state(context);

    println!("Starting server at 0.0.0.0:3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[derive(Serialize)]
struct Protected {
    msg: String,
}

async fn sign_in(mut session: WritableSession, State(_ctx): State<Context>) -> impl IntoResponse {
    session
        .insert("signed_in", true)
        .expect("can't set signed_in");

    Redirect::to("/reviews")
}

async fn login(State(ctx): State<Context>) -> impl IntoResponse {
    let client = ctx.client;

    let (auth_url, _state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

    Redirect::to(auth_url.as_str())
}

#[derive(Debug, Deserialize)]
struct AuthQuery {
    code: String,
    _state: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
struct NewUser {
    email: String,
}
async fn callback(
    State(ctx): State<Context>,
    Query(query): Query<AuthQuery>,
    mut session: WritableSession,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let conn = ctx.pool.get().await.map_err(internal_error)?;
    let client = ctx.client;

    let token = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .map_err(internal_error)?;

    let req_client = reqwest::Client::new();
    let user_info = req_client
        .get("https://dev-ncm5w43saalu6lgg.us.auth0.com/userinfo")
        .bearer_auth(token.access_token().clone().secret())
        .send()
        .await
        .map_err(internal_error)?
        .json::<NewUser>()
        .await
        .map_err(internal_error)?;

    let user_email = Arc::new(user_info.email);
    let email = user_email.clone();

    session
        .insert("email", user_email.to_string())
        .expect("Can't set email");

    let user = conn
        .interact(move |conn| {
            users::table
                .filter(users::email.eq(email.to_string()))
                .select(User::as_select())
                .first(conn)
        })
        .await
        .map_err(internal_error)?;

    let user_id = if let Ok(user) = user {
        user.id
    } else {
        conn.interact(move |conn| {
            diesel::insert_into(users::table)
                .values(users::email.eq(user_email.to_string()))
                .returning(users::id)
                .get_result::<i32>(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?
    };
    session
        .insert("user_id", user_id)
        .expect("can't set userid");
    session
        .insert("signed_in", true)
        .expect("can't set signed_in");

    let _ = conn
        .interact(move |conn| {
            diesel::insert_into(sessions::table)
                .values((
                    sessions::user_id.eq(user_id),
                    sessions::session_token.eq(session.id()),
                    sessions::access_token.eq(token.access_token().secret().to_string()),
                ))
                .returning(Session::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Redirect::to("/reviews"))
}

async fn protected(headers: HeaderMap) -> Result<Json<Protected>, (StatusCode, String)> {
    Ok(Json(Protected {
        msg: String::from("protected"),
    }))
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
    Ok(Html(components::reviews::reviews(reviews_with_place)))
}

async fn list_reviews(
    session: ReadableSession,
    State(state): State<Context>,
) -> Result<Json<Vec<Review>>, (StatusCode, String)> {
    let conn = state.pool.get().await.map_err(internal_error)?;

    if session.get::<bool>("signed_in").unwrap_or(false) {
        println!("LIST signed in!");
    } else {
        println!("LIST not signed in");
    }

    let res = conn
        .interact(|conn| reviews::table.select(Review::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(res))
}

async fn create_review(
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

#[derive(Deserialize, Insertable)]
#[diesel(table_name = places)]
struct NewPlace {
    name: String,
    address: String,
}

async fn create_place(
    State(state): State<Context>,
    Json(new_place): Json<NewPlace>,
) -> Result<Json<Place>, (StatusCode, String)> {
    let conn = state.pool.get().await.map_err(internal_error)?;

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
    State(state): State<Context>,
) -> Result<Json<Vec<Place>>, (StatusCode, String)> {
    let conn = state.pool.get().await.map_err(internal_error)?;

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

async fn auth<B>(
    State(ctx): State<Context>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, (StatusCode, String)> {
    let conn = ctx.pool.get().await.map_err(internal_error)?;

    let (mut parts, body) = request.into_parts();
    let session_handle: ReadableSession = parts.extract().await.map_err(internal_error)?;

    if let Some(email) = session_handle.get::<String>("email") {
        println!("My email: {}", email);
    } else {
        println!("No email xd");
    }

    if session_handle.get::<bool>("signed_in").unwrap_or(false) {
        println!("Logged in!");
    } else {
        println!("Shh, it's secret!");
    }

    match session_handle.get::<i32>("user_id") {
        Some(user_id) => {
            println!("LOGGED IN!");

            let user = conn
                .interact(move |conn| {
                    users::table
                        .find(user_id)
                        .select(User::as_select())
                        .first(conn)
                })
                .await
                .map_err(internal_error)?
                .map_err(internal_error)?;

            let mut request = Request::from_parts(parts, body);
            request.extensions_mut().insert(user);
            Ok(next.run(request).await)
        }
        None => {
            let request = Request::from_parts(parts, body);
            Ok(next.run(request).await)
        }
    }
}
