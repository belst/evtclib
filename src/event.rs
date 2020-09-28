//! Event definitions.
//!
//! This module contains the different types of events in their high-level form.
use super::raw;

use std::convert::TryFrom;
use std::io;

use byteorder::{BigEndian, WriteBytesExt, LE};
use getset::{CopyGetters, Getters};
use num_traits::FromPrimitive;
use thiserror::Error;

/// Any error that can occur when trying to convert a raw [`CbtEvent`][raw::CbtEvent] to a
/// [`Event`][Event].
#[derive(Clone, Debug, Error)]
pub enum FromRawEventError {
    #[error("event contains an unknown state change: {0:?}")]
    UnknownStateChange(raw::CbtStateChange),
    #[error("event contains an unknown damage event")]
    UnknownDamageEvent,
    #[error("the event contains invalid text")]
    InvalidText,
}

/// A rusty enum for all possible combat events.
///
/// This makes dealing with [`CbtEvent`][raw::CbtEvent] a bit saner (and safer).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
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
        skill_id: u32,
        activation: Activation,
    },

    /// Condition damage tick.
    ConditionTick {
        source_agent_addr: u64,
        destination_agent_addr: u64,
        condition_id: u32,
        damage: i32,
    },

    /// Condition damage tick that was negated by invulnerability.
    InvulnTick {
        source_agent_addr: u64,
        destination_agent_addr: u64,
        condition_id: u32,
    },

    /// Physical damage.
    Physical {
        source_agent_addr: u64,
        destination_agent_addr: u64,
        skill_id: u32,
        damage: i32,
        result: raw::CbtResult,
    },

    /// Buff applied.
    BuffApplication {
        source_agent_addr: u64,
        destination_agent_addr: u64,
        buff_id: u32,
        duration: i32,
        overstack: u32,
    },

    /// Buff removed.
    BuffRemove {
        source_agent_addr: u64,
        destination_agent_addr: u64,
        buff_id: u32,
        total_duration: i32,
        longest_stack: i32,
        removal: raw::CbtBuffRemove,
    },

    /// Position of the agent has changed.
    Position {
        agent_addr: u64,
        x: f32,
        y: f32,
        z: f32,
    },

    /// Velocity of the agent has changed.
    Velocity {
        agent_addr: u64,
        x: f32,
        y: f32,
        z: f32,
    },

    /// The agent is facing in the given direction.
    Facing { agent_addr: u64, x: f32, y: f32 },

    /// The given agent changed their team.
    TeamChange { agent_addr: u64, team_id: u64 },

    /// Establishes an "attack target" relationship between two agents.
    ///
    /// Attack targets are somewhat not really documented, but the gist seems to be that some
    /// agents act as an "attack target" for other agents. This is mainly for the purpose of some
    /// status update events, such as [`Targetable`][EventKind::Targetable] or
    /// [`MaxHealthUpdate`][EventKind::MaxHealthUpdate].
    ///
    /// Damage events seem to not have attack targets as their target, so if your only goal is to
    /// calculate the damage dealt, you should be fine ignoring attack targets.
    ///
    /// Further sources:
    /// * [AttackTargetEvent.cs](https://github.com/baaron4/GW2-Elite-Insights-Parser/blob/8a0ccd381be8680d53a5840c569d0b8a111cea41/GW2EIParser/Parser/ParsedData/CombatEvents/StatusEvents/AttackTargetEvent.cs)
    /// * [Deimos.cs](https://github.com/baaron4/GW2-Elite-Insights-Parser/blob/8a0ccd381be8680d53a5840c569d0b8a111cea41/GW2EIParser/FightLogic/Raids/W4/Deimos.cs)
    /// * [ConjuredAmalgamate.cs](https://github.com/baaron4/GW2-Elite-Insights-Parser/blob/8a0ccd381be8680d53a5840c569d0b8a111cea41/GW2EIParser/FightLogic/Raids/W6/ConjuredAmalgamate.cs)
    /// * [Adina.cs](https://github.com/baaron4/GW2-Elite-Insights-Parser/blob/8a0ccd381be8680d53a5840c569d0b8a111cea41/GW2EIParser/FightLogic/Raids/W7/Adina.cs)
    AttackTarget {
        agent_addr: u64,
        parent_agent_addr: u64,
        targetable: bool,
    },

    /// Updates the targetable state for the given agent.
    Targetable { agent_addr: u64, targetable: bool },

    /// Information about the map id.
    MapId { map_id: u64 },

    /// Guild identification
    Guild {
        source_agent_addr: u64,
        raw_bytes: [u8; 16],
        api_guild_id: Option<String>,
    },

    /// An error was reported by arcdps.
    Error { text: String },

    /// The given agent has the tag.
    ///
    /// Note that the tag id is volatile and depends on the game build. Do not rely on the actual
    /// value of this!
    Tag { agent_addr: u64, tag_id: i32 },
}

/// A higher-level representation of a combat event.
///
/// Events can be many things, from damage events to general status messages (e.g. there's
/// [`EventKind::MapId`][EventKind::MapId] to give information about the current map). The main way
/// to use events is to match on the [`EventKind`][EventKind] stored in [`.kind`][Event::kind] and
/// then decide how to proceed. Note that all [`Event`][Event]s have certain fields that are always
/// present, but they might not always be useful or contain sensible information. This is just an
/// artifact of how arcdps saves the events.
///
/// The main way to deal with events is to iterate/use the [`.events()`][super::Log::events]
/// provided by a parsed [`Log`][super::Log]. However, if you end up working with raw events
/// ([`CbtEvent`][raw::CbtEvent]), then you can convert them to a "high-level" event using the
/// standard [`TryFrom`][TryFrom]/[`TryInto`][std::convert::TryInto] mechanisms:
///
/// ```no_run
/// # use evtclib::{raw, Event};
/// use std::convert::TryInto;
/// let raw_event: raw::CbtEvent = panic!();
/// let event: Event = raw_event.try_into().unwrap();
/// ```
///
/// Note that if you plan on re-using the raw event afterwards, you should use the implementation
/// that works on a reference instead: `Event::try_from(&raw_event)`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, CopyGetters, Getters)]
pub struct Event {
    /// The time when the event happened.
    ///
    /// This are the milliseconds since Windows has been started (`timeGetTime()`).
    #[get_copy = "pub"]
    time: u64,
    /// The kind of the event.
    #[get = "pub"]
    kind: EventKind,
    /// Whether the agent had more than 90% of its health.
    ///
    /// This is the scholar threshold.
    #[get_copy = "pub"]
    is_ninety: bool,
    /// Whether the target health was below 50%.
    ///
    /// This is the threshold for many runes and trait damage modifiers (e.g.
    /// *Bolt to the Heart*).
    #[get_copy = "pub"]
    is_fifty: bool,
    /// Whether the source agent was moving.
    #[get_copy = "pub"]
    is_moving: bool,
    /// Whether the source agent was flanking the target.
    #[get_copy = "pub"]
    is_flanking: bool,
    /// Whether some (or all) damage was mitigated by shields.
    #[get_copy = "pub"]
    is_shields: bool,
}

impl TryFrom<raw::CbtEvent> for Event {
    type Error = FromRawEventError;
    /// Convenience method to avoid manual borrowing.
    ///
    /// Note that this conversion will consume the event, so if you plan on re-using it, use the
    /// `TryFrom<&raw::CbtEvent>` implementation that works with a reference.
    fn try_from(raw_event: raw::CbtEvent) -> Result<Self, Self::Error> {
        Event::try_from(&raw_event)
    }
}

impl TryFrom<&raw::CbtEvent> for Event {
    type Error = FromRawEventError;

    /// Transform a raw event to a "high-level" event.
    ///
    /// If the event is not known, or some other error occured, `None` is
    /// returned.
    ///
    /// * `raw_event` - the raw event to transform.
    fn try_from(raw_event: &raw::CbtEvent) -> Result<Self, Self::Error> {
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
            CbtStateChange::Guild => EventKind::Guild {
                source_agent_addr: raw_event.src_agent,
                raw_bytes: get_guild_id_bytes(raw_event),
                api_guild_id: get_api_guild_string(&get_guild_id_bytes(raw_event)),
            },
            CbtStateChange::Position => EventKind::Position {
                agent_addr: raw_event.src_agent,
                x: f32::from_bits((raw_event.dst_agent >> 32) as u32),
                y: f32::from_bits((raw_event.dst_agent & 0xffff_ffff) as u32),
                z: f32::from_bits(raw_event.value as u32),
            },
            CbtStateChange::Velocity => EventKind::Velocity {
                agent_addr: raw_event.src_agent,
                x: f32::from_bits((raw_event.dst_agent >> 32) as u32),
                y: f32::from_bits((raw_event.dst_agent & 0xffff_ffff) as u32),
                z: f32::from_bits(raw_event.value as u32),
            },
            CbtStateChange::Facing => EventKind::Facing {
                agent_addr: raw_event.src_agent,
                x: f32::from_bits((raw_event.dst_agent >> 32) as u32),
                y: f32::from_bits((raw_event.dst_agent & 0xffff_ffff) as u32),
            },
            CbtStateChange::MapId => EventKind::MapId {
                map_id: raw_event.src_agent,
            },
            CbtStateChange::TeamChange => EventKind::TeamChange {
                agent_addr: raw_event.src_agent,
                team_id: raw_event.dst_agent,
            },
            CbtStateChange::AttackTarget => EventKind::AttackTarget {
                agent_addr: raw_event.src_agent,
                parent_agent_addr: raw_event.dst_agent,
                targetable: raw_event.value != 0,
            },
            CbtStateChange::Targetable => EventKind::Targetable {
                agent_addr: raw_event.src_agent,
                targetable: raw_event.dst_agent != 0,
            },
            CbtStateChange::Error => {
                let data = get_error_bytes(&raw_event);
                EventKind::Error {
                    text: raw::cstr_up_to_nul(&data)
                        .ok_or(FromRawEventError::InvalidText)?
                        .to_string_lossy()
                        .into_owned(),
                }
            }
            CbtStateChange::Tag => EventKind::Tag {
                agent_addr: raw_event.src_agent,
                tag_id: raw_event.value,
            },
            // XXX: implement proper handling of those events!
            CbtStateChange::BuffInitial
            | CbtStateChange::ReplInfo
            | CbtStateChange::StackActive
            | CbtStateChange::StackReset
            | CbtStateChange::BuffInfo
            | CbtStateChange::BuffFormula
            | CbtStateChange::SkillInfo
            | CbtStateChange::SkillTiming
            | CbtStateChange::BreakbarState
            | CbtStateChange::BreakbarPercent => {
                return Err(FromRawEventError::UnknownStateChange(
                    raw_event.is_statechange,
                ))
            }

            CbtStateChange::None => check_activation(raw_event)?,
        };
        Ok(Event {
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

fn check_activation(raw_event: &raw::CbtEvent) -> Result<EventKind, FromRawEventError> {
    use raw::CbtActivation;
    match raw_event.is_activation {
        CbtActivation::None => check_buffremove(raw_event),

        activation => Ok(EventKind::SkillUse {
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

fn check_buffremove(raw_event: &raw::CbtEvent) -> Result<EventKind, FromRawEventError> {
    use raw::CbtBuffRemove;
    match raw_event.is_buffremove {
        CbtBuffRemove::None => check_damage(raw_event),

        removal => Ok(EventKind::BuffRemove {
            source_agent_addr: raw_event.src_agent,
            destination_agent_addr: raw_event.dst_agent,
            buff_id: raw_event.skillid,
            total_duration: raw_event.value,
            longest_stack: raw_event.buff_dmg,
            removal,
        }),
    }
}

fn check_damage(raw_event: &raw::CbtEvent) -> Result<EventKind, FromRawEventError> {
    if raw_event.buff == 0 && raw_event.iff == raw::IFF::Foe && raw_event.dst_agent != 0 {
        Ok(EventKind::Physical {
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
        Ok(EventKind::ConditionTick {
            source_agent_addr: raw_event.src_agent,
            destination_agent_addr: raw_event.dst_agent,
            condition_id: raw_event.skillid,
            damage: raw_event.buff_dmg,
        })
    } else if raw_event.buff == 1 && raw_event.buff_dmg == 0 && raw_event.value != 0 {
        Ok(EventKind::BuffApplication {
            source_agent_addr: raw_event.src_agent,
            destination_agent_addr: raw_event.dst_agent,
            buff_id: raw_event.skillid,
            duration: raw_event.value,
            overstack: raw_event.overstack_value,
        })
    } else if raw_event.buff == 1 && raw_event.buff_dmg == 0 && raw_event.value == 0 {
        Ok(EventKind::InvulnTick {
            source_agent_addr: raw_event.src_agent,
            destination_agent_addr: raw_event.dst_agent,
            condition_id: raw_event.skillid,
        })
    } else {
        Err(FromRawEventError::UnknownDamageEvent)
    }
}

fn get_guild_id_bytes(raw_event: &raw::CbtEvent) -> [u8; 16] {
    let mut result = [0; 16];
    let mut cursor = io::Cursor::new(&mut result as &mut [u8]);
    cursor.write_u64::<BigEndian>(raw_event.dst_agent).unwrap();
    cursor.write_i32::<BigEndian>(raw_event.value).unwrap();
    cursor.write_i32::<BigEndian>(raw_event.buff_dmg).unwrap();
    result
}

fn get_api_guild_string(bytes: &[u8; 16]) -> Option<String> {
    if bytes == &[0; 16] {
        return None;
    }
    static PACKS: &[&[usize]] = &[
        &[4, 5, 6, 7],
        &[2, 3],
        &[0, 1],
        &[11, 10],
        &[9, 8, 15, 14, 13, 12],
    ];
    let result = PACKS
        .iter()
        .map(|p| p.iter().map(|i| format!("{:02X}", bytes[*i])).collect())
        .collect::<Vec<String>>()
        .join("-");
    Some(result)
}

fn get_error_bytes(raw_event: &raw::CbtEvent) -> [u8; 32] {
    let mut result = [0; 32];
    let mut cursor = io::Cursor::new(&mut result as &mut [u8]);
    cursor.write_u64::<LE>(raw_event.time).unwrap();
    cursor.write_u64::<LE>(raw_event.src_agent).unwrap();
    cursor.write_u64::<LE>(raw_event.dst_agent).unwrap();
    cursor.write_i32::<LE>(raw_event.value).unwrap();
    cursor.write_i32::<LE>(raw_event.buff_dmg).unwrap();
    result
}

/// The different weapon-sets in game.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
