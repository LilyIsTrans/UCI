#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::MoveList;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Hash, PartialEq, Eq)]
pub enum OptionType {
    CheckBox(bool),
    /// Called `spin` in the standard
    Integer(isize),
    /// Called `combo` in the standard. Avoid using spaces here, it might break things.
    Dropdown(Box<str>),
    Button,
    String(Box<str>),
}
/// A single command, sent from the GUI (or other UCI host) to an engine
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Hash, PartialEq, Eq)]
pub enum GUICommand {
    UCI,
    /// `true` indicates debug mode on, `false` indicates debug mode off
    Debug(bool),
    /// Used to wait for the engine to complete potentially blocking operations. Must wait for `EngineCommand::ReadyOk`. If the built in interface is used, this command will be handled by the interface.
    IsReady,
    SetOption {
        name: Box<str>,
        value: OptionType,
    },
    /// Optional feature. Tells the engine the next `position` command will be from a different game.
    UCINewGame,
    Go(go::GoCommand),
    PonderHit,
    Position(Position),
    Quit,
    Stop,
    Register(Option<Register>),
}

pub mod go;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Hash, PartialEq, Eq)]
pub enum PositionBase {
    /// The default chess start position
    StartPos,
    /// FEN String
    FEN(Box<str>),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Position {
    base: PositionBase,
    moves: MoveList,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Register {
    name: Box<str>,
    code: Box<str>,
}
