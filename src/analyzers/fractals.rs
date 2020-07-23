//! Analyzers for (challenge mote) fractal encounters.
use crate::{
    analyzers::{helpers, Analyzer, Outcome},
    Log,
};

pub const SKORVALD_CM_HEALTH: u64 = 5_551_340;

/// Analyzer for the first boss of 100 CM, Skorvald.
///
/// The CM is detected by the boss's health, which is higher in the challenge mote.
#[derive(Debug, Clone, Copy)]
pub struct Skorvald<'log> {
    log: &'log Log,
}

impl<'log> Skorvald<'log> {
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

#[derive(Debug, Clone, Copy)]
pub struct GenericFractal<'log> {
    log: &'log Log,
}

impl<'log> GenericFractal<'log> {
    pub fn new(log: &'log Log) -> Self {
        GenericFractal { log }
    }
}

impl<'log> Analyzer for GenericFractal<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        true
    }

    fn outcome(&self) -> Option<Outcome> {
        Outcome::from_bool(self.log.was_rewarded() || helpers::boss_is_dead(self.log))
    }
}
