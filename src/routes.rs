use crate::{app_route_handlers, default_route_handlers, AppState};
use axum::{
    routing::{get, patch, post, put},
    Router,
};
use std::sync::Arc;

pub fn get_protected_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/account/verifyEmail",
            post(default_route_handlers::verify_email),
        )
        .route(
            "/account/changePassword",
            patch(default_route_handlers::change_password),
        )
        .route(
            "/api/allFixtures",
            get(app_route_handlers::generate_fixtures),
        )
        .route("/api/result", put(app_route_handlers::put_result))
        .route("/api/player", post(app_route_handlers::create_player))
        .route("/api/league", post(app_route_handlers::create_league))
        .route(
            "/api/player",
            patch(app_route_handlers::add_player_to_league),
        )
}

pub fn get_open_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/account/register", post(default_route_handlers::register))
        .route("/account/login", post(default_route_handlers::login))
        .route(
            "/account/resetPassword",
            post(default_route_handlers::password_reset_initiate),
        )
        .route(
            "/account/resetPassword",
            patch(default_route_handlers::password_reset_complete),
        )
        .route(
            "/api/leagueTable/:league_id",
            get(app_route_handlers::generate_league_table),
        )
        .route("/api/leagues", get(app_route_handlers::get_leagues))
}
