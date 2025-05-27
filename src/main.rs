use axum::{
    Router,
    http::{HeaderValue, Method},
    routing::{delete, get, post},
};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod handlers;
mod models;
use models::AppState;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let frontend_url =
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".into());
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let cors = CorsLayer::new()
        .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::POST, Method::GET, Method::DELETE])
        .allow_headers(Any);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let app_state = AppState { db_pool: pool };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/events", get(handlers::get_events))
        .route("/create-event", post(handlers::create_event))
        .route("/delete-event/{event_id}", delete(handlers::delete_event))
        .route("/events/{event_id}/teams", get(handlers::get_teams_for_event))
        .route("/teams", post(handlers::create_team))
        .route("/teams/{event_id}", get(handlers::get_teams_for_event))
        .layer(cors)
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
