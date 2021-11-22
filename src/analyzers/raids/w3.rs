use crate::{
    analyzers::{helpers, Analyzer, Outcome},
    Log,
};

/// Analyzer for the final fight of Wing 3, Xera.
#[derive(Debug, Clone, Copy)]
pub struct TwistedCastle<'log> {
    log: &'log Log,
}

impl<'log> TwistedCastle<'log> {
    /// Create a new [`TwistedCastle`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
    pub fn new(log: &'log Log) -> Self {
        TwistedCastle { log }
    }
}

impl<'log> Analyzer for TwistedCastle<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        false
    }

    fn outcome(&self) -> Option<Outcome> {
        Outcome::from_bool(self.log.was_rewarded())
    }
}

/// Analyzer for the final fight of Wing 3, Xera.
#[derive(Debug, Clone, Copy)]
pub struct Xera<'log> {
    log: &'log Log,
}

impl<'log> Xera<'log> {
    /// Create a new [`Xera`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
    pub fn new(log: &'log Log) -> Self {
        Xera { log }
    }
}

impl<'log> Analyzer for Xera<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        false
    }

    fn outcome(&self) -> Option<Outcome> {
        check_reward!(self.log);
        Outcome::from_bool(helpers::players_exit_after_boss(self.log))
    }
}
