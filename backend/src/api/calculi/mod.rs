use axum::Router;

pub mod board;
pub mod game;
pub mod player;

pub fn router() -> Router {

    Router::new()
}