//! Boss fight analyzers for Wing 4 (Bastion of the Penitent).
use crate::{
    analyzers::{helpers, Analyzer, Outcome},
    EventKind, Log,
};

pub const CAIRN_CM_BUFF: u32 = 38_098;

/// Analyzer for the first fight of Wing 4, Cairn.
///
/// The CM is detected by the presence of the buff representing the countdown before which you have
/// to use your special action skill.
#[derive(Debug, Clone, Copy)]
pub struct Cairn<'log> {
    log: &'log Log,
}

impl<'log> Cairn<'log> {
    /// Create a new [`Cairn`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
    pub fn new(log: &'log Log) -> Self {
        Cairn { log }
    }
}

impl<'log> Analyzer for Cairn<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        helpers::buff_present(self.log, CAIRN_CM_BUFF)
    }

    fn outcome(&self) -> Option<Outcome> {
        Outcome::from_bool(helpers::boss_is_dead(self.log))
    }
}

pub const MO_CM_HEALTH: u64 = 30_000_000;

/// Analyzer for the second fight of Wing 4, Mursaat Overseer.
///
/// The CM is detected by the boss's health, which is higher in the challenge mote.
#[derive(Debug, Clone, Copy)]
pub struct MursaatOverseer<'log> {
    log: &'log Log,
}

impl<'log> MursaatOverseer<'log> {
    /// Create a new [`MursaatOverseer`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
    pub fn new(log: &'log Log) -> Self {
        MursaatOverseer { log }
    }
}

impl<'log> Analyzer for MursaatOverseer<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        helpers::boss_health(self.log)
            .map(|h| h >= MO_CM_HEALTH)
            .unwrap_or(false)
    }

    fn outcome(&self) -> Option<Outcome> {
        Outcome::from_bool(helpers::boss_is_dead(self.log))
    }
}

pub const SAMAROG_CM_HEALTH: u64 = 40_000_000;

/// Analyzer for the third fight of Wing 4, Samarog.
///
/// The CM is detected by the boss's health, which is higher in the challenge mote.
#[derive(Debug, Clone, Copy)]
pub struct Samarog<'log> {
    log: &'log Log,
}

impl<'log> Samarog<'log> {
    /// Create a new [`Samarog`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
    pub fn new(log: &'log Log) -> Self {
        Samarog { log }
    }
}

impl<'log> Analyzer for Samarog<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        helpers::boss_health(self.log)
            .map(|h| h >= SAMAROG_CM_HEALTH)
            .unwrap_or(false)
    }

    fn outcome(&self) -> Option<Outcome> {
        Outcome::from_bool(helpers::boss_is_dead(self.log))
    }
}

pub const DEIMOS_CM_HEALTH: u64 = 42_000_000;

/// Analyzer for the fourth fight of Wing 4, Deimos.
///
/// The CM is detected by the boss's health, which is higher in the challenge mote.
#[derive(Debug, Clone, Copy)]
pub struct Deimos<'log> {
    log: &'log Log,
}

impl<'log> Deimos<'log> {
    /// Create a new [`Deimos`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
    pub fn new(log: &'log Log) -> Self {
        Deimos { log }
    }
}

impl<'log> Analyzer for Deimos<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        helpers::boss_health(self.log)
            .map(|h| h >= DEIMOS_CM_HEALTH)
            .unwrap_or(false)
    }

    fn outcome(&self) -> Option<Outcome> {
        // The idea for Deimos is that we first need to figure out when the 10% split happens (if
        // it even happens), then we can find the time when 10%-Deimos becomes untargetable and
        // then we can compare this time to the player exit time.

        let split_time = deimos_10_time(self.log);
        // We never got to 10%, so this is a fail.
        if split_time == 0 {
            return Some(Outcome::Failure);
        }

        let at_address = deimos_at_address(self.log);
        if at_address == 0 {
            return Some(Outcome::Failure);
        }

        let mut player_exit = 0u64;
        let mut at_exit = 0u64;
        for event in self.log.events() {
            match event.kind() {
                EventKind::ExitCombat { agent_addr }
                    if self
                        .log
                        .agent_by_addr(*agent_addr)
                        .map(|a| a.kind().is_player())
                        .unwrap_or(false)
                        && event.time() >= player_exit =>
                {
                    player_exit = event.time();
                }

                EventKind::Targetable {
                    agent_addr,
                    targetable,
                } if *agent_addr == at_address && !targetable && event.time() >= at_exit => {
                    at_exit = event.time();
                }

                _ => (),
            }
        }

        // Safety margin
        Outcome::from_bool(player_exit > at_exit + 1000)
    }
}

// Extracts the timestamp when Deimos's 10% phase started.
//
// This function may panic when passed non-Deimos logs!
fn deimos_10_time(log: &Log) -> u64 {
    let mut first_aware = 0u64;

    for event in log.events() {
        if let EventKind::Targetable { targetable, .. } = event.kind() {
            if *targetable {
                first_aware = event.time();
                println!("First aware: {}", first_aware);
            }
        }
    }

    first_aware
}

// Returns the attack target address for the 10% Deimos phase.
//
// Returns 0 when the right attack target is not found.
fn deimos_at_address(log: &Log) -> u64 {
    for event in log.events().iter().rev() {
        if let EventKind::AttackTarget {
            agent_addr,
            parent_agent_addr,
            ..
        } = event.kind()
        {
            let parent = log.agent_by_addr(*parent_agent_addr);
            if let Some(parent) = parent {
                if Some("Deimos") == parent.as_gadget().map(|g| g.name()) {
                    return *agent_addr;
                }
            }
        }
    }
    0
}
