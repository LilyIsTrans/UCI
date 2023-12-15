#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct EngineOption {
    pub name: Box<[u8]>,
    pub data: OptionType,
}

/// Also contains the default value, and any variants!
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum OptionType {
    CheckBox(bool),
    /// Called `spin` in the standard
    Integer {
        min: isize,
        max: isize,
        default: isize,
    },
    /// Called `combo` in the standard. Avoid using spaces here, it might break things.
    Dropdown {
        /// A list of the variants
        variants: Box<[Box<[u8]>]>,
        default: Box<[u8]>,
    },
    Button,
    String(Box<[u8]>),
}
