//! Boss fight analyzers for Wing 5 (Hall of Chains)
use crate::{
    analyzers::{helpers, Analyzer, Outcome},
    Encounter, EventKind, Log,
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

/// Analyzer for the River of Souls escort event in Wing 5.
#[derive(Debug, Clone, Copy)]
pub struct RiverOfSouls<'log> {
    log: &'log Log,
}

impl<'log> RiverOfSouls<'log> {
    pub fn new(log: &'log Log) -> Self {
        RiverOfSouls { log }
    }
}

impl<'log> Analyzer for RiverOfSouls<'log> {
    fn log(&self) -> &'log Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        false
    }

    fn outcome(&self) -> Option<Outcome> {
        const TRASH_IDS: &[u16] = &[0x4d97, 0x4bc7, 0x4d75, 0x4c05, 0x4bc8, 0x4cec];
        check_reward!(self.log);

        // First, let's get the Desmina NPC
        let desmina = self
            .log
            .characters()
            .find(|npc| npc.id() == Encounter::RiverOfSouls as u16)?;

        // We need to see when our friendly Desmina exited combat, because if she didn't, the event
        // failed.
        let exit_combat = self
            .log
            .events()
            .iter()
            .find(|e| matches!(e.kind(), &EventKind::ExitCombat { agent_addr } if agent_addr == desmina.addr()));
        if exit_combat.is_none() {
            return Some(Outcome::Failure);
        }

        let trash_aware = self
            .log
            .characters()
            .filter(|npc| TRASH_IDS.contains(&npc.id()))
            .map(|npc| npc.last_aware())
            .filter(|&i| i != u64::MAX)
            .max()
            .unwrap_or(0);

        let desmina_despawn = self
            .log()
            .events()
            .iter()
            .find(|e| matches!(e.kind(), &EventKind::Despawn { agent_addr } if agent_addr == desmina.addr()));

        Outcome::from_bool(
            trash_aware != 0
                && desmina_despawn.is_none()
                // Add some leeway and see if we saw Desmina after all the trash was gone
                && trash_aware + 500 <= desmina.last_aware()
                && some_player_alive(self.log),
        )
    }
}

fn some_player_alive(log: &Log) -> bool {
    let deaths_and_dcs = log
        .events()
        .iter()
        .filter_map(|e| match *e.kind() {
            EventKind::Despawn { agent_addr } => Some(agent_addr),
            EventKind::ChangeDead { agent_addr } => Some(agent_addr),
            _ => None,
        })
        .filter(|&addr| {
            log.agent_by_addr(addr)
                .map(|a| a.kind().is_player())
                .unwrap_or(false)
        })
        .count();

    deaths_and_dcs < log.players().count()
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
