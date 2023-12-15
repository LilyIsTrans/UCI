#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::MoveList;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GoCommand {
    /// The standard calls this `searchmoves`
    pub restrict_first_move_to: Option<MoveList>,
    /// If true, the engine should consider the last move in the previous `position` command a suggestion for which move to ponder on, and may begin preemptively searching
    pub ponder: bool,
    pub wtime: Option<usize>,
    pub btime: Option<usize>,
    pub winc: Option<usize>,
    pub binc: Option<usize>,
    pub movestogo: Option<usize>,
    pub depth: Option<usize>,
    pub nodes: Option<usize>,
    pub mate: Option<usize>,
    pub movetime: Option<usize>,
    pub infinite: bool,
}
