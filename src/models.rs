use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

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
