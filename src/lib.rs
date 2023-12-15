#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
pub mod engine;
pub mod gui;
use thiserror::Error;
use tracing::warn;

pub mod interface;

pub type MoveList = Box<[Move]>;
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub struct Square(u8);
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub struct Move(Square, Square);

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file: u8 = b'a' + (self.0 & 0b111);
        let rank: u8 = (self.0 & 0b111000) >> 3;
        if !((b'a'..=b'h').contains(&file) && (0u8..=7u8).contains(&rank)) {
            warn!("Invalid Square!");
        };
        file.escape_ascii().fmt(f)?;
        rank.fmt(f)
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)?;
        self.1.fmt(f)
    }
}

#[derive(Clone, Debug, Error)]
pub enum SquareError {
    #[error("{} is not a valid file!", (.0 + b'a').escape_ascii())]
    InvalidFile(u8),
    #[error("{0} is not a valid rank!")]
    InvalidRank(u8),
    #[error("File not found")]
    NoFile,
    #[error("Rank not found")]
    NoRank,
}

impl FromStr for Square {
    type Err = SquareError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = s.bytes();
        let file = bytes.next().ok_or(SquareError::NoFile)? - b'a';
        let rank = bytes.next().ok_or(SquareError::NoRank)? - b'0';
        if !(0u8..=7u8).contains(&file) {
            Err(SquareError::InvalidFile(file))
        } else if !(0u8..=7u8).contains(&rank) {
            Err(SquareError::InvalidRank(rank))
        } else {
            Ok(Self(file & (rank << 3)))
        }
    }
}

impl FromStr for Move {
    type Err = SquareError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let from_square: Square = s[0..2].parse()?;
        let to_square: Square = s[2..4].parse()?;
        Ok(Self(from_square, to_square))
    }
}
