use crate::models;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::{Error, Row};

pub async fn get_events(State(app_state): State<models::AppState>) -> impl IntoResponse {
    let events = sqlx::query_as::<_, models::Event>("SELECT * FROM events")
        .fetch_all(&app_state.db_pool)
        .await;

    match events {
        Ok(events) => Json(events).into_response(),
        Err(err) => {
            eprintln!("Error fetching events: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch events").into_response()
        }
    }
}

pub async fn create_event(
    State(app_state): State<models::AppState>,
    Json(event): Json<models::CreateEvent>,
) -> impl IntoResponse {
    let result = sqlx::query(
        "INSERT INTO events (name, date, location) VALUES ($1, $2, $3) RETURNING event_id",
    )
    .bind(&event.name)
    .bind(event.date)
    .bind(event.location)
    .fetch_one(&app_state.db_pool)
    .await;

    match result {
        Ok(row) => {
            let event_id: i32 = row.get(0);
            (
                StatusCode::CREATED,
                format!("Event created with ID: {}", event_id),
            )
                .into_response()
        }
        Err(err) => {
            eprintln!("Error creating event: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create event").into_response()
        }
    }
}

pub async fn delete_event(
    State(app_state): State<models::AppState>,
    Path(event_id): Path<i32>,
) -> impl IntoResponse {
    let result = sqlx::query("DELETE FROM events WHERE event_id = $1")
        .bind(event_id)
        .execute(&app_state.db_pool)
        .await;

    match result {
        Ok(rows_affected) if rows_affected.rows_affected() == 1 => {
            (StatusCode::OK, "Event deleted").into_response()
        }
        Ok(_) => (StatusCode::NOT_FOUND, "Event not found").into_response(),
        Err(err) => {
            eprintln!("Error deleting event: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete event").into_response()
        }
    }
}
