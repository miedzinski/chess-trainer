use std::ops::RangeInclusive;

use crate::puzzle::types::{Puzzle, Theme, ThemeChoice};

#[cfg_attr(test, mockall::automock)]
pub trait PuzzleRepository {
    fn create(&self, puzzle: CreatePuzzle) -> anyhow::Result<Puzzle>;
    fn find(&self) -> Vec<Puzzle>;
    fn find_random(
        &self,
        count: usize,
        rating: &RangeInclusive<u16>,
        themes: &ThemeChoice,
    ) -> anyhow::Result<Vec<Puzzle>>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct CreatePuzzle {
    pub fen: String,
    pub moves: String,
    pub lichess_id: String,
    pub lichess_rating: u16,
    pub lichess_rating_deviation: u16,
    pub lichess_popularity: i8,
    pub lichess_play_count: u32,
    pub themes: Vec<Theme>,
    pub lichess_game_url: String,
}

pub struct DummyPuzzleRepository;

impl PuzzleRepository for DummyPuzzleRepository {
    fn create(&self, _puzzle: CreatePuzzle) -> anyhow::Result<Puzzle> {
        unimplemented!()
    }

    fn find(&self) -> Vec<Puzzle> {
        unimplemented!()
    }

    fn find_random(
        &self,
        _count: usize,
        _rating: &RangeInclusive<u16>,
        _themes: &ThemeChoice,
    ) -> anyhow::Result<Vec<Puzzle>> {
        unimplemented!()
    }
}
