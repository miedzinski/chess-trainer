use std::sync::Arc;

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};

use crate::infrastructure::rest::Context;
use crate::puzzle::types::Puzzle;
use crate::puzzle::PuzzleService;

pub fn make_router<T>() -> Router<Arc<Context<T>>>
where
    T: PuzzleService + Send + Sync + 'static,
{
    Router::new().route("/puzzles", get(list_puzzles))
}

pub async fn list_puzzles<T>(State(ctx): State<Arc<Context<T>>>) -> Json<Vec<Puzzle>>
where
    T: PuzzleService + Send + Sync + 'static,
{
    Json(ctx.puzzle_service.list_puzzles())
}
