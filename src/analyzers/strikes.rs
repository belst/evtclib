//! Analyzers for Strike Mission logs.
use crate::{
    analyzers::{helpers, Analyzer, Outcome},
    Log,
};

/// Analyzer for strikes.
///
/// Since there are currently no strikes requiring special logic, this analyzer is used for all
/// strike missions.
#[derive(Debug, Clone, Copy)]
pub struct GenericStrike<'log> {
    log: &'log Log,
}

impl<'log> GenericStrike<'log> {
    /// Create a new [`GenericStrike`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
    pub fn new(log: &'log Log) -> Self {
        GenericStrike { log }
    }
}

impl<'log> Analyzer for GenericStrike<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        false
    }

    fn outcome(&self) -> Option<Outcome> {
        Outcome::from_bool(helpers::boss_is_dead(self.log))
    }
}
