use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub _unit: (),
}

impl FromRef<AppState> for () {
    fn from_ref(input: &AppState) -> Self {
        input._unit
    }
}
