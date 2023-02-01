use anyhow::ensure;

use crate::puzzle::consts::{MAX_SET_NAME_LENGTH, MAX_SET_SIZE, MIN_SET_SIZE};
use crate::puzzle::errors::CreateTrainingSetError;
use crate::puzzle::puzzle_repository::PuzzleRepository;
use crate::puzzle::training_set_repository;
use crate::puzzle::training_set_repository::TrainingSetRepository;
use crate::puzzle::types::{CreateTrainingSetOptions, LichessPuzzleImport, Puzzle, TrainingSet};

pub trait PuzzleService {
    fn import_puzzle(&self, lichess_puzzle: LichessPuzzleImport) -> anyhow::Result<Puzzle>;
    fn list_puzzles(&self) -> Vec<Puzzle>;
    fn create_set(
        &self,
        options: CreateTrainingSetOptions,
    ) -> Result<TrainingSet, CreateTrainingSetError>;
}

#[cfg_attr(test, derive(derive_builder::Builder))]
#[cfg_attr(test, builder(pattern = "owned"))]
pub struct PuzzleServiceImpl<P, T>
where
    P: PuzzleRepository,
    T: TrainingSetRepository,
{
    puzzle_repository: P,
    training_set_repository: T,
}

impl<P, T> PuzzleServiceImpl<P, T>
where
    P: PuzzleRepository,
    T: TrainingSetRepository,
{
    pub fn new(puzzle_repository: P, training_set_repository: T) -> PuzzleServiceImpl<P, T> {
        PuzzleServiceImpl {
            puzzle_repository,
            training_set_repository,
        }
    }
}

impl<P, T> PuzzleService for PuzzleServiceImpl<P, T>
where
    P: PuzzleRepository,
    T: TrainingSetRepository,
{
    fn import_puzzle(&self, lichess_puzzle: LichessPuzzleImport) -> anyhow::Result<Puzzle> {
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

    fn create_set(
        &self,
        options: CreateTrainingSetOptions,
    ) -> Result<TrainingSet, CreateTrainingSetError> {
        if options.name.is_empty() {
            return Err(CreateTrainingSetError::EmptyName);
        }
        if options.name.len() > MAX_SET_NAME_LENGTH {
            return Err(CreateTrainingSetError::NameLengthLimitExceeded);
        }
        if options.size < MIN_SET_SIZE {
            return Err(CreateTrainingSetError::SizeTooSmall);
        }
        if options.size > MAX_SET_SIZE {
            return Err(CreateTrainingSetError::SizeLimitExceeded);
        }

        let puzzles = self
            .puzzle_repository
            .find_random(options.size, &options.rating, &options.themes)
            .map_err(|source| CreateTrainingSetError::RepositoryError { source })?;

        if puzzles.len() != options.size {
            return Err(CreateTrainingSetError::CriteriaUnmet);
        }

        let puzzle_ids = puzzles.iter().map(|puzzle| puzzle.id).collect();

        let create_set = training_set_repository::CreateTrainingSet {
            puzzle_ids,
            name: options.name,
            rating: options.rating,
            themes: options.themes,
            current_progress: 0,
            cycles_done: 0,
        };
        self.training_set_repository
            .create(create_set)
            .map_err(|source| CreateTrainingSetError::RepositoryError { source })
    }
}

#[cfg(test)]
mod tests {
    use std::iter::repeat_with;

    use parking_lot::Mutex;
    use uuid::uuid;

    use crate::puzzle::errors::CreateTrainingSetError;
    use crate::puzzle::puzzle_repository::MockPuzzleRepository;
    use crate::puzzle::service::PuzzleServiceImplBuilder;
    use crate::puzzle::training_set_repository::MockTrainingSetRepository;
    use crate::puzzle::types::{
        CreateTrainingSetOptions, CreateTrainingSetOptionsBuilder, LichessPuzzleImportBuilder,
        Puzzle, PuzzleBuilder, PuzzleId, Theme, ThemeChoice, TrainingSet, TrainingSetId,
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

    fn sample_training_set_id() -> TrainingSetId {
        uuid!("e649d0cc-3244-483d-922a-e8269d006ffe")
    }

    fn make_service() -> PuzzleServiceImplBuilder<MockPuzzleRepository, MockTrainingSetRepository> {
        PuzzleServiceImplBuilder::default()
            .puzzle_repository(MockPuzzleRepository::new())
            .training_set_repository(MockTrainingSetRepository::new())
    }

    fn stub_puzzle_repository_creates(puzzle_repository: &mut MockPuzzleRepository) {
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

    fn stub_puzzle_repository_finds_random(
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

    fn stub_set_repository_creates(training_set_repository: &mut MockTrainingSetRepository) {
        training_set_repository
            .expect_create()
            .returning_st(move |set| {
                Ok(TrainingSet {
                    id: sample_training_set_id(),
                    puzzle_ids: set.puzzle_ids,
                    name: set.name,
                    rating: set.rating,
                    themes: set.themes,
                    current_progress: set.current_progress,
                    cycles_done: set.cycles_done,
                })
            });
    }

    #[test]
    fn should_import_lichess_puzzle() {
        // given Lichess puzzle:
        let lichess_puzzle = sample_lichess_puzzle().build().unwrap();

        // and repository that saves puzzles:
        let mut puzzle_repository = MockPuzzleRepository::new();
        stub_puzzle_repository_creates(&mut puzzle_repository);

        // when Lichess puzzle is imported:
        let service = make_service()
            .puzzle_repository(puzzle_repository)
            .build()
            .unwrap();
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
        stub_puzzle_repository_finds_random(&mut puzzle_repository, None);

        // and repository that creates sets:
        let mut training_set_repository = MockTrainingSetRepository::default();
        stub_set_repository_creates(&mut training_set_repository);

        // when set is created:
        let service = make_service()
            .puzzle_repository(puzzle_repository)
            .training_set_repository(training_set_repository)
            .build()
            .unwrap();
        let set = service.create_set(options.clone()).unwrap();

        // then it has correct data:
        let expected = TrainingSet {
            id: sample_training_set_id(),
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
    fn should_disallow_creating_sets_with_name_empty() {
        // given too long name:
        let options = sample_create_training_set_options()
            .name("".to_string())
            .build()
            .unwrap();

        // when set is created:
        let service = make_service().build().unwrap();
        let create_set_result = service.create_set(options);

        // then error is returned:
        assert!(matches!(
            create_set_result,
            Err(CreateTrainingSetError::EmptyName)
        ));
    }

    #[test]
    fn should_disallow_creating_sets_with_name_too_long() {
        // given too long name:
        let name: String = repeat_with(|| "a").take(101).collect();
        let options = sample_create_training_set_options()
            .name(name)
            .build()
            .unwrap();

        // when set is created:
        let service = make_service().build().unwrap();
        let create_set_result = service.create_set(options);

        // then error is returned:
        assert!(matches!(
            create_set_result,
            Err(CreateTrainingSetError::NameLengthLimitExceeded)
        ));
    }

    #[test]
    fn should_disallow_creating_sets_with_size_too_small() {
        // given too small size:
        let options = sample_create_training_set_options()
            .size(4)
            .build()
            .unwrap();

        // when set is created:
        let service = make_service().build().unwrap();
        let create_set_result = service.create_set(options);

        // then error is returned:
        assert!(matches!(
            create_set_result,
            Err(CreateTrainingSetError::SizeTooSmall)
        ));
    }

    #[test]
    fn should_disallow_creating_sets_with_size_too_large() {
        // given too large size:
        let options = sample_create_training_set_options()
            .size(1001)
            .build()
            .unwrap();

        // when set is created:
        let service = make_service().build().unwrap();
        let create_set_result = service.create_set(options);

        // then error is returned:
        assert!(matches!(
            create_set_result,
            Err(CreateTrainingSetError::SizeLimitExceeded)
        ));
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
        stub_puzzle_repository_finds_random(&mut puzzle_repository, Some(10));

        // when set is created:
        let service = make_service()
            .puzzle_repository(puzzle_repository)
            .build()
            .unwrap();
        let create_set_result = service.create_set(options);

        // then error is returned:
        assert!(matches!(
            create_set_result,
            Err(CreateTrainingSetError::CriteriaUnmet)
        ));
    }
}
