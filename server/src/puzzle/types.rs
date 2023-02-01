use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};
use serde_with::formats::SpaceSeparator;
use serde_with::serde_as;
use serde_with::StringWithSeparator;
use strum::{Display as EnumDisplay, EnumString};

use crate::puzzle::puzzle_repository::CreatePuzzle;

pub type PuzzleId = u64;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(derive_builder::Builder))]
pub struct Puzzle {
    pub id: PuzzleId,
    pub fen: String,
    pub moves: String,
    pub lichess_id: String,
    pub lichess_rating: u16,
    pub lichess_rating_deviation: u16,
    pub lichess_popularity: i8,
    pub lichess_play_count: u32,
    pub themes: Vec<Theme>,
    pub lichess_game_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumDisplay, EnumString)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum Theme {
    AdvancedPawn,
    Advantage,
    AnastasiaMate,
    ArabianMate,
    #[strum(serialize = "attackingF2F7")]
    AttackingF2F7,
    Attraction,
    BackRankMate,
    BishopEndgame,
    BodenMate,
    CapturingDefender,
    Castling,
    Clearance,
    Crushing,
    DefensiveMove,
    Deflection,
    DiscoveredAttack,
    DoubleBishopMate,
    DoubleCheck,
    DovetailMate,
    EnPassant,
    Endgame,
    Equality,
    ExposedKing,
    Fork,
    HangingPiece,
    HookMate,
    Interference,
    Intermezzo,
    KingsideAttack,
    KnightEndgame,
    Long,
    Master,
    MasterVsMaster,
    Mate,
    MateIn1,
    MateIn2,
    MateIn3,
    MateIn4,
    MateIn5,
    Middlegame,
    OneMove,
    Opening,
    PawnEndgame,
    Pin,
    Promotion,
    QueenEndgame,
    QueenRookEndgame,
    QueensideAttack,
    QuietMove,
    RookEndgame,
    Sacrifice,
    Short,
    Skewer,
    SmotheredMate,
    #[strum(serialize = "superGM")]
    SuperGM,
    TrappedPiece,
    UnderPromotion,
    VeryLong,
    XRayAttack,
    Zugzwang,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeChoice {
    Themes(Vec<Theme>),
    HealthyMix,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(test, derive(derive_builder::Builder))]
pub struct LichessPuzzleImport {
    pub puzzle_id: String,
    #[serde(rename = "FEN")]
    pub fen: String,
    pub moves: String,
    pub rating: u16,
    pub rating_deviation: u16,
    pub popularity: i8,
    #[serde(rename = "NbPlays")]
    pub play_count: u32,
    #[serde_as(as = "StringWithSeparator::<SpaceSeparator, Theme>")]
    pub themes: Vec<Theme>,
    pub game_url: String,
}

impl From<LichessPuzzleImport> for CreatePuzzle {
    fn from(value: LichessPuzzleImport) -> Self {
        CreatePuzzle {
            fen: value.fen,
            moves: value.moves,
            lichess_id: value.puzzle_id,
            lichess_rating: value.rating,
            lichess_rating_deviation: value.rating_deviation,
            lichess_popularity: value.popularity,
            lichess_play_count: value.play_count,
            themes: value.themes,
            lichess_game_url: value.game_url,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrainingSet {
    pub puzzle_ids: Vec<PuzzleId>,
    pub name: String,
    pub rating: RangeInclusive<u16>,
    pub themes: ThemeChoice,
    pub current_progress: u32,
    pub cycles_done: u32,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(derive_builder::Builder))]
pub struct CreateTrainingSetOptions {
    pub name: String,
    pub size: usize,
    pub rating: RangeInclusive<u16>,
    pub themes: ThemeChoice,
}
