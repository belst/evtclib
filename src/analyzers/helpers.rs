//! This module contains helper methods that are used in different analyzers.
use std::collections::HashMap;

use crate::{EventKind, Log};

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
