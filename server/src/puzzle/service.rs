use anyhow::{ensure, Result};

use crate::puzzle::puzzle_repository::PuzzleRepository;
use crate::puzzle::types::{CreateTrainingSetOptions, LichessPuzzleImport, Puzzle, TrainingSet};

const MAX_SET_NAME_LENGTH: usize = 100;
const MIN_SET_SIZE: usize = 5;
const MAX_SET_SIZE: usize = 1000;

pub trait PuzzleService {
    fn import_puzzle(&self, lichess_puzzle: LichessPuzzleImport) -> Result<Puzzle>;
    fn list_puzzles(&self) -> Vec<Puzzle>;
    fn create_set(&self, options: CreateTrainingSetOptions) -> anyhow::Result<TrainingSet>;
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
    fn import_puzzle(&self, lichess_puzzle: LichessPuzzleImport) -> Result<Puzzle> {
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

    fn create_set(&self, options: CreateTrainingSetOptions) -> anyhow::Result<TrainingSet> {
        ensure!(
            options.name.len() <= MAX_SET_NAME_LENGTH,
            "Set name length can't exceed {}.",
            MAX_SET_NAME_LENGTH
        );
        ensure!(
            options.size >= MIN_SET_SIZE,
            "Set size must be at least {}.",
            MIN_SET_SIZE
        );
        ensure!(
            options.size <= MAX_SET_SIZE,
            "Set size can't exceed {}.",
            MAX_SET_SIZE
        );

        let puzzles =
            self.puzzle_repository
                .find_random(options.size, &options.rating, &options.themes)?;

        ensure!(
            puzzles.len() == options.size,
            "Not enough puzzles meet the criteria given."
        );

        let puzzle_ids = puzzles.iter().map(|puzzle| puzzle.id).collect();
        let set = TrainingSet {
            puzzle_ids,
            name: options.name,
            rating: options.rating,
            themes: options.themes,
            current_progress: 0,
            cycles_done: 0,
        };
        Ok(set)
    }
}

#[cfg(test)]
mod tests {
    use std::iter::repeat_with;

    use parking_lot::Mutex;

    use crate::puzzle::puzzle_repository::MockPuzzleRepository;
    use crate::puzzle::service::PuzzleServiceImpl;
    use crate::puzzle::types::{
        CreateTrainingSetOptions, CreateTrainingSetOptionsBuilder, LichessPuzzleImportBuilder,
        Puzzle, PuzzleBuilder, PuzzleId, Theme, ThemeChoice, TrainingSet,
    };
    use crate::puzzle::PuzzleService;

    fn sample_puzzle() -> PuzzleBuilder {
        let mut builder = PuzzleBuilder::default();
        builder
            .id(0)
            .fen("sample-fen".to_string())
            .moves("sample-moves".to_string())
            .lichess_id("sample-lichess-id".to_string())
            .lichess_rating(1500)
            .lichess_rating_deviation(50)
            .lichess_popularity(50)
            .lichess_play_count(1000)
            .themes(vec![Theme::DiscoveredAttack, Theme::MateIn2])
            .lichess_game_url("sample-lichess-game-url".to_string());
        builder
    }

    fn sample_lichess_puzzle() -> LichessPuzzleImportBuilder {
        let mut builder = LichessPuzzleImportBuilder::default();
        builder
            .puzzle_id("sample-lichess-id".to_string())
            .fen("sample-fen".to_string())
            .moves("sample-moves".to_string())
            .rating(1500)
            .rating_deviation(50)
            .popularity(50)
            .play_count(1000)
            .themes(vec![Theme::DiscoveredAttack, Theme::MateIn2])
            .game_url("sample-lichess-game-url".to_string());
        builder
    }

    fn sample_create_training_set_options() -> CreateTrainingSetOptionsBuilder {
        let mut builder = CreateTrainingSetOptionsBuilder::default();
        builder
            .name("sample-training-set-name".to_string())
            .size(10)
            .rating(1500..=1600)
            .themes(ThemeChoice::HealthyMix);
        builder
    }

    fn repository_creates(puzzle_repository: &mut MockPuzzleRepository) {
        puzzle_repository.expect_create().returning(|puzzle| {
            static COUNTER_MUTEX: Mutex<usize> = Mutex::new(0);
            let mut guard = COUNTER_MUTEX.lock();
            let id = *guard as PuzzleId;
            *guard += 1;
            Ok(Puzzle {
                id,
                fen: puzzle.fen.clone(),
                moves: puzzle.moves.clone(),
                lichess_id: puzzle.lichess_id.clone(),
                lichess_rating: puzzle.lichess_rating,
                lichess_rating_deviation: puzzle.lichess_rating_deviation,
                lichess_popularity: puzzle.lichess_popularity,
                lichess_play_count: puzzle.lichess_play_count,
                themes: puzzle.themes.clone(),
                lichess_game_url: puzzle.lichess_game_url,
            })
        });
    }

    fn repository_finds_random(
        puzzle_repository: &mut MockPuzzleRepository,
        size_limit: Option<usize>,
    ) {
        puzzle_repository
            .expect_find_random()
            .returning(move |size, _, _| {
                Ok((0..usize::min(size, size_limit.unwrap_or(usize::MAX)))
                    .map(|id| sample_puzzle().id(id as PuzzleId).build().unwrap())
                    .take(size)
                    .collect())
            });
    }

    #[test]
    fn should_import_lichess_puzzle() {
        // given Lichess puzzle:
        let lichess_puzzle = sample_lichess_puzzle().build().unwrap();

        // and repository that saves puzzles:
        let mut puzzle_repository = MockPuzzleRepository::new();
        repository_creates(&mut puzzle_repository);

        // when Lichess puzzle is imported:
        let service = PuzzleServiceImpl::new(puzzle_repository);
        let imported_puzzle = service.import_puzzle(lichess_puzzle).unwrap();

        // then it has correct data:
        let expected_puzzle = sample_puzzle().build().unwrap();
        assert_eq!(imported_puzzle, expected_puzzle);
    }

    #[test]
    fn should_create_set() {
        // given set options:
        let name = "My training set";
        let rating = 1500..=1600;
        let themes = ThemeChoice::HealthyMix;
        let options = CreateTrainingSetOptions {
            name: name.to_string(),
            size: 10,
            rating,
            themes,
        };

        // and repository that finds random puzzles:
        let mut puzzle_repository = MockPuzzleRepository::new();
        repository_finds_random(&mut puzzle_repository, None);

        // when set is created:
        let service = PuzzleServiceImpl::new(puzzle_repository);
        let set = service.create_set(options.clone()).unwrap();

        // then it has correct data:
        let expected = TrainingSet {
            puzzle_ids: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            name: name.to_string(),
            rating: options.rating,
            themes: options.themes,
            current_progress: 0,
            cycles_done: 0,
        };
        assert_eq!(set, expected);
    }

    #[test]
    fn should_disallow_creating_sets_with_name_too_long() {
        // given too long name:
        let name: String = repeat_with(|| "a").take(101).collect();
        let options = sample_create_training_set_options()
            .name(name)
            .build()
            .unwrap();

        // and repository:
        let puzzle_repository = MockPuzzleRepository::default();

        // when set is created:
        let service = PuzzleServiceImpl::new(puzzle_repository);
        let create_set_result = service.create_set(options);

        // then error is returned:
        assert!(create_set_result.is_err())
    }

    #[test]
    fn should_disallow_creating_sets_with_size_too_small() {
        // given too small size:
        let options = sample_create_training_set_options()
            .size(4)
            .build()
            .unwrap();

        // and repository:
        let puzzle_repository = MockPuzzleRepository::default();

        // when set is created:
        let service = PuzzleServiceImpl::new(puzzle_repository);
        let create_set_result = service.create_set(options);

        // then error is returned:
        assert!(create_set_result.is_err())
    }

    #[test]
    fn should_disallow_creating_sets_with_size_too_large() {
        // given too large size:
        let options = sample_create_training_set_options()
            .size(1001)
            .build()
            .unwrap();

        // and repository:
        let puzzle_repository = MockPuzzleRepository::default();

        // when set is created:
        let service = PuzzleServiceImpl::new(puzzle_repository);
        let create_set_result = service.create_set(options);

        // then error is returned:
        assert!(create_set_result.is_err())
    }

    #[test]
    fn should_fail_when_creating_set_with_unmet_criteria() {
        // given valid criteria:
        let size = 20;
        let options = sample_create_training_set_options()
            .size(size)
            .build()
            .unwrap();

        // and repository that finds less than requested puzzles:
        let mut puzzle_repository = MockPuzzleRepository::default();
        repository_finds_random(&mut puzzle_repository, Some(10));

        // when set is created:
        let service = PuzzleServiceImpl::new(puzzle_repository);
        let create_set_result = service.create_set(options);

        // then error is returned:
        assert!(create_set_result.is_err())
    }
}
