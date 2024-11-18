use crate::{app_route_handlers, default_route_handlers, AppState};
use axum::{
    routing::{get, patch, post},
    Router,
};
use std::sync::Arc;

pub fn get_protected_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(default_route_handlers::hello_world))
        .route(
            "/account/verifyEmail",
            post(default_route_handlers::verify_email),
        )
        .route(
            "/account/changePassword",
            patch(default_route_handlers::change_password),
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
            "/api/generateFixtures",
            get(app_route_handlers::generate_fixtures),
        )
}
