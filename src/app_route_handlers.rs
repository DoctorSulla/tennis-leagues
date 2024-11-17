use crate::{default_route_handlers::AppError, AppState};
use axum::extract::{Json, State};
use http::StatusCode;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
struct NewPlayerRequest {
    name: String,
    league_id: u32,
}

#[derive(Deserialize)]
struct NewLeagueRequest {
    name: String,
}

#[derive(Deserialize)]
struct AmendPlayerRequest {
    player_id: u32,
    new_league_id: u32,
}

async fn create_player(
    State(state): State<Arc<AppState>>,
    Json(player): Json<NewPlayerRequest>,
) -> Result<StatusCode, AppError> {
    sqlx::query("INSERT INTO PLAYERS(name,league_id) values(?,?) RETURNS player_id")
        .bind(player.name)
        .bind(player.league_id)
        .execute(&state.db_connection_pool)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn create_league(
    State(state): State<Arc<AppState>>,
    Json(league): Json<NewLeagueRequest>,
) -> Result<StatusCode, AppError> {
    sqlx::query("INSERT INTO LEAGUES(name) values(?) RETURNS league_id")
        .bind(league.name)
        .execute(&state.db_connection_pool)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn add_player_to_league(
    State(state): State<Arc<AppState>>,
    Json(player): Json<AmendPlayerRequest>,
) -> Result<StatusCode, AppError> {
    sqlx::query("UPDATE PLAYERS SET league_id=? WHERE player_id=?")
        .bind(player.new_league_id)
        .bind(player.player_id)
        .execute(&state.db_connection_pool)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn generate_fixtures(State(state): State<Arc<AppState>>, league_id: u32) {
    let league_players = sqlx::query("SELECT player_id FROM players WHERE league_id=?")
        .bind(league_id)
        .fetch_all(&state.db_connection_pool)
        .await;
}

async fn submit_result() {}

async fn generate_league_table(league_id: u32) {}
