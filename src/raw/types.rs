use std::{self, fmt};

/// The "friend or foe" enum.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
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

/// Combat result (physical)
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
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
    /// Field is not used in this kind of event.
    None,
}

/// Combat activation
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
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

/// Combat state change
///
/// The referenced fields are of the [`CbtEvent`](struct.CbtEvent.html)
/// struct.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
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
    /// src_agent changed, cast float* p = (float*)&dst_agent, access as x/y/z (float[3])
	Position,
    /// src_agent changed, cast float* v = (float*)&dst_agent, access as x/y/z (float[3])
	Velocity,
}

/// Combat buff remove type
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
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

/// Custom skill ids
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum CbtCustomSkill {
    /// Not custom but important and unnamed.
    Resurrect = 1066,
    /// Personal healing only.
    Bandage = 1175,
    /// Will occur in is_activation==normal event.
    Dodge = 65001,
}

/// Language
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
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

/// A combat event.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
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
    pub overstack_value: u16,
    /// Skill id.
    pub skillid: u16,
    /// Agent map instance id.
    pub src_instid: u16,
    /// Agent map instance id.
    pub dst_instid: u16,
    /// Master source agent map instance id if source is a minion/pet.
    pub src_master_instid: u16,
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
}

/// An agent.
#[repr(C)]
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

impl fmt::Debug for Agent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Agent {{ addr: {}, \
             prof: {}, is_elite: {}, toughness: {}, concentration: {}, \
             healing: {}, condition: {}, name: {} }}",
            self.addr,
            self.prof,
            self.is_elite,
            self.toughness,
            self.concentration,
            self.healing,
            self.condition,
            String::from_utf8_lossy(&self.name)
        )
    }
}

/// A skill.
#[repr(C)]
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
        write!(
            f,
            "Skill {{ id: {}, name: {:?} }}",
            self.id,
            self.name_string()
        )
    }
}
