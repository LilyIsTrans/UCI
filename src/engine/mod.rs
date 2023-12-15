use crate::Move;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod authentication;
pub mod info;
pub mod option;

/// A single command, sent from the Engine

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Hash, PartialEq, Eq)]
pub enum EngineCommand {
    Registration(authentication::Registration),
    CopyProtection(authentication::CopyProtection),
    BestMove(BestMove),
    ID(ID),
    Info(info::Info),
    Option(option::EngineOption),
    ReadyOk,
    UCIOk,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct BestMove {
    best: Move,
    ponder: Option<Move>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Hash, PartialEq, Eq)]
pub enum ID {
    Author(Box<str>),
    Name(Box<str>),
}
