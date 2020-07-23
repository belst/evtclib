//! Boss fight analyzers for Wing 6 (Mythwright Gambit)
use crate::{
    analyzers::{helpers, Analyzer, Outcome},
    Log,
};

pub const ADINA_CM_HEALTH: u64 = 24_800_000;

/// Analyzer for the first fight of Wing 7, Cardinal Adina.
///
/// The CM is detected by the boss's health, which is higher in the challenge mote.
#[derive(Debug, Clone, Copy)]
pub struct CardinalAdina<'log> {
    log: &'log Log,
}

impl<'log> CardinalAdina<'log> {
    pub fn new(log: &'log Log) -> Self {
        CardinalAdina { log }
    }
}

impl<'log> Analyzer for CardinalAdina<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        helpers::boss_health(self.log)
            .map(|h| h >= ADINA_CM_HEALTH)
            .unwrap_or(false)
    }

    fn outcome(&self) -> Option<Outcome> {
        Outcome::from_bool(helpers::boss_is_dead(self.log))
    }
}

pub const SABIR_CM_HEALTH: u64 = 32_400_000;

/// Analyzer for the second fight of Wing 7, Cardinal Sabir.
///
/// The CM is detected by the boss's health, which is higher in the challenge mote.
#[derive(Debug, Clone, Copy)]
pub struct CardinalSabir<'log> {
    log: &'log Log,
}

impl<'log> CardinalSabir<'log> {
    pub fn new(log: &'log Log) -> Self {
        CardinalSabir { log }
    }
}

impl<'log> Analyzer for CardinalSabir<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        helpers::boss_health(self.log)
            .map(|h| h >= SABIR_CM_HEALTH)
            .unwrap_or(false)
    }

    fn outcome(&self) -> Option<Outcome> {
        Outcome::from_bool(helpers::boss_is_dead(self.log))
    }
}

pub const QADIMP_CM_HEALTH: u64 = 51_000_000;

#[derive(Debug, Clone, Copy)]
pub struct QadimThePeerless<'log> {
    log: &'log Log,
}

impl<'log> QadimThePeerless<'log> {
    pub fn new(log: &'log Log) -> Self {
        QadimThePeerless { log }
    }
}

impl<'log> Analyzer for QadimThePeerless<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        helpers::boss_health(self.log)
            .map(|h| h >= QADIMP_CM_HEALTH)
            .unwrap_or(false)
    }

    fn outcome(&self) -> Option<Outcome> {
        Outcome::from_bool(helpers::boss_is_dead(self.log))
    }
}
