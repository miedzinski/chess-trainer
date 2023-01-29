use std::sync::Arc;

use axum::Router;

use crate::puzzle;
use crate::puzzle::PuzzleService;

pub struct Context<P>
where
    P: PuzzleService + Send + Sync + 'static,
{
    pub puzzle_service: P,
}

pub fn make_router<P>(puzzle_service: P) -> Router
where
    P: PuzzleService + Send + Sync + 'static,
{
    let ctx = Context { puzzle_service };

    Router::new()
        .merge(puzzle::make_router())
        .with_state(Arc::new(ctx))
}
