use crate::{default_route_handlers::AppError, AppState};
use axum::extract::{Json, Path, State};
use http::StatusCode;
use serde::Deserialize;
use sqlx::Row;
use std::sync::Arc;

#[derive(Deserialize)]
struct NewPlayerRequest {
    name: String,
    league_id: i64,
}

#[derive(Deserialize)]
struct NewLeagueRequest {
    name: String,
}

#[derive(Deserialize)]
struct AmendPlayerRequest {
    player_id: i64,
    new_league_id: i64,
}

async fn create_player(
    State(state): State<Arc<AppState>>,
    Json(player): Json<NewPlayerRequest>,
) -> Result<StatusCode, AppError> {
    sqlx::query("INSERT INTO PLAYERS(name,league_id) values(?,?) RETURNING player_id")
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
    sqlx::query("INSERT INTO LEAGUES(name) values(?) RETURNING league_id")
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

pub async fn generate_fixtures(
    State(state): State<Arc<AppState>>,
    Path(league_id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let league_players = sqlx::query("SELECT rowid FROM players WHERE league_id=?")
        .bind(league_id)
        .fetch_all(&state.db_connection_pool)
        .await?;

    let player_ids: Vec<i64> = league_players.into_iter().map(|x| x.get(0)).collect();
    println!("{:?}", player_ids);

    let mut i = 0;
    let mut j = 1;

    while i < player_ids.len() {
        while j < player_ids.len() {
            sqlx::query("INSERT INTO fixtures (season,league_id,player_one_id,player_two_id) values(?,?,?,?)")
                .bind(1)
                .bind(league_id)
                .bind(player_ids[i])
                .bind(player_ids[j])
                .execute(&state.db_connection_pool)
                .await?;
            j += 1;
        }
        i += 1;
        j = i + 1;
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn submit_result() {}

async fn generate_league_table(league_id: u64) {}
