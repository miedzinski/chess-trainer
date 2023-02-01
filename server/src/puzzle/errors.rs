use crate::puzzle::consts::{MAX_SET_NAME_LENGTH, MAX_SET_SIZE, MIN_SET_SIZE};

#[derive(Debug, thiserror::Error)]
pub enum CreateTrainingSetError {
    #[error("Set name can't be blank.")]
    EmptyName,
    #[error("Set name length can't exceed {}.", MAX_SET_NAME_LENGTH)]
    NameLengthLimitExceeded,
    #[error("Set size must be at least {}.", MIN_SET_SIZE)]
    SizeTooSmall,
    #[error("Set size can't exceed {}.", MAX_SET_SIZE)]
    SizeLimitExceeded,
    #[error("Not enough puzzles meet the criteria given.")]
    CriteriaUnmet,
    #[error("Repository error.")]
    RepositoryError { source: anyhow::Error },
}
