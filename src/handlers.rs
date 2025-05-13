use crate::models;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::NaiveDateTime;
use sqlx::Row;

pub async fn get_events(State(app_state): State<models::AppState>) 
    -> impl IntoResponse 
{
    let events = sqlx::query_as::<_, models::Event>(
            "SELECT * FROM events"
        )
        .fetch_all(&app_state.db_pool)
        .await;

    match events {
        Ok(events) => Json(events).into_response(),
        Err(err) => {
            eprintln!("Error fetching events: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, 
             "Failed to fetch events").into_response()
        }
    }
}

pub async fn create_event(
    State(app_state): State<models::AppState>,
    Json(event): Json<models::CreateEvent>,
) -> impl IntoResponse {
    let result = sqlx::query(
            "INSERT INTO events (name, date, location)
             VALUES ($1, $2, $3)
             RETURNING event_id",
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
            (StatusCode::INTERNAL_SERVER_ERROR, 
             "Failed to create event").into_response()
        }
    }
}

pub async fn delete_event(
    State(app_state): State<models::AppState>,
    Path(event_id): Path<i32>,
) -> impl IntoResponse {
    let result = sqlx::query(
            "DELETE FROM events WHERE event_id = $1"
        )
        .bind(event_id)
        .execute(&app_state.db_pool)
        .await;

    match result {
        Ok(result) if result.rows_affected() == 1 => {
            (StatusCode::OK, "Event deleted").into_response()
        }
        Ok(_) => (StatusCode::NOT_FOUND, "Event not found").into_response(),
        Err(err) => {
            eprintln!("Error deleting event: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, 
             "Failed to delete event").into_response()
        }
    }
}

/// Inserts a new team into the `teams` table and returns its `team_id`.
pub async fn create_team(
    State(app_state): State<models::AppState>,
    Json(team): Json<models::CreateTeam>,
) -> impl IntoResponse {
    let result = sqlx::query(
            "INSERT INTO teams
             (event_id, date_created, name, content)
             VALUES ($1, $2, $3, $4)
             RETURNING team_id",
        )
        .bind(team.event_id)
        .bind(team.date_created)
        .bind(&team.name)
        .bind(&team.content)
        .fetch_one(&app_state.db_pool)
        .await;

    match result {
        Ok(row) => {
            let team_id: i32 = row.get("team_id");
            (
                StatusCode::CREATED,
                format!("Team created with ID: {}", team_id),
            )
                .into_response()
        }
        Err(err) => {
            eprintln!("Error creating team: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, 
             "Failed to create team").into_response()
        }
    }
}
