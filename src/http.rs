use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde::Serialize;

use crate::db::OracleClient;

#[derive(Clone)]
pub struct AppState {
    oracle_client: OracleClient,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    oracle: &'static str,
    result: i64,
}

pub fn router(oracle_client: OracleClient) -> Router {
    let state = AppState { oracle_client };

    Router::new()
        .route("/health/oracle", get(oracle_health))
        .with_state(state)
}

async fn oracle_health(
    State(state): State<AppState>,
) -> Result<Json<HealthResponse>, (StatusCode, String)> {
    let result = state
        .oracle_client
        .select_one_from_dual()
        .await
        .map_err(internal_error)?;

    Ok(Json(HealthResponse {
        oracle: "up",
        result,
    }))
}

fn internal_error(error: anyhow::Error) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}
