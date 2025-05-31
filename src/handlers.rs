use crate::models;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::Row; // <-- Added for row.try_get()

pub async fn get_events(
    State(app_state): State<models::AppState>,
) -> impl IntoResponse {
    let events = sqlx::query_as::<_, models::Event>(
        "SELECT * FROM events",
    )
    .fetch_all(&app_state.db_pool)
    .await;

    match events {
        Ok(evts) => Json(evts).into_response(),
        Err(err) => {
            eprintln!("Error fetching events: {:?}", err); // Added !
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch events",
            )
                .into_response()
        }
    }
}

pub async fn create_event(
    State(app_state): State<models::AppState>,
    Json(event): Json<models::CreateEvent>,
) -> impl IntoResponse {
    let result = sqlx::query( // No '!', using function syntax
        r#"
        INSERT INTO events (name, date, location)
        VALUES ($1, $2, $3)
        RETURNING event_id
        "#,
    )
    .bind(event.name) // Bind parameters
    .bind(event.date)
    .bind(event.location)
    .fetch_one(&app_state.db_pool)
    .await;

    match result {
        Ok(row) => {
            // Get event_id from the row
            let event_id: i32 = match row.try_get("event_id") {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Failed to get event_id from row: {:?}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to process event creation",
                    )
                        .into_response();
                }
            };
            (
                StatusCode::CREATED,
                format!("Event created with ID: {}", event_id), // Added !
            )
                .into_response()
        }
        Err(err) => {
            eprintln!("Error creating event: {:?}", err); // Added !
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create event",
            )
                .into_response()
        }
    }
}

pub async fn delete_event(
    State(app_state): State<models::AppState>,
    Path(event_id): Path<i32>,
) -> impl IntoResponse {
    let result = sqlx::query( // No '!', using function syntax
        "DELETE FROM events WHERE event_id = $1",
    )
    .bind(event_id) // Bind parameter
    .execute(&app_state.db_pool)
    .await;

    match result {
        Ok(r) if r.rows_affected() == 1 => {
            (StatusCode::OK, "Event deleted").into_response()
        }
        Ok(_) => (StatusCode::NOT_FOUND, "Event not found").into_response(),
        Err(err) => {
            eprintln!("Error deleting event: {:?}", err); // Added !
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete event",
            )
                .into_response()
        }
    }
}

pub async fn create_team(
    State(app_state): State<models::AppState>,
    Json(team): Json<models::CreateTeam>,
) -> impl IntoResponse {
    let result = sqlx::query( // No '!', using function syntax
        r#"
        INSERT INTO teams (event_id, date_created, name, content)
        VALUES ($1, $2, $3, $4)
        RETURNING team_id
        "#,
    )
    .bind(team.event_id) // Bind parameters
    .bind(team.date_created)
    .bind(team.name)
    .bind(team.content)
    .fetch_one(&app_state.db_pool)
    .await;

    match result {
        Ok(row) => {
            // Get team_id from the row
            let team_id: i32 = match row.try_get("team_id") {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Failed to get team_id from row: {:?}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to process team creation",
                    )
                        .into_response();
                }
            };
            (
                StatusCode::CREATED,
                format!("Team created with ID: {}", team_id), // Added !
            )
                .into_response()
        }
        Err(err) => {
            eprintln!("Error creating team: {:?}", err); // Added !
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create team",
            )
                .into_response()
        }
    }
}

pub async fn get_team(
    State(app_state): State<models::AppState>,
    Path(team_id): Path<i32>,
) -> impl IntoResponse {
    // query_as is fine, it's a function and doesn't do compile-time checks
    // in the same way query! does.
    let team = sqlx::query_as::<_, models::Team>(
        "SELECT * FROM teams WHERE team_id = $1",
    )
    .bind(team_id)
    .fetch_optional(&app_state.db_pool)
    .await;

    match team {
        Ok(Some(team)) => Json(team).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Team not found").into_response(),
        Err(err) => {
            eprintln!("Error fetching team: {:?}", err); // Added !
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch team",
            )
                .into_response()
        }
    }
}


pub async fn get_teams_for_event(
    State(app_state): State<models::AppState>,
    Path(event_id): Path<i32>,
) -> impl IntoResponse {
    // query_as is fine
    let teams = sqlx::query_as::<_, models::Team>(
        "SELECT * FROM teams WHERE event_id = $1",
    )
    .bind(event_id)
    .fetch_all(&app_state.db_pool)
    .await;

    match teams {
        Ok(list) => Json(list).into_response(),
        Err(err) => {
            eprintln!( // Added !
                "Error fetching teams for event {}: {:?}",
                event_id, err
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch teams",
            )
                .into_response()
        }
    }
}
