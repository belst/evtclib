//! This module defines raw types that correspond 1:1 to the C types used in
//! [arcdps](https://www.deltaconnected.com/arcdps/evtc/README.txt).
//!
//! It is not advised to use those types and functions, as dealing with all the
//! low-level details can be quite tedious. Instead, use the higher-level
//! functions whenever possible.
mod types;

pub use self::types::{
    Agent, CbtActivation, CbtBuffRemove, CbtCustomSkill, CbtEvent, CbtResult, CbtStateChange,
    Language, Skill, IFF,
};

pub mod parser;

pub use self::parser::{parse_file, Evtc, ParseError};
