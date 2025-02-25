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
    /// Create a new [`CardinalAdina`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
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
        check_reward!(self.log);
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
    /// Create a new [`CardinalSabir`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
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
        check_reward!(self.log);
        Outcome::from_bool(helpers::boss_is_dead(self.log))
    }
}

pub const QADIMP_CM_HEALTH: u64 = 51_000_000;

/// Analyzer for the final fight of Wing 7, Qadim The Peerless.
#[derive(Debug, Clone, Copy)]
pub struct QadimThePeerless<'log> {
    log: &'log Log,
}

impl<'log> QadimThePeerless<'log> {
    /// Create a new [`QadimThePeerless`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
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
        check_reward!(self.log);
        Outcome::from_bool(helpers::boss_is_dead(self.log))
    }
}
