use crate::{default_route_handlers::AppError, AppState};
use axum::extract::{Json, Path, State};
use http::StatusCode;
use serde::Deserialize;
use sqlx::prelude::FromRow;
use sqlx::Row;
use std::collections::HashMap;
use std::sync::Arc;

const SEASON: i8 = 1;

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

#[derive(Deserialize, FromRow)]
struct MatchResult {
    season: i64,
    league_id: i64,
    player_one_id: i64,
    player_two_id: i64,
    player_one_set_one_games: i8,
    player_two_set_one_games: i8,
    player_one_set_two_games: i8,
    player_two_set_two_games: i8,
    player_one_tiebreak_games: Option<i8>,
    player_two_tiebreak_games: Option<i8>,
    completed: i8,
}

#[derive(Deserialize)]
struct LeagueTableAndFixtures {
    league_table: Vec<LeagueTableRow>,
    completed_fixtures: Vec<MatchResult>,
    uncompleted_fixtures: Vec<MatchResult>,
}

#[derive(Deserialize)]
struct LeagueTableRow {
    name: String,
    played: u8,
    matches_won: u8,
    matches_lost: u8,
    sets_won: u8,
    sets_lost: u8,
    games_won: u8,
    games_lost: u8,
}

async fn create_player(
    State(state): State<Arc<AppState>>,
    Json(player): Json<NewPlayerRequest>,
) -> Result<StatusCode, AppError> {
    sqlx::query("INSERT INTO PLAYERS(name,league_id) values(?,?) RETURNING rowid")
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
    sqlx::query("INSERT INTO LEAGUES(name) values(?) RETURNING rowid")
        .bind(league.name)
        .execute(&state.db_connection_pool)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn add_player_to_league(
    State(state): State<Arc<AppState>>,
    Json(player): Json<AmendPlayerRequest>,
) -> Result<StatusCode, AppError> {
    sqlx::query("UPDATE PLAYERS SET league_id=? WHERE rowid=?")
        .bind(player.new_league_id)
        .bind(player.player_id)
        .execute(&state.db_connection_pool)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn generate_fixtures(State(state): State<Arc<AppState>>) -> Result<StatusCode, AppError> {
    let leagues = sqlx::query("SELECT rowid FROM leagues")
        .fetch_all(&state.db_connection_pool)
        .await?;

    let league_ids: Vec<i32> = leagues.into_iter().map(|x| x.get(0)).collect();

    for league_id in league_ids {
        let league_players = sqlx::query("SELECT rowid FROM players WHERE league_id=?")
            .bind(league_id)
            .fetch_all(&state.db_connection_pool)
            .await?;

        let player_ids: Vec<i64> = league_players.into_iter().map(|x| x.get(0)).collect();

        let mut i = 0;
        let mut j = 1;

        while i < player_ids.len() {
            while j < player_ids.len() {
                sqlx::query("INSERT INTO fixtures (season,league_id,player_one_id,player_two_id) values(?,?,?,?)")
                .bind(SEASON)
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
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn submit_result(
    State(state): State<Arc<AppState>>,
    Json(match_result): Json<MatchResult>,
) -> Result<StatusCode, AppError> {
    Ok(StatusCode::NO_CONTENT)
}

async fn generate_league_table(
    Path(league_id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<LeagueTableAndFixtures>, AppError> {
    let uncompleted_fixtures = sqlx::query_as::<_, MatchResult>(
        "SELECT * FROM fixtures WHERE league_id=? and completed=0",
    )
    .bind(league_id)
    .fetch_all(&state.db_connection_pool)
    .await?;

    let completed_fixtures = sqlx::query_as::<_, MatchResult>(
        "SELECT * FROM fixtures WHERE league_id=? and completed=1",
    )
    .bind(league_id)
    .fetch_all(&state.db_connection_pool)
    .await?;

    let league_players = get_league_players(league_id, state).await?;

    let league_table = compute_league_table(league_players, &completed_fixtures).await;

    let mut league_table_and_fixtures = LeagueTableAndFixtures {
        completed_fixtures,
        uncompleted_fixtures,
        league_table: vec![],
    };

    Ok(Json(league_table_and_fixtures))
}

async fn get_players_map(state: Arc<AppState>) -> Result<HashMap<i64, String>, anyhow::Error> {
    let players = sqlx::query("SELECT rowid,name FROM players")
        .fetch_all(&state.db_connection_pool)
        .await?;
    let mut players_map: HashMap<i64, String> = HashMap::new();

    for player in players {
        players_map.insert(player.get(0), player.get(1));
    }
    Ok(players_map)
}

async fn compute_league_table(
    league_players: Vec<i64>,
    completed_fixures: &Vec<MatchResult>,
) -> Vec<LeagueTableRow> {
    vec![]
}

async fn get_league_players(
    league_id: i64,
    state: Arc<AppState>,
) -> Result<Vec<i64>, anyhow::Error> {
    let league_players = sqlx::query("SELECT rowid FROM players WHERE league_id=?")
        .bind(league_id)
        .fetch_all(&state.db_connection_pool)
        .await?;

    Ok(league_players.into_iter().map(|x| x.get(0)).collect())
}
