use crate::{default_route_handlers::AppError, AppState};
use axum::extract::{Json, Path, State};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::Row;
use std::collections::HashMap;
use std::sync::Arc;

const SEASON: i8 = 1;

#[derive(Deserialize)]
pub struct NewPlayerRequest {
    name: String,
    league_id: i64,
}

#[derive(Deserialize)]
pub struct NewLeagueRequest {
    name: String,
}

#[derive(Deserialize)]
pub struct AmendPlayerRequest {
    player_id: i64,
    new_league_id: i64,
}

#[derive(Deserialize, FromRow, Serialize)]
pub struct MatchResult {
    season: i64,
    league_id: i64,
    player_one_id: i64,
    player_two_id: i64,
    player_one_set_one_games: i8,
    player_two_set_one_games: i8,
    player_one_set_two_games: i8,
    player_two_set_two_games: i8,
    player_one_tiebreak_points: Option<i8>,
    player_two_tiebreak_points: Option<i8>,
    completed: i8,
}

#[derive(Serialize, Deserialize)]
pub struct LeagueTableAndFixtures {
    league_table: Vec<LeagueTableRow>,
    completed_fixtures: Vec<MatchResult>,
    uncompleted_fixtures: Vec<MatchResult>,
}

#[derive(Serialize, Deserialize)]
struct LeagueTableRow {
    name: String,
    played: i8,
    matches_won: i8,
    matches_lost: i8,
    sets_won: i8,
    sets_lost: i8,
    games_won: i8,
    games_lost: i8,
    points: i8,
}

impl LeagueTableRow {
    pub fn new(name: String) -> Self {
        Self {
            name,
            played: 0,
            matches_won: 0,
            matches_lost: 0,
            sets_won: 0,
            sets_lost: 0,
            games_won: 0,
            games_lost: 0,
            points: 0,
        }
    }
}

pub async fn create_player(
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

pub async fn create_league(
    State(state): State<Arc<AppState>>,
    Json(league): Json<NewLeagueRequest>,
) -> Result<StatusCode, AppError> {
    sqlx::query("INSERT INTO LEAGUES(name) values(?) RETURNING rowid")
        .bind(league.name)
        .execute(&state.db_connection_pool)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_player_to_league(
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

pub async fn submit_result(
    State(state): State<Arc<AppState>>,
    Json(match_result): Json<MatchResult>,
) -> Result<StatusCode, AppError> {
    sqlx::query(
        "UPDATE FIXTURES SET
        player_one_set_one_games=?,
        player_one_set_two_games=?,
        player_two_set_one_games=?,
        player_two_set_two_games=?,
        player_one_tiebreak_points=?,
        player_two_tiebreak_points=?
        WHERE
        season=? and
        league_id=? and
        player_one_id=? and
        player_two_id=?
        ",
    )
    .bind(match_result.player_one_set_one_games)
    .bind(match_result.player_one_set_two_games)
    .bind(match_result.player_two_set_one_games)
    .bind(match_result.player_two_set_two_games)
    .bind(match_result.player_one_tiebreak_points)
    .bind(match_result.player_two_tiebreak_points)
    .bind(match_result.season)
    .bind(match_result.league_id)
    .bind(match_result.player_one_id)
    .bind(match_result.player_two_id)
    .execute(&state.db_connection_pool)
    .await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn generate_league_table(
    Path(league_id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<LeagueTableAndFixtures>, AppError> {
    let player_map = get_player_map(state.clone()).await?;

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

    let league_table = compute_league_table(league_players, player_map, &completed_fixtures).await;

    let mut league_table_and_fixtures = LeagueTableAndFixtures {
        completed_fixtures,
        uncompleted_fixtures,
        league_table,
    };

    Ok(Json(league_table_and_fixtures))
}

async fn get_player_map(state: Arc<AppState>) -> Result<HashMap<i64, String>, anyhow::Error> {
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
    player_map: HashMap<i64, String>,
    completed_fixures: &Vec<MatchResult>,
) -> Vec<LeagueTableRow> {
    let mut league_table = vec![];
    for player_id in league_players {
        let mut row = LeagueTableRow::new(
            player_map
                .get(&player_id)
                .expect("Player not found in map")
                .to_owned(),
        );
        // Loop through fixtures
        for fixture in completed_fixures {
            if fixture.player_one_id == player_id {
                // Match logic
                row.points += 1;
                // Set 1 logic
                // Won set
                if fixture.player_one_set_one_games > fixture.player_two_set_one_games {
                    row.sets_won += 1;
                    row.points += 1;
                }
                // Lost set
                else if fixture.player_one_set_one_games < fixture.player_two_set_one_games {
                    row.sets_lost += 1;
                }
                row.games_won += fixture.player_one_set_one_games;
                // Set 2 logic
                // Won set
                if fixture.player_one_set_two_games > fixture.player_two_set_two_games {
                    row.sets_won += 1;
                    row.points += 1;
                }
                // Lost set
                else if fixture.player_one_set_two_games < fixture.player_two_set_two_games {
                    row.sets_lost += 1;
                }
                row.games_won += fixture.player_one_set_two_games;

                // Tiebreak if applicable
                if fixture.player_one_tiebreak_points.is_some()
                    && fixture.player_two_tiebreak_points.is_some()
                {
                    if fixture.player_one_tiebreak_points.unwrap()
                        > fixture.player_two_tiebreak_points.unwrap()
                    {
                        row.sets_won += 1;
                    } else if fixture.player_one_tiebreak_points.unwrap()
                        < fixture.player_two_tiebreak_points.unwrap()
                    {
                        row.sets_lost += 1;
                    }
                }
            } else if fixture.player_two_id == player_id {
                // Match logic
                row.points += 1;
                // Set 1 logic
                // Won set
                if fixture.player_two_set_one_games > fixture.player_one_set_one_games {
                    row.sets_won += 1;
                    row.points += 1;
                }
                // Lost set
                else if fixture.player_two_set_one_games < fixture.player_one_set_one_games {
                    row.sets_lost += 1;
                }
                row.games_won += fixture.player_two_set_one_games;
                // Set 2 logic
                // Won set
                if fixture.player_two_set_two_games > fixture.player_one_set_two_games {
                    row.sets_won += 1;
                    row.points += 1;
                }
                // Lost set
                else if fixture.player_two_set_two_games < fixture.player_one_set_two_games {
                    row.sets_lost += 1;
                }
                row.games_won += fixture.player_two_set_two_games;
                // Tiebreak if applicable
                if fixture.player_one_tiebreak_points.is_some()
                    && fixture.player_two_tiebreak_points.is_some()
                {
                    if fixture.player_one_tiebreak_points.unwrap()
                        < fixture.player_two_tiebreak_points.unwrap()
                    {
                        row.sets_won += 1;
                    } else if fixture.player_one_tiebreak_points.unwrap()
                        > fixture.player_two_tiebreak_points.unwrap()
                    {
                        row.sets_lost += 1;
                    }
                }
            }
        }
        league_table.push(row);
    }
    league_table.sort_by_key(|f| f.points);
    league_table
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
