use std::ops::RangeInclusive;

use crate::puzzle::types::{PuzzleId, ThemeChoice, TrainingSet};

#[cfg_attr(test, mockall::automock)]
pub trait TrainingSetRepository {
    fn create(&self, training_set: CreateTrainingSet) -> anyhow::Result<TrainingSet>;
}

pub struct CreateTrainingSet {
    pub puzzle_ids: Vec<PuzzleId>,
    pub name: String,
    pub rating: RangeInclusive<u16>,
    pub themes: ThemeChoice,
    pub current_progress: u32,
    pub cycles_done: u32,
}

pub struct DummyTrainingSetRepository;

impl TrainingSetRepository for DummyTrainingSetRepository {
    fn create(&self, _training_set: CreateTrainingSet) -> anyhow::Result<TrainingSet> {
        unimplemented!()
    }
}
