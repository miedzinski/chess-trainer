pub use config::make_service;
pub use rest::make_router;
pub use service::PuzzleService;

mod config;
pub(self) mod consts;
pub mod errors;
mod puzzle_repository;
mod rest;
mod service;
mod training_set_repository;
pub mod types;
