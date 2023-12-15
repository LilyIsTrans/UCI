#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::MoveList;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GoCommand {
    /// The standard calls this `searchmoves`
    restrict_first_move_to: Option<MoveList>,
    /// If true, the engine should consider the last move in the previous `position` command a suggestion for which move to ponder on, and may begin preemptively searching
    ponder: bool,
    wtime: Option<usize>,
    btime: Option<usize>,
    winc: Option<usize>,
    binc: Option<usize>,
    movestogo: Option<usize>,
    depth: Option<usize>,
    nodes: Option<usize>,
    mate: Option<usize>,
    movetime: Option<usize>,
    infinite: bool,
}
