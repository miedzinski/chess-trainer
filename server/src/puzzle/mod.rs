pub use config::make_service;
pub use rest::make_router;
pub use service::PuzzleService;

mod config;
pub(self) mod consts;
pub mod errors;
mod puzzle_repository;
mod rest;
mod service;
pub mod types;
