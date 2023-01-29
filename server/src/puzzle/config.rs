use crate::puzzle::puzzle_repository::InMemoryPuzzleRepository;
use crate::puzzle::service::PuzzleServiceImpl;
use crate::puzzle::PuzzleService;

pub fn make_service() -> impl PuzzleService {
    let puzzle_repository = InMemoryPuzzleRepository::new();

    PuzzleServiceImpl::new(puzzle_repository)
}
