use crate::engine::EngineCommand;

use crate::gui::GUICommand;
use tracing::instrument;

/// Assumes the partner is being nice and sending one command per line
#[instrument(level = "trace")]
pub(super) fn parse_line_from_gui(line: &str) -> GUICommand {
    todo!()
}

/// Assumes the partner is being nice and sending one command per line
#[instrument(level = "trace")]
pub(super) fn parse_line_from_engine(line: &str) -> EngineCommand {
    todo!()
}
