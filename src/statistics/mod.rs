//! This module aids in the creation of actual boss statistics.
use super::*;
use std::collections::HashMap;
use std::error::Error;

pub mod boon;
pub mod damage;
pub mod gamedata;
pub mod math;
pub mod trackers;

use self::boon::BoonLog;
use self::damage::DamageLog;
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
    /// The complete damage log.
    pub damage_log: DamageLog,
    /// A map mapping agent addresses to their stats.
    pub agent_stats: HashMap<u64, AgentStats>,
}

/// A struct describing the agent statistics.
#[derive(Clone, Debug, Default)]
pub struct AgentStats {
    /// Average stacks of boons.
    ///
    /// This also includes conditions.
    ///
    /// For duration-based boons, the average amount of stacks is the same as
    /// the uptime.
    pub boon_log: BoonLog,
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
        // XXX: This used to be enter_time - log_start_time, as it makes more
        // sense to have the time relative to the log start instead of the
        // Windows boot time. However, this also means that we need to modify
        // all event times before we do any tracking, as many trackers rely on
        // event.time to track information related to time.
        if enter_time != 0 {
            agent.enter_combat = enter_time;
        } else {
            agent.enter_combat = log_start_time;
        }
        if exit_time != 0 {
            agent.exit_combat = exit_time;
        }
    }

    let boon_logs = try_tracker!(boon_tracker.finalize());
    for (agent_addr, boon_log) in boon_logs {
        let agent = agent_stats
            .entry(agent_addr)
            .or_insert_with(Default::default);
        agent.boon_log = boon_log;
    }

    let damage_log = try_tracker!(damage_tracker.finalize());

    Ok(Statistics {
        damage_log,
        agent_stats,
    })
}
