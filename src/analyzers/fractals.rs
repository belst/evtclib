//! Analyzers for (challenge mote) fractal encounters.
use crate::{
    analyzers::{helpers, Analyzer, Outcome},
    Boss, EventKind, Log,
};

/// The ID of the invulnerability buff that Ai gets when she has been defeated.
pub const AI_INVULNERABILITY_ID: u32 = 895;
/// The ID of the skill with which we determine when Ai has phased.
pub const AI_PHASE_SKILL: u32 = 53_569;
/// The ID of the skill with which we determine Ai has the dark phase fight.
pub const AI_HAS_DARK_MODE_SKILL: u32 = 61_356;

/// Gets the timestamp when the second phase of Ai starts.
///
/// If the log is missing dark phase, `None` is returned.
///
/// If the whole log is in dark phase, `Some(0)` is returned.
fn get_dark_phase_start(log: &Log) -> Option<u64> {
    // Determine if we even have a dark phase.
    if !log.events().iter().any(|event| {
        if let EventKind::SkillUse { skill_id, .. } = event.kind() {
            *skill_id == AI_HAS_DARK_MODE_SKILL
        } else {
            false
        }
    }) {
        return None;
    };

    // If we are here, either the whole log is in dark mode, or we phased.
    let mut dark_phase_start = None;
    for event in log.events() {
        if let EventKind::SkillUse { skill_id, .. } = event.kind() {
            if *skill_id == AI_PHASE_SKILL {
                dark_phase_start = Some(event.time());
            }
        }
    }

    dark_phase_start.or(Some(0))
}

/// Analyzer for the fight of 100 CM, Ai, Keeper of the Peak.
///
/// This fight is special in that it consists of two phases, and the bosses each count as "success"
/// when they reach 1% health, i.e. they don't die.
#[derive(Debug, Clone, Copy)]
pub struct Ai<'log> {
    log: &'log Log,
}

impl<'log> Ai<'log> {
    /// Create a new [`Ai`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
    pub fn new(log: &'log Log) -> Self {
        Ai { log }
    }
}

impl<'log> Analyzer for Ai<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        // We assume that every Ai log is from CM, like the other fractal logs.
        true
    }

    fn outcome(&self) -> Option<Outcome> {
        let dark_phase_start = get_dark_phase_start(self.log);
        if dark_phase_start.is_none() {
            return Some(Outcome::Failure);
        }

        let dark_phase_start = dark_phase_start.unwrap();

        for event in self.log.events() {
            // Make sure we only count the invulnerability in dark phase
            if event.time() < dark_phase_start {
                continue;
            }
            if let EventKind::BuffApplication {
                buff_id,
                destination_agent_addr,
                ..
            } = event.kind()
            {
                let agent = self
                    .log
                    .agent_by_addr(*destination_agent_addr)
                    .and_then(|a| a.as_character());
                if let Some(c) = agent {
                    if c.id() == Boss::Ai as u16 && *buff_id == AI_INVULNERABILITY_ID {
                        return Some(Outcome::Success);
                    }
                }
            }
        }

        Some(Outcome::Failure)
    }
}

/// Health threshold for Skorvald to be detected as Challenge Mote.
pub const SKORVALD_CM_HEALTH: u64 = 5_551_340;

/// Character IDs for the anomalies in Skorvald's Challenge Mote.
pub static SKORVALD_CM_ANOMALY_IDS: &[u16] = &[17_599, 17_673, 17_770, 17_851];

/// Analyzer for the first boss of 99 CM, Skorvald.
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
