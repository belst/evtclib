//! Analyzers for (challenge mote) fractal encounters.
use crate::{
    analyzers::{helpers, Analyzer, Outcome},
    Log,
};

/// Health threshold for Skorvald to be detected as Challenge Mote.
pub const SKORVALD_CM_HEALTH: u64 = 5_551_340;

/// Character IDs for the anomalies in Skorvald's Challenge Mote.
pub static SKORVALD_CM_ANOMALY_IDS: &[u16] = &[17_599, 17_673, 17_770, 17_851];

/// Analyzer for the first boss of 100 CM, Skorvald.
///
/// The CM was detected by the boss's health, which was higher in the challenge mote.
///
/// The 2020-09-15 update which introduced a new fractal and shifted Shattered Observator CM to 99
/// which changed the bosses' maximal health, so this method no longer works. Instead, we rely on
/// the split phase to differentiate the "normal mode" flux anomalies from the "challenge mode"
/// flux anomalies, with the downside that the CM detection is only working if players make it to
/// the split phase.
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
        // Shortcut for old logs for which this method still works.
        if Some(true) == helpers::boss_health(self.log).map(|h| h >= SKORVALD_CM_HEALTH) {
            return true;
        }

        self.log
            .npcs()
            .any(|character| SKORVALD_CM_ANOMALY_IDS.contains(&character.id()))
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
