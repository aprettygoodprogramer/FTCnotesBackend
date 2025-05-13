use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use chrono::NaiveDateTime;
#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Event {
    pub event_id: i32,
    pub name: String,
    pub date: Option<NaiveDate>,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEvent {
    pub name: String,
    pub date: Option<NaiveDate>,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEvent {
    pub name: Option<String>,
    pub date: Option<NaiveDate>,
    pub location: Option<String>,
}
#[derive(Serialize)]
pub struct Team {
    pub team_id: i32,
    pub event_id: i32,
    pub date_created: NaiveDateTime,
    pub name: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct CreateTeam {
    pub event_id: i32,
    pub date_created: NaiveDateTime,
    pub name: String,
    pub content: String,
}