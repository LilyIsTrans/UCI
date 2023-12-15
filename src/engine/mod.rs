use crate::Move;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod authentication;
pub mod info;
pub mod option;

/// A single command, sent from the Engine

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum EngineCommand {
    Registration(authentication::Registration),
    CopyProtection(authentication::CopyProtection),
    BestMove(BestMove),
    ID(ID),
    Info(info::Info),
    Option(option::EngineOption),
    ReadyOk,
    UCIOk,
    /// Not really a UCI command. Indicates that the stream has reached EOF, most likely meaning that the engine has unexpectedly crashed.
    EOF,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct BestMove {
    pub best: Move,
    pub ponder: Option<Move>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ID {
    Author(Box<[u8]>),
    Name(Box<[u8]>),
}
