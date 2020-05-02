//! Raw evtc structs.
//!
//! This module contains the translated definitions from arcdps's C structs.
use num_derive::FromPrimitive;
use std::{self, fmt};

use std::hash::{Hash, Hasher};

/// The "friend or foe" enum.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, FromPrimitive)]
pub enum IFF {
    /// Green vs green, red vs red.
    Friend,
    /// Green vs red.
    Foe,
    /// Something very wrong happened.
    Unknown,
    /// Field is not used in this kind of event.
    None,
}

impl Default for IFF {
    fn default() -> Self {
        IFF::None
    }
}

/// Combat result (physical)
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, FromPrimitive)]
pub enum CbtResult {
    /// Good physical hit
    Normal,
    /// Physical hit was a critical hit
    Crit,
    /// Physical hit was a glance
    Glance,
    /// Physical hit was blocked (e.g. Shelter)
    Block,
    /// Physical hit was evaded (e.g. dodge)
    Evade,
    /// Physical hit interrupted something
    Interrupt,
    /// Physical hit was absorbed (e.g. invulnerability)
    Absorb,
    /// Physical hit missed
    Blind,
    /// Physical hit was the killing blow
    KillingBlow,
    /// Hit was downing hit.
    Downed,
    /// Field is not used in this kind of event.
    None,
}

impl Default for CbtResult {
    fn default() -> Self {
        CbtResult::None
    }
}

/// Combat activation
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, FromPrimitive)]
pub enum CbtActivation {
    /// Field is not used in this kind of event.
    None,
    /// Activation without quickness
    Normal,
    /// Activation with quickness
    Quickness,
    /// Cancel with reaching channel time
    CancelFire,
    /// Cancel without reaching channel time
    CancelCancel,
    /// Animation completed fully
    Reset,
}

impl Default for CbtActivation {
    fn default() -> Self {
        CbtActivation::None
    }
}

/// Combat state change
///
/// The referenced fields are of the [`CbtEvent`](struct.CbtEvent.html)
/// struct.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, FromPrimitive)]
pub enum CbtStateChange {
    /// Field is not used in this kind of event.
    None,
    /// `src_agent` entered combat.
    ///
    /// * `dst_agent` specifies the agent's subgroup.
    EnterCombat,
    /// `src_agent` left combat.
    ExitCombat,
    /// `src_agent` is now alive.
    ChangeUp,
    /// `src_agent` is now dead.
    ChangeDead,
    /// `src_agent` is now downed.
    ChangeDown,
    /// `src_agent` is now in game tracking range.
    Spawn,
    /// `src_agent` is no longer being tracked.
    Despawn,
    /// `src_agent` has reached a health marker.
    ///
    /// * `dst_agent` will be set to the new health percentage, multiplied by
    ///   10000.
    HealthUpdate,
    /// Log start.
    ///
    /// * `value` is the server unix timestamp.
    /// * `buff_dmg` is the local unix timestamp.
    /// * `src_agent` is set to `0x637261` (arcdps id)
    LogStart,
    /// Log end.
    ///
    /// * `value` is the server unix timestamp.
    /// * `buff_dmg` is the local unix timestamp.
    /// * `src_agent` is set to `0x637261` (arcdps id)
    LogEnd,
    /// `src_agent` swapped the weapon set.
    ///
    /// * `dst_agent` is the current set id (0/1 for water sets, 4/5 for land
    ///   sets)
    WeapSwap,
    /// `src_agent` has had it's maximum health changed.
    ///
    /// * `dst_agent` is the new maximum health.
    MaxHealthUpdate,
    /// `src_agent` is the agent of the recording player.
    PointOfView,
    /// `src_agent` is the text language.
    Language,
    /// `src_agent` is the game build.
    GwBuild,
    /// `src_agent` is the server shard id.
    ShardId,
    /// Represents the reward (wiggly box)
    ///
    /// * `src_agent` is self
    /// * `dst_agent` is reward id.
    /// * `value` is reward type.
    Reward,
    /// Combat event that will appear once per buff per agent on logging start (zero duration,
    /// buff==18)
    BuffInitial,
    /// `src_agent` changed position.
    ///
    /// * `dst_agent` is a 2-element float array (x, y).
    /// * `value` is a single float (z).
    Position,
    /// `src_agent` changed velocity.
    ///
    /// * `dst_agent` is a 2-element float array (x, y).
    /// * `value` is a single float (z).
    Velocity,
    /// `src_agent` changed the direction that they're facing.
    ///
    /// * `dst_agent` is a 2-element float array (x, y).
    Facing,
    /// `src_agent` changed team.
    ///
    /// * `dst_agent` is the new team id
    TeamChange,
    /// `src_agent` is an attacktarget, `dst_agent` is the parent agent (gadget type), `value` is the current targetable state
    AttackTarget,
    /// `dst_agent` is the new target-able state (0 = no, 1 = yes. default yes)
    Targetable,
    /// Information about the map that the log was done on.
    ///
    /// * `src_agent` is map id
    MapId,
    /// internal use by arcDPS, won't see anywhere
    ReplInfo,
    /// `src_agent` is agent with buff, `dst_agent` is the stackid marked active
    StackActive,
    /// `src_agent` is agent with buff, `value` is the duration to reset to (also marks inactive),
    /// `pad61-` is the stackid
    StackReset,
    /// Information about the guild.
    ///
    /// * `src_agent` is the agent
    /// * `dst_agent` through `buff_dmg` is 16 byte guild id (client form, needs minor rearrange
    ///   for api form)
    Guild,
    /// `is_flanking` = probably invuln, `is_shields` = probably invert, `is_offcycle` = category, `pad61` = stacking type, `src_master_instid` = max stacks (not in realtime)
    BuffInfo,
    /// `(float*)&time [9]`: type attr1 attr2 param1 param2 param3 trait_src trait_self, `is_flanking` = !npc, `is_shields` = !player, `is_offcycle` = break (not in realtime, one per formula)
    BuffFormula,
    /// `(float*)&time [9]`: recharge range0 range1 tooltiptime (not in realtime)
    SkillInfo,
    /// `src_agent` = action, `dst_agent` = at millisecond (not in realtime, one per timing)
    SkillTiming,
}

impl Default for CbtStateChange {
    fn default() -> Self {
        CbtStateChange::None
    }
}

/// Combat buff remove type
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, FromPrimitive)]
pub enum CbtBuffRemove {
    /// Field is not used in this kind of event.
    None,
    /// All stacks removed.
    All,
    /// Single stack removed.
    ///
    /// Disabled on server trigger, will happen for each stack on cleanse.
    Single,
    /// Autoremoved by OOC or allstack.
    ///
    /// (Ignore for strip/cleanse calc, use for in/out volume)-
    Manual,
}

impl Default for CbtBuffRemove {
    fn default() -> Self {
        CbtBuffRemove::None
    }
}

/// Custom skill ids
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, FromPrimitive)]
pub enum CbtCustomSkill {
    /// Not custom but important and unnamed.
    Resurrect = 1066,
    /// Personal healing only.
    Bandage = 1175,
    /// Will occur in is_activation==normal event.
    Dodge = 65001,
}

/// Language
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, FromPrimitive)]
pub enum Language {
    /// English.
    Eng = 0,
    /// French.
    Fre = 2,
    /// German.
    Gem = 3,
    /// Spanish.
    Spa = 4,
}

impl Default for Language {
    fn default() -> Self {
        Language::Eng
    }
}

/// A combat event.
///
/// This event combines both the old structure and the new structure. Fields not
/// present in the source are zeroed. When dealing with events, always make sure
/// to check the header.revision tag.
///
/// For conflicting data types, the bigger one is chosen (e.g. u32 over u16).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct CbtEvent {
    /// System time since Windows was started, in milliseconds.
    pub time: u64,
    /// Unique identifier of the source agent.
    pub src_agent: u64,
    /// Unique identifier of the destination agent.
    pub dst_agent: u64,
    /// Event-specific value.
    pub value: i32,
    /// Estimated buff damage. Zero on application event.
    pub buff_dmg: i32,
    /// Estimated overwritten stack duration for buff application.
    pub overstack_value: u32,
    /// Skill id.
    pub skillid: u32,
    /// Agent map instance id.
    pub src_instid: u16,
    /// Agent map instance id.
    pub dst_instid: u16,
    /// Master source agent map instance id if source is a minion/pet.
    pub src_master_instid: u16,
    /// Master destination agent map instance id if destination is a minion/pet.
    pub dst_master_instid: u16,
    pub iff: IFF,
    /// Buff application, removal or damage event.
    pub buff: u8,
    pub result: CbtResult,
    pub is_activation: CbtActivation,
    pub is_buffremove: CbtBuffRemove,
    /// Source agent health was over 90%.
    pub is_ninety: bool,
    /// Target agent health was under 90%.
    pub is_fifty: bool,
    /// Source agent was moving.
    pub is_moving: bool,
    pub is_statechange: CbtStateChange,
    /// Target agent was not facing source.
    pub is_flanking: bool,
    /// All or part damage was vs. barrier/shield.
    pub is_shields: bool,
    /// False if buff dmg happened during tick, true otherwise.
    pub is_offcycle: bool,
}

/// An agent.
#[derive(Clone)]
pub struct Agent {
    /// Agent id.
    pub addr: u64,
    /// Agent profession id.
    pub prof: u32,
    /// Agent elite specialisation.
    pub is_elite: u32,
    /// Toughnes.
    pub toughness: i16,
    /// Concentration.
    pub concentration: i16,
    /// Healing.
    pub healing: i16,
    /// Condition
    pub condition: i16,
    /// Name/Account combo field.
    pub name: [u8; 64],
}

impl Agent {
    /// Checks whether this agent is a gadget.
    ///
    /// Gadgets are entities spawned by some skills, like the "Binding Roots"
    /// spawned by Entangle.
    pub fn is_gadget(&self) -> bool {
        self.is_elite == std::u32::MAX && (self.prof & 0xffff_0000) == 0xffff_0000
    }

    /// Checks whether this agent is a character.
    ///
    /// Characters are entities like clones, pets, minions, spirits, but also
    /// minis.
    pub fn is_character(&self) -> bool {
        self.is_elite == std::u32::MAX && (self.prof & 0xffff_0000) != 0xffff_0000
    }

    /// Checks whether this agent is a player.
    pub fn is_player(&self) -> bool {
        self.is_elite != std::u32::MAX
    }
}

// We need to implement this manually, as our array is bigger than 32.

impl fmt::Debug for Agent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Agent")
            .field("addr", &self.addr)
            .field("prof", &self.prof)
            .field("is_elite", &self.is_elite)
            .field("toughness", &self.toughness)
            .field("concentration", &self.concentration)
            .field("healing", &self.healing)
            .field("condition", &self.condition)
            .field("name", &(&self.name as &[u8]))
            .finish()
    }
}

impl PartialEq for Agent {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
            && self.prof == other.prof
            && self.is_elite == other.is_elite
            && self.toughness == other.toughness
            && self.concentration == other.concentration
            && self.healing == other.healing
            && self.condition == other.condition
            && &self.name as &[u8] == &other.name as &[u8]
    }
}

impl Eq for Agent {}

impl Hash for Agent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.addr.hash(state);
        self.prof.hash(state);
        self.is_elite.hash(state);
        self.toughness.hash(state);
        self.concentration.hash(state);
        self.healing.hash(state);
        self.condition.hash(state);
        self.name.hash(state);
    }
}

impl Default for Agent {
    fn default() -> Self {
        Self {
            addr: Default::default(),
            prof: Default::default(),
            is_elite: Default::default(),
            toughness: Default::default(),
            concentration: Default::default(),
            healing: Default::default(),
            condition: Default::default(),
            name: [0; 64],
        }
    }
}

/// A skill.
#[derive(Clone)]
pub struct Skill {
    /// Skill id.
    pub id: i32,
    /// Skill name.
    pub name: [u8; 64],
}

impl Skill {
    /// Return the name of the skill as a `String`.
    ///
    /// Returns `None` if the name is not valid UTF-8.
    pub fn name_string(&self) -> Option<String> {
        let bytes = self
            .name
            .iter()
            .cloned()
            .take_while(|b| *b != 0)
            .collect::<Vec<_>>();
        String::from_utf8(bytes).ok()
    }
}

impl fmt::Debug for Skill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Skill")
            .field("id", &self.id)
            .field("name", &(&self.name as &[u8]))
            .finish()
    }
}

impl PartialEq for Skill {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && &self.name as &[u8] == &other.name as &[u8]
    }
}

impl Eq for Skill {}

impl Hash for Skill {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
    }
}

impl Default for Skill {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: [0; 64],
        }
    }
}
