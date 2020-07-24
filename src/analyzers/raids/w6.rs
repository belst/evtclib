//! Boss fight analyzers for Wing 6 (Mythwright Gambit)
use crate::{
    analyzers::{helpers, Analyzer, Outcome},
    gamedata::{KENUT_ID, NIKARE_ID},
    EventKind, Log,
};

pub const CA_CM_BUFF: u32 = 53_075;
pub const ZOMMOROS_ID: u16 = 21_118;

/// Analyzer for the first fight of Wing 6, Conjured Amalgamate.
///
/// The CM is detected by the presence of the buff that the player targeted by the laser has.
#[derive(Debug, Clone, Copy)]
pub struct ConjuredAmalgamate<'log> {
    log: &'log Log,
}

impl<'log> ConjuredAmalgamate<'log> {
    /// Create a new [`ConjuredAmalgamate`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
    pub fn new(log: &'log Log) -> Self {
        ConjuredAmalgamate { log }
    }
}

impl<'log> Analyzer for ConjuredAmalgamate<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        helpers::buff_present(self.log, CA_CM_BUFF)
    }

    fn outcome(&self) -> Option<Outcome> {
        for event in self.log.events() {
            if let EventKind::Spawn { agent_addr } = event.kind() {
                if self
                    .log
                    .agent_by_addr(*agent_addr)
                    .and_then(|a| a.as_character())
                    .map(|a| a.id() == ZOMMOROS_ID)
                    .unwrap_or(false)
                {
                    return Some(Outcome::Success);
                }
            }
        }
        Some(Outcome::Failure)
    }
}

pub const LARGOS_CM_HEALTH: u64 = 19_200_000;

/// Analyzer for the second fight of Wing 6, Largos Twins.
///
/// The CM is detected by the boss's health, which is higher in the challenge mote.
#[derive(Debug, Clone, Copy)]
pub struct LargosTwins<'log> {
    log: &'log Log,
}

impl<'log> LargosTwins<'log> {
    /// Create a new [`LargosTwins`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
    pub fn new(log: &'log Log) -> Self {
        LargosTwins { log }
    }
}

impl<'log> Analyzer for LargosTwins<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        helpers::boss_health(self.log)
            .map(|h| h >= LARGOS_CM_HEALTH)
            .unwrap_or(false)
    }

    fn outcome(&self) -> Option<Outcome> {
        let mut nikare_dead = false;
        let mut kenut_dead = false;

        for event in self.log.events() {
            if let EventKind::ChangeDead { agent_addr } = event.kind() {
                let agent = if let Some(agent) = self
                    .log
                    .agent_by_addr(*agent_addr)
                    .and_then(|a| a.as_character())
                {
                    agent
                } else {
                    continue;
                };

                if agent.id() == NIKARE_ID {
                    nikare_dead = true;
                } else if agent.id() == KENUT_ID {
                    kenut_dead = true;
                }
            }
        }

        Outcome::from_bool(kenut_dead && nikare_dead)
    }
}

pub const QADIM_CM_HEALTH: u64 = 21_100_000;

/// Analyzer for the third fight of Wing 6, Qadim.
///
/// The CM is detected by the boss's health, which is higher in the challenge mote.
#[derive(Debug, Clone, Copy)]
pub struct Qadim<'log> {
    log: &'log Log,
}

impl<'log> Qadim<'log> {
    /// Create a new [`Qadim`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
    pub fn new(log: &'log Log) -> Self {
        Qadim { log }
    }
}

impl<'log> Analyzer for Qadim<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        helpers::boss_health(self.log)
            .map(|h| h >= QADIM_CM_HEALTH)
            .unwrap_or(false)
    }

    fn outcome(&self) -> Option<Outcome> {
        Outcome::from_bool(helpers::players_exit_after_boss(self.log))
    }
}
