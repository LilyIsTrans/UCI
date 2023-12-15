#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Registration {
    Ok,
    Checking,
    Error,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum CopyProtection {
    Ok,
    Checking,
    Error,
}
