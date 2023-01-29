use std::env;

use anyhow::Context;

use chess_trainer::puzzle::make_service;
use chess_trainer::puzzle::types::LichessPuzzleImport;
use chess_trainer::puzzle::PuzzleService;

fn main() -> anyhow::Result<()> {
    let path = env::args().nth(1).context("missing input file")?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(path)?;
    let mut puzzle_service = make_service();

    let count = reader
        .deserialize()
        .map(|record: Result<LichessPuzzleImport, _>| puzzle_service.import_puzzle(record?))
        .filter(Result::is_ok)
        .count();

    println!("imported {} puzzles", count);

    Ok(())
}
