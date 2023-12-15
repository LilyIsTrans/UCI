#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Move;

use crate::MoveList;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PV {
    pub(crate) depth: Option<usize>,
    pub(crate) seldepth: Option<usize>,
    pub(crate) time: Option<usize>,
    pub(crate) nodes: Option<usize>,
    pub(crate) multi_pv_rank: Option<usize>,
    pub(crate) score: Option<Score>,
    pub(crate) moves: MoveList,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum ScoreBound {
    Upper,
    Lower,
    Exact,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum ScoreType {
    CentiPawns,
    ForcedMateMoves,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Score {
    pub(crate) bound_type: ScoreBound,
    pub(crate) score_type: ScoreType,
    pub(crate) value: isize,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Info {
    PV(PV),
    CurrentMove(Move),
    CurrentMoveNumber(usize),
    HashTableUse(u16),
    NodesPerSecond(usize),
    TableBaseHits(usize),
    ShredderBaseHits(usize),
    CPULoad(u16),
    String(Box<str>),
    Refutation(MoveList),
    CurrentLine(Option<usize>, MoveList),
}
