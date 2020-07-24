//! Analyzers for (challenge mote) fractal encounters.
use crate::{
    analyzers::{helpers, Analyzer, Outcome},
    Log,
};

/// Health threshold for Skorvald to be detected as Challenge Mote.
pub const SKORVALD_CM_HEALTH: u64 = 5_551_340;

/// Analyzer for the first boss of 100 CM, Skorvald.
///
/// The CM is detected by the boss's health, which is higher in the challenge mote.
#[derive(Debug, Clone, Copy)]
pub struct Skorvald<'log> {
    log: &'log Log,
}

impl<'log> Skorvald<'log> {
    /// Create a new [`Skorvald`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
    pub fn new(log: &'log Log) -> Self {
        Skorvald { log }
    }
}

impl<'log> Analyzer for Skorvald<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        helpers::boss_health(self.log)
            .map(|h| h >= SKORVALD_CM_HEALTH)
            .unwrap_or(false)
    }

    fn outcome(&self) -> Option<Outcome> {
        Outcome::from_bool(self.log.was_rewarded() || helpers::boss_is_dead(self.log))
    }
}

/// Analyzer for fractals that don't require special logic.
///
/// This is used for Artsariiv, Arkk, MAMA, Siax and Ensolyss.
#[derive(Debug, Clone, Copy)]
pub struct GenericFractal<'log> {
    log: &'log Log,
}

impl<'log> GenericFractal<'log> {
    /// Create a new [`GenericFractal`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
    pub fn new(log: &'log Log) -> Self {
        GenericFractal { log }
    }
}

impl<'log> Analyzer for GenericFractal<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        // Besides Skorvald normal mode, we only get logs for the challenge mote encounters (at
        // least, only for those we'll use this analyzer). So we can safely return true here in any
        // case.
        true
    }

    fn outcome(&self) -> Option<Outcome> {
        Outcome::from_bool(self.log.was_rewarded() || helpers::boss_is_dead(self.log))
    }
}
