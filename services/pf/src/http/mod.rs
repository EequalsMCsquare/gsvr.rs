mod api;
mod error;
use api::v1::{account, player};
use axum::routing::{get, post};
#[allow(unused)]
use error::Error;

fn account_route() -> axum::Router {
    axum::Router::new()
        .route("/account", post(account::register))
        .route("/account/auth", get(account::auth))
}

fn player_route() -> axum::Router {
    axum::Router::new()
        .route("/player", get(player::get_players))
        .route("/player", post(player::create_player))
}

pub fn make_app() -> axum::Router {
    axum::Router::new().nest("/api/v1", account_route().merge(player_route()))
}
