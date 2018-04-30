//! This module aids in the creation of actual boss statistics.
use super::*;
use std::collections::HashMap;
use std::error::Error;

pub mod boon;
pub mod gamedata;
pub mod trackers;

use self::trackers::{RunnableTracker, Tracker};

pub type StatResult<T> = Result<T, StatError>;

quick_error! {
    #[derive(Debug)]
    pub enum StatError {
        TrackerError(err: Box<Error>) {
            description("tracker error")
            display("tracker returned an error: {}", err)
            cause(&**err)
        }
    }
}

macro_rules! try_tracker {
    ($expr:expr) => {
        #[allow(unreachable_code)]
        match $expr {
            Ok(e) => e,
            Err(e) => return Err(StatError::TrackerError(e)),
        }
    };
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
    /// Average stacks of boons.
    ///
    /// This also includes conditions.
    ///
    /// For duration-based boons, the average amount of stacks is the same as
    /// the uptime.
    pub boon_averages: HashMap<u16, f64>,
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

/// Takes a bunch of trackers and runs them on the given log.
///
/// This method returns "nothing", as the statistics are saved in the trackers.
/// It's the job of the caller to extract them appropriately.
pub fn run_trackers(log: &Log, trackers: &mut [&mut RunnableTracker]) -> StatResult<()> {
    for event in log.events() {
        for mut tracker in trackers.iter_mut() {
            try_tracker!((*tracker).run_feed(event));
        }
    }
    Ok(())
}

/// Calculate the statistics for the given log.
pub fn calculate(log: &Log) -> StatResult<Statistics> {
    let mut agent_stats = HashMap::<u64, AgentStats>::new();

    let mut damage_tracker = trackers::DamageTracker::new(log);
    let mut log_start_tracker = trackers::LogStartTracker::new();
    let mut combat_time_tracker = trackers::CombatTimeTracker::new();
    let mut boon_tracker = trackers::BoonTracker::new();

    run_trackers(
        log,
        &mut [
            &mut damage_tracker,
            &mut log_start_tracker,
            &mut combat_time_tracker,
            &mut boon_tracker,
        ],
    )?;

    let log_start_time = try_tracker!(log_start_tracker.finalize());

    let combat_times = try_tracker!(combat_time_tracker.finalize());
    for (agent_addr, &(enter_time, exit_time)) in &combat_times {
        let agent = agent_stats
            .entry(*agent_addr)
            .or_insert_with(Default::default);
        if enter_time != 0 {
            agent.enter_combat = enter_time - log_start_time;
        }
        if exit_time != 0 {
            agent.exit_combat = exit_time - log_start_time;
        }
    }

    let damages = try_tracker!(damage_tracker.finalize());
    for agent in damages.keys() {
        agent_stats
            .entry(*agent)
            .or_insert_with(Default::default)
            .per_target_damage = damages[agent].clone();
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

    let boons = try_tracker!(boon_tracker.finalize());
    for (agent, boon_map) in &boons {
        let agent = agent_stats.entry(*agent).or_insert_with(Default::default);
        if agent.exit_combat < agent.enter_combat {
            continue;
        }
        let combat_time = agent.combat_time() as f64;
        if combat_time == 0. {
            continue;
        }
        agent.boon_averages = boon_map
            .iter()
            .map(|(id, area)| (*id, *area as f64 / combat_time))
            .collect();
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
