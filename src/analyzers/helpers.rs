//! This module contains helper methods that are used in different analyzers.
use std::collections::HashMap;

use crate::{AgentKind, EventKind, Log};

/// Check if the log was rewarded, and if yes, return `Outcome::Success` early.
macro_rules! check_reward {
    ($log:expr) => {
        let log: &Log = $log;
        if log.was_rewarded() {
            return Some(crate::analyzers::Outcome::Success);
        }
    };
}

/// Returns the maximum health of the boss agent.
///
/// If the health cannot be determined, this function returns `None`.
///
/// The boss agent is determined by using [`Log::is_boss`][Log::is_boss].
pub fn boss_health(log: &Log) -> Option<u64> {
    let mut health: Option<u64> = None;
    for event in log.events() {
        if let EventKind::MaxHealthUpdate {
            agent_addr,
            max_health,
        } = *event.kind()
        {
            if log.is_boss(agent_addr) {
                health = health.map(|h| h.max(max_health)).or(Some(max_health));
            }
        }
    }
    health
}

/// Checks if any of the boss NPCs have died.
///
/// Death is determined by checking for the [`EventKind::ChangeDead`][EventKind::ChangeDead] event,
/// and whether a NPC is a boss is determined by the [`Log::is_boss`][Log::is_boss] method.
pub fn boss_is_dead(log: &Log) -> bool {
    log.events().iter().any(
        |ev| matches!(ev.kind(), EventKind::ChangeDead { agent_addr } if log.is_boss(*agent_addr)),
    )
}

/// Checks whether the players exit combat after the boss.
///
/// This is useful to determine the success state of some fights.
pub fn players_exit_after_boss(log: &Log) -> bool {
    let mut player_exit = 0u64;
    let mut boss_exit = 0u64;

    for event in log.events() {
        if let EventKind::ExitCombat { agent_addr } = event.kind() {
            let agent = if let Some(a) = log.agent_by_addr(*agent_addr) {
                a
            } else {
                continue;
            };

            match agent.kind() {
                AgentKind::Player(_) if event.time() >= player_exit => {
                    player_exit = event.time();
                }
                AgentKind::Character(_)
                    if event.time() >= boss_exit && log.is_boss(*agent_addr) =>
                {
                    boss_exit = event.time();
                }
                _ => (),
            }
        }
    }
    // Safety margin
    boss_exit != 0 && player_exit > boss_exit + 1000
}

/// Checks if the given buff is present in the log.
pub fn buff_present(log: &Log, wanted_buff_id: u32) -> bool {
    for event in log.events() {
        if let EventKind::BuffApplication { buff_id, .. } = *event.kind() {
            if buff_id == wanted_buff_id {
                return true;
            }
        }
    }
    false
}

/// Returns the (minimum) time between applications of the given buff in milliseconds.
pub fn time_between_buffs(log: &Log, wanted_buff_id: u32) -> u64 {
    let mut time_maps: HashMap<u64, Vec<u64>> = HashMap::new();
    for event in log.events() {
        if let EventKind::BuffApplication {
            destination_agent_addr,
            buff_id,
            ..
        } = event.kind()
        {
            if *buff_id == wanted_buff_id {
                time_maps
                    .entry(*destination_agent_addr)
                    .or_default()
                    .push(event.time());
            }
        }
    }
    let timestamps = if let Some(ts) = time_maps.values().max_by_key(|v| v.len()) {
        ts
    } else {
        return 0;
    };
    timestamps
        .iter()
        .zip(timestamps.iter().skip(1))
        .map(|(a, b)| b - a)
        // Arbitrary limit to filter out duplicated buff application events
        .filter(|x| *x > 50)
        .min()
        .unwrap_or(0)
}
