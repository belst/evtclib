//! Boss fight analyzers for Wing 5 (Hall of Chains)
use crate::{
    analyzers::{helpers, Analyzer, Outcome},
    EventKind, Log,
};

pub const DESMINA_BUFF_ID: u32 = 47414;
pub const DESMINA_MS_THRESHOLD: u64 = 11_000;
pub const DESMINA_DEATH_BUFF: u32 = 895;

/// Analyzer for the first fight of Wing 5, Soulless Horror (aka. Desmina).
///
/// The CM is detected by the time between applications of the Necrosis debuff, which is applied at
/// a faster rate when the challenge mote is active.
#[derive(Debug, Clone, Copy)]
pub struct SoullessHorror<'log> {
    log: &'log Log,
}

impl<'log> SoullessHorror<'log> {
    /// Create a new [`SoullessHorror`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
    pub fn new(log: &'log Log) -> Self {
        SoullessHorror { log }
    }
}

impl<'log> Analyzer for SoullessHorror<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        let tbb = helpers::time_between_buffs(self.log, DESMINA_BUFF_ID);
        tbb > 0 && tbb <= DESMINA_MS_THRESHOLD
    }

    fn outcome(&self) -> Option<Outcome> {
        check_reward!(self.log);
        Outcome::from_bool(self.log.events().iter().any(|event| {
            if let EventKind::BuffApplication {
                buff_id,
                destination_agent_addr,
                ..
            } = event.kind()
            {
                self.log.is_boss(*destination_agent_addr) && *buff_id == DESMINA_DEATH_BUFF
            } else {
                false
            }
        }))
    }
}

pub const DHUUM_CM_HEALTH: u64 = 40_000_000;

/// Analyzer for the second fight of Wing 5, Dhuum.
///
/// The CM is detected by the boss's health, which is higher in the challenge mote.
#[derive(Debug, Clone, Copy)]
pub struct Dhuum<'log> {
    log: &'log Log,
}

impl<'log> Dhuum<'log> {
    /// Create a new [`Dhuum`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
    pub fn new(log: &'log Log) -> Self {
        Dhuum { log }
    }
}

impl<'log> Analyzer for Dhuum<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        helpers::boss_health(self.log)
            .map(|h| h >= DHUUM_CM_HEALTH)
            .unwrap_or(false)
    }

    fn outcome(&self) -> Option<Outcome> {
        check_reward!(self.log);
        Outcome::from_bool(helpers::boss_is_dead(self.log))
    }
}
