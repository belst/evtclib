//! This module aids in the creation of actual boss statistics.
use super::*;
use std::collections::HashMap;

pub type StatResult<T> = Result<T, StatError>;

quick_error! {
    #[derive(Clone, Debug)]
    pub enum StatError {
    }
}

/// A struct containing the calculated statistics for the log.
#[derive(Clone, Debug)]
pub struct Statistics {
    /// A map mapping agent addresses to their stats.
    pub agent_stats: HashMap<u64, AgentStats>,
}

/// A struct describing the agent statistics.
#[derive(Clone, Debug, Default)]
pub struct AgentStats {
    /// Damage done per target during the fight.
    ///
    /// Maps from target address to the damage done to this target.
    pub per_target_damage: HashMap<u64, DamageStats>,
    /// Total damage dealt during the fight.
    pub total_damage: DamageStats,
    /// Damage directed to the boss.
    pub boss_damage: DamageStats,
    /// Time when the agent has entered combat (millseconds since log start).
    pub enter_combat: u64,
    /// Time when the agent has left combat (millseconds since log start).
    pub exit_combat: u64,
}

impl AgentStats {
    /// Returns the combat time of this agent in milliseconds.
    pub fn combat_time(&self) -> u64 {
        self.exit_combat - self.enter_combat
    }
}

/// Damage statistics for a given target.
#[derive(Debug, Clone, Copy, Default)]
pub struct DamageStats {
    /// The total damage of the player, including all minions/pets/...
    pub total_damage: u64,
    /// The condition damage that the player dealt.
    pub condition_damage: u64,
    /// The power damage that the player dealt.
    pub power_damage: u64,
    /// The damage that was done by minions/pets/...
    pub add_damage: u64,
}

// A support macro to introduce a new block.
//
// Doesn't really require a macro, but it's nicer to look at
//   with! { foo = bar }
// rather than
//   { let foo = bar; ... }
macro_rules! with {
    ($name:ident = $expr:expr => $bl:block) => {{
        let $name = $expr;
        $bl
    }};
}

/// Calculate the statistics for the given log.
pub fn calculate(log: &Log) -> StatResult<Statistics> {
    use super::EventKind::*;

    let mut agent_stats = HashMap::<u64, AgentStats>::new();
    let mut log_start_time = 0;

    fn get_stats(map: &mut HashMap<u64, AgentStats>, source: u64, target: u64) -> &mut DamageStats {
        map.entry(source)
            .or_insert_with(Default::default)
            .per_target_damage
            .entry(target)
            .or_insert_with(Default::default)
    }

    for event in log.events() {
        match event.kind {
            LogStart { .. } => {
                log_start_time = event.time;
            }

            EnterCombat { agent_addr, .. } => {
                agent_stats
                    .entry(agent_addr)
                    .or_insert_with(Default::default)
                    .enter_combat = event.time - log_start_time;
            }

            ExitCombat { agent_addr } => {
                agent_stats
                    .entry(agent_addr)
                    .or_insert_with(Default::default)
                    .exit_combat = event.time - log_start_time;
            }

            Physical {
                source_agent_addr,
                destination_agent_addr,
                damage,
                ..
            } => {
                with! { stats = get_stats(&mut agent_stats, source_agent_addr, destination_agent_addr) => {
                    stats.total_damage += damage as u64;
                    stats.power_damage += damage as u64;
                }}

                if let Some(master) = log.master_agent(source_agent_addr) {
                    let master_stats =
                        get_stats(&mut agent_stats, master.addr, destination_agent_addr);
                    master_stats.total_damage += damage as u64;
                    master_stats.add_damage += damage as u64;
                }
            }

            ConditionTick {
                source_agent_addr,
                destination_agent_addr,
                damage,
                ..
            } => {
                with! { stats = get_stats(&mut agent_stats, source_agent_addr, destination_agent_addr) => {
                    stats.total_damage += damage as u64;
                    stats.condition_damage += damage as u64;
                }}

                if let Some(master) = log.master_agent(source_agent_addr) {
                    let master_stats =
                        get_stats(&mut agent_stats, master.addr, destination_agent_addr);
                    master_stats.total_damage += damage as u64;
                    master_stats.add_damage += damage as u64;
                }
            }

            _ => (),
        }
    }

    let boss = log.boss();

    for agent_stat in agent_stats.values_mut() {
        tally_damage(agent_stat);
        agent_stat.boss_damage = agent_stat
            .per_target_damage
            .get(&boss.addr)
            .cloned()
            .unwrap_or_else(Default::default);
    }

    Ok(Statistics { agent_stats })
}

/// Takes the per target damage stats and tallies them up into the total damage
/// stats.
fn tally_damage(stats: &mut AgentStats) {
    for damage in stats.per_target_damage.values() {
        stats.total_damage.total_damage += damage.total_damage;
        stats.total_damage.power_damage += damage.power_damage;
        stats.total_damage.condition_damage += damage.condition_damage;
        stats.total_damage.add_damage += damage.add_damage;
    }
}
