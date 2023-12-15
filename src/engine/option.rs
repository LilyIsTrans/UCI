#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct EngineOption {
    name: Box<str>,
    data: OptionType,
}

/// Also contains the default value, and any variants!
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Hash, PartialEq, Eq)]
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
        variants: Box<[Box<str>]>,
        default: Box<str>,
    },
    Button,
    String(Box<str>),
}
