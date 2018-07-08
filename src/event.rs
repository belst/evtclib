use super::raw;

use num_traits::FromPrimitive;

/// A rusty enum for all possible combat events.
///
/// This makes dealing with `CbtEvent` a bit saner (and safer).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventKind {
    // State change events
    /// The agent has entered combat.
    EnterCombat { agent_addr: u64, subgroup: u64 },
    /// The agent has left combat.
    ExitCombat { agent_addr: u64 },
    /// The agent is now alive.
    ChangeUp { agent_addr: u64 },
    /// The agent is now downed.
    ChangeDown { agent_addr: u64 },
    /// The agent is now dead.
    ChangeDead { agent_addr: u64 },
    /// The agent is now in tracking range.
    Spawn { agent_addr: u64 },
    /// The agent has left the tracking range.
    Despawn { agent_addr: u64 },
    /// The agent has reached a health treshold.
    HealthUpdate {
        agent_addr: u64,
        /// The new health, as percentage multiplied by 10000.
        health: u16,
    },
    /// The logging has started.
    LogStart {
        server_timestamp: u32,
        local_timestamp: u32,
    },
    /// The logging has finished.
    LogEnd {
        server_timestamp: u32,
        local_timestamp: u32,
    },
    /// The agent has swapped the weapon set.
    WeaponSwap { agent_addr: u64, set: WeaponSet },
    /// The given agent has its max health changed.
    MaxHealthUpdate { agent_addr: u64, max_health: u64 },
    /// The given agent is the point-of-view.
    PointOfView { agent_addr: u64 },
    /// The given language is the text language.
    Language { language: raw::Language },
    /// The log was made with the given game build.
    Build { build: u64 },
    /// The shard id of the server.
    ShardId { shard_id: u64 },
    /// A reward has been awarded.
    Reward { reward_id: u64, reward_type: i32 },

    /// A skill has been used.
    SkillUse {
        source_agent_addr: u64,
        skill_id: u16,
        activation: Activation,
    },

    /// Condition damage tick.
    ConditionTick {
        source_agent_addr: u64,
        destination_agent_addr: u64,
        condition_id: u16,
        damage: i32,
    },

    /// Condition damage tick that was negated by invulnerability.
    InvulnTick {
        source_agent_addr: u64,
        destination_agent_addr: u64,
        condition_id: u16,
    },

    /// Physical damage.
    Physical {
        source_agent_addr: u64,
        destination_agent_addr: u64,
        skill_id: u16,
        damage: i32,
        result: raw::CbtResult,
    },

    /// Buff applied.
    BuffApplication {
        source_agent_addr: u64,
        destination_agent_addr: u64,
        buff_id: u16,
        duration: i32,
        overstack: u16,
    },

    /// Buff removed.
    BuffRemove {
        source_agent_addr: u64,
        destination_agent_addr: u64,
        buff_id: u16,
        total_duration: i32,
        longest_stack: i32,
        removal: raw::CbtBuffRemove,
    },
}

/// A higher-level representation of a combat event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    /// The time when the event happened.
    ///
    /// This are the milliseconds since Windows has been started (`timeGetTime()`).
    pub time: u64,
    /// The kind of the event.
    pub kind: EventKind,
    /// Whether the agent had more than 90% of its health.
    ///
    /// This is the scholar threshold.
    pub is_ninety: bool,
    /// Whether the target health was below 50%.
    ///
    /// This is the threshold for many runes and trait damage modifiers (e.g.
    /// *Bolt to the Heart*).
    pub is_fifty: bool,
    /// Whether the source agent was moving.
    pub is_moving: bool,
    /// Whether the source agent was flanking the target.
    pub is_flanking: bool,
    /// Whether some (or all) damage was mitigated by shields.
    pub is_shields: bool,
}

impl Event {
    /// Transform a raw event to a "high-level" event.
    ///
    /// If the event is not known, or some other error occured, `None` is
    /// returned.
    ///
    /// * `raw_event` - the raw event to transform.
    pub fn from_raw(raw_event: &raw::CbtEvent) -> Option<Event> {
        use raw::CbtStateChange;
        let kind = match raw_event.is_statechange {
            // Check for state change events first.
            CbtStateChange::EnterCombat => EventKind::EnterCombat {
                agent_addr: raw_event.src_agent,
                subgroup: raw_event.dst_agent,
            },
            CbtStateChange::ExitCombat => EventKind::ExitCombat {
                agent_addr: raw_event.src_agent,
            },
            CbtStateChange::ChangeUp => EventKind::ChangeUp {
                agent_addr: raw_event.src_agent,
            },
            CbtStateChange::ChangeDead => EventKind::ChangeDead {
                agent_addr: raw_event.src_agent,
            },
            CbtStateChange::ChangeDown => EventKind::ChangeDown {
                agent_addr: raw_event.src_agent,
            },
            CbtStateChange::Spawn => EventKind::Spawn {
                agent_addr: raw_event.src_agent,
            },
            CbtStateChange::Despawn => EventKind::Despawn {
                agent_addr: raw_event.src_agent,
            },
            CbtStateChange::HealthUpdate => EventKind::HealthUpdate {
                agent_addr: raw_event.src_agent,
                health: raw_event.dst_agent as u16,
            },
            CbtStateChange::LogStart => EventKind::LogStart {
                server_timestamp: raw_event.value as u32,
                local_timestamp: raw_event.buff_dmg as u32,
            },
            CbtStateChange::LogEnd => EventKind::LogEnd {
                server_timestamp: raw_event.value as u32,
                local_timestamp: raw_event.buff_dmg as u32,
            },
            CbtStateChange::WeapSwap => EventKind::WeaponSwap {
                agent_addr: raw_event.src_agent,
                set: WeaponSet::from_u64(raw_event.dst_agent),
            },
            CbtStateChange::MaxHealthUpdate => EventKind::MaxHealthUpdate {
                agent_addr: raw_event.src_agent,
                max_health: raw_event.dst_agent,
            },
            CbtStateChange::PointOfView => EventKind::PointOfView {
                agent_addr: raw_event.src_agent,
            },
            CbtStateChange::Language => EventKind::Language {
                language: raw::Language::from_u64(raw_event.src_agent).unwrap(),
            },
            CbtStateChange::GwBuild => EventKind::Build {
                build: raw_event.src_agent,
            },
            CbtStateChange::ShardId => EventKind::ShardId {
                shard_id: raw_event.src_agent,
            },
            CbtStateChange::Reward => EventKind::Reward {
                reward_id: raw_event.dst_agent,
                reward_type: raw_event.value,
            },
            // XXX: implement proper handling of those events!
            CbtStateChange::BuffInitial | CbtStateChange::Position | CbtStateChange::Velocity => return None,

            CbtStateChange::None => if let Some(kind) = check_activation(raw_event) {
                kind
            } else {
                return None;
            },
        };
        Some(Event {
            time: raw_event.time,
            kind,
            is_ninety: raw_event.is_ninety,
            is_fifty: raw_event.is_fifty,
            is_moving: raw_event.is_moving,
            is_flanking: raw_event.is_flanking,
            is_shields: raw_event.is_shields,
        })
    }
}

fn check_activation(raw_event: &raw::CbtEvent) -> Option<EventKind> {
    use raw::CbtActivation;
    match raw_event.is_activation {
        CbtActivation::None => check_buffremove(raw_event),

        activation => Some(EventKind::SkillUse {
            source_agent_addr: raw_event.src_agent,
            skill_id: raw_event.skillid,
            activation: match activation {
                CbtActivation::Quickness => Activation::Quickness(raw_event.value),
                CbtActivation::Normal => Activation::Normal(raw_event.value),
                CbtActivation::CancelFire => Activation::CancelFire(raw_event.value),
                CbtActivation::CancelCancel => Activation::CancelCancel(raw_event.value),
                CbtActivation::Reset => Activation::Reset,
                // Already checked and handled above
                CbtActivation::None => unreachable!(),
            },
        }),
    }
}

fn check_buffremove(raw_event: &raw::CbtEvent) -> Option<EventKind> {
    use raw::CbtBuffRemove;
    match raw_event.is_buffremove {
        CbtBuffRemove::None => check_damage(raw_event),

        removal => Some(EventKind::BuffRemove {
            source_agent_addr: raw_event.src_agent,
            destination_agent_addr: raw_event.dst_agent,
            buff_id: raw_event.skillid,
            total_duration: raw_event.value,
            longest_stack: raw_event.buff_dmg,
            removal,
        }),
    }
}

fn check_damage(raw_event: &raw::CbtEvent) -> Option<EventKind> {
    if raw_event.buff == 0 && raw_event.iff == raw::IFF::Foe && raw_event.dst_agent != 0 {
        Some(EventKind::Physical {
            source_agent_addr: raw_event.src_agent,
            destination_agent_addr: raw_event.dst_agent,
            skill_id: raw_event.skillid,
            damage: raw_event.value,
            result: raw_event.result,
        })
    } else if raw_event.buff == 1
        && raw_event.buff_dmg != 0
        && raw_event.dst_agent != 0
        && raw_event.value == 0
    {
        Some(EventKind::ConditionTick {
            source_agent_addr: raw_event.src_agent,
            destination_agent_addr: raw_event.dst_agent,
            condition_id: raw_event.skillid,
            damage: raw_event.buff_dmg,
        })
    } else if raw_event.buff == 1 && raw_event.buff_dmg == 0 && raw_event.value != 0 {
        Some(EventKind::BuffApplication {
            source_agent_addr: raw_event.src_agent,
            destination_agent_addr: raw_event.dst_agent,
            buff_id: raw_event.skillid,
            duration: raw_event.value,
            overstack: raw_event.overstack_value,
        })
    } else if raw_event.buff == 1 && raw_event.buff_dmg == 0 && raw_event.value == 0 {
        Some(EventKind::InvulnTick {
            source_agent_addr: raw_event.src_agent,
            destination_agent_addr: raw_event.dst_agent,
            condition_id: raw_event.skillid,
        })
    } else {
        None
    }
}

/// The different weapon-sets in game.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeaponSet {
    /// First water weapon set.
    Water0,
    /// Second water weapon set.
    Water1,
    /// First land set.
    Land0,
    /// Second land set.
    Land1,
    /// An unknown weapon set.
    ///
    /// This can be caused bundles or anything else that uses the "weapon swap"
    /// event but is not a normal weapon set.
    Unknown(u8),
}

impl WeaponSet {
    /// Parse a given integer into the correct enum value.
    fn from_u64(value: u64) -> WeaponSet {
        match value {
            // magic constants from arcdps README
            0 => WeaponSet::Water0,
            1 => WeaponSet::Water1,
            4 => WeaponSet::Land0,
            5 => WeaponSet::Land1,
            _ => WeaponSet::Unknown(value as u8),
        }
    }
}

/// The different types to activate a skill.
///
/// The parameter is the animation time in milliseconds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Activation {
    /// The skill was activated with quickness.
    Quickness(i32),
    /// The skill was activated normally.
    Normal(i32),
    /// The skill was cancelled with reaching the channel time.
    CancelFire(i32),
    /// The skill was cancelled without reaching the channel time.
    CancelCancel(i32),
    /// The channel was completed successfully.
    Reset,
}
