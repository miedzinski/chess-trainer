use anyhow::{ensure, Result};

use crate::puzzle::puzzle_repository::PuzzleRepository;
use crate::puzzle::types::{LichessPuzzleImport, Puzzle};

pub trait PuzzleService {
    fn import_puzzle(&mut self, lichess_puzzle: LichessPuzzleImport) -> Result<Puzzle>;
    fn list_puzzles(&self) -> Vec<Puzzle>;
}

pub struct PuzzleServiceImpl<P>
where
    P: PuzzleRepository + Send + Sync,
{
    puzzle_repository: P,
}

impl<P> PuzzleServiceImpl<P>
where
    P: PuzzleRepository + Send + Sync,
{
    pub fn new(puzzle_repository: P) -> PuzzleServiceImpl<P> {
        PuzzleServiceImpl { puzzle_repository }
    }
}

impl<P> PuzzleService for PuzzleServiceImpl<P>
where
    P: PuzzleRepository + Send + Sync,
{
    fn import_puzzle(&mut self, lichess_puzzle: LichessPuzzleImport) -> Result<Puzzle> {
        ensure!(
            (-100..=100).contains(&lichess_puzzle.popularity),
            "puzzle {}: popularity {} is out of range [-100, 100].",
            lichess_puzzle.puzzle_id,
            lichess_puzzle.popularity
        );
        self.puzzle_repository.create(lichess_puzzle.into())
    }

    fn list_puzzles(&self) -> Vec<Puzzle> {
        self.puzzle_repository.find()
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle::puzzle_repository::InMemoryPuzzleRepository;
    use crate::puzzle::service::PuzzleServiceImpl;
    use crate::puzzle::{Puzzle, PuzzleService};

    fn make_service() -> PuzzleServiceImpl<P> {
        let puzzle_repository = InMemoryPuzzleRepository::new();
        PuzzleServiceImpl { puzzle_repository }
    }

    fn sample_puzzle() -> Puzzle {
        Puzzle::new("3R4/8/K7/pB2b3/1p6/1P2k3/3p4/8 w - - 4 58".to_string()).unwrap()
    }

    #[test]
    fn xxx() {
        let mut service = make_service();

        service.add_puzzle(sample_puzzle());

        let puzzles = service.list_puzzles();
    }
}
