use crate::puzzle::puzzle_repository::DummyPuzzleRepository;
use crate::puzzle::service::PuzzleServiceImpl;
use crate::puzzle::PuzzleService;

pub fn make_service() -> impl PuzzleService {
    PuzzleServiceImpl::new(DummyPuzzleRepository)
}
