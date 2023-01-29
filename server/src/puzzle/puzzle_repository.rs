use std::collections::HashMap;

use anyhow::Result;
use parking_lot::Mutex;

use crate::puzzle::types::{Puzzle, PuzzleId, Theme};

pub trait PuzzleRepository {
    fn create(&self, puzzle: CreatePuzzle) -> Result<Puzzle>;
    fn find(&self) -> Vec<Puzzle>;
}

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

pub struct InMemoryPuzzleRepository {
    inner: Mutex<InMemoryPuzzleRepositoryInner>,
}

impl InMemoryPuzzleRepository {
    pub fn new() -> InMemoryPuzzleRepository {
        InMemoryPuzzleRepository {
            inner: Mutex::new(InMemoryPuzzleRepositoryInner::new()),
        }
    }
}

impl PuzzleRepository for InMemoryPuzzleRepository {
    fn create(&self, puzzle: CreatePuzzle) -> Result<Puzzle> {
        self.inner.lock().create(puzzle)
    }

    fn find(&self) -> Vec<Puzzle> {
        self.inner.lock().find()
    }
}

struct InMemoryPuzzleRepositoryInner {
    puzzles: HashMap<PuzzleId, Puzzle>,
    id_sequence: PuzzleId,
}

impl InMemoryPuzzleRepositoryInner {
    fn new() -> InMemoryPuzzleRepositoryInner {
        InMemoryPuzzleRepositoryInner {
            puzzles: HashMap::new(),
            id_sequence: 0,
        }
    }

    fn create(&mut self, create_puzzle: CreatePuzzle) -> Result<Puzzle> {
        self.id_sequence += 1;
        let puzzle = Puzzle {
            id: self.id_sequence,
            fen: create_puzzle.fen,
            moves: create_puzzle.moves,
            lichess_id: create_puzzle.lichess_id,
            lichess_rating: create_puzzle.lichess_rating,
            lichess_rating_deviation: create_puzzle.lichess_rating_deviation,
            lichess_popularity: create_puzzle.lichess_popularity,
            lichess_play_count: create_puzzle.lichess_play_count,
            themes: create_puzzle.themes,
            lichess_game_url: create_puzzle.lichess_game_url,
        };
        self.puzzles.insert(self.id_sequence, puzzle.clone());
        Ok(puzzle)
    }

    fn find(&self) -> Vec<Puzzle> {
        self.puzzles.values().cloned().collect()
    }
}
