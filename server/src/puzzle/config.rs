use crate::puzzle::puzzle_repository::DummyPuzzleRepository;
use crate::puzzle::service::PuzzleServiceImpl;
use crate::puzzle::training_set_repository::DummyTrainingSetRepository;
use crate::puzzle::PuzzleService;

pub fn make_service() -> impl PuzzleService {
    let puzzle_repository = DummyPuzzleRepository;
    let training_set_repository = DummyTrainingSetRepository;
    PuzzleServiceImpl::new(puzzle_repository, training_set_repository)
}
