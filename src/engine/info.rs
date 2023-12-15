#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Move;

use crate::MoveList;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PV {
    pub depth: Option<usize>,
    pub seldepth: Option<usize>,
    pub time: Option<usize>,
    pub nodes: Option<usize>,
    pub multi_pv_rank: Option<usize>,
    pub score: Option<Score>,
    pub moves: MoveList,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ScoreBound {
    Upper,
    Lower,
    Exact,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ScoreType {
    CentiPawns,
    ForcedMateMoves,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Score {
    pub bound_type: ScoreBound,
    pub score_type: ScoreType,
    pub value: isize,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Info {
    PV(PV),
    CurrentMove(Move),
    CurrentMoveNumber(usize),
    HashTableUse(u16),
    NodesPerSecond(usize),
    TableBaseHits(usize),
    ShredderBaseHits(usize),
    CPULoad(u16),
    String(Box<[u8]>),
    Refutation(MoveList),
    CurrentLine(Option<usize>, MoveList),
}
