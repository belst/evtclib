//! `evtclib` is a crate aiming to provide utility functions to parse and work
//! with `.evtc` reports generated by arcdps.
//!
//! # About evtc Files
//!
//! evtc files are files generated by the (inofficial) arcdps addon to Guild Wars 2. They contain
//! metadata about a fight in the game, such as the boss's name (if it was a raid or fractal boss),
//! the participants, and a stripped-down log of the complete fight.
//!
//! There are other programs (such as
//! [GW2-Elite-Insights-Parser](https://github.com/baaron4/GW2-Elite-Insights-Parser/)) and
//! websites (such as [dps.report](https://dps.report)) which allow you to generate reports from
//! evtc files.
//!
//! A common way to store and distribute evtc files is to zip them to either a `.evtc.zip` (old
//! way) or a `.zevtc` (new way). evtclib uses [`zip`](https://crates.io/crates/zip) to read them,
//! prodiving the [`raw::parse_zip`][raw::parse_zip] convenience function.
//!
//! # Crate Structure
//!
//! The crate consists of two main parts: The [`raw`][raw] parser, which is used to read structured
//! data from binary input streams, and the higher-level abstrations provided in the root and
//! [`event`][event] submodules.
//!
//! Additionally, there are some defintions (such as IDs for various game items) in the
//! [`gamedata`][gamedata] module.
//!
//! The main structs that you should be dealing with are the [`Log`][Log] and its components, such
//! as [`Event`][Event] and [`Agent`][Agent].
//!
//! # Workflow
//!
//! Currently, there is no convenience function to turn a file into a [`Log`][Log] directly, so you
//! have to use the [`raw`][raw] submodule to obtain a low-level [`Evtc`][raw::Evtc], and then
//! convert it to the high-level [`Log`][Log].
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use std::fs::File;
//! // Open a file for processing
//! let mut file = File::open("my_log.evtc")?;
//! // Parse the raw content of the file
//! let raw_log = evtclib::raw::parse_file(&mut file)?;
//! // Process the file to do the nitty-gritty low-level stuff done
//! let log = evtclib::process(&raw_log)?;
//! // Do work on the log
//! for player in log.players() {
//!     println!("Player {} participated!", player.account_name());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! Make sure to take a look at the note on "Buffering" in the [parser
//! module](raw/parser/index.html#buffering) in order to increase the speed of your application.

use std::convert::TryFrom;
use std::marker::PhantomData;

use getset::{CopyGetters, Getters};
use num_traits::FromPrimitive;
use thiserror::Error;

pub mod raw;

pub mod event;
pub use event::{Event, EventKind};

pub mod gamedata;
pub use gamedata::{Boss, EliteSpec, Profession};

/// Any error that can occur during the processing of evtc files.
#[derive(Error, Debug)]
pub enum EvtcError {
    /// Generic error for invalid data in the evtc file.
    #[error("invalid data has been provided")]
    InvalidData,
    /// The profession id is not known.
    ///
    /// The field contains the unknown profession id.
    #[error("invalid profession id: {0}")]
    InvalidProfession(u32),
    /// The elite specialization id is not known.
    ///
    /// The field contains the unknown elite specialization id.
    #[error("invalid elite specialization id: {0}")]
    InvalidEliteSpec(u32),
    /// The file contains invalid utf-8.
    #[error("utf8 decoding error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
}

/// Player-specific agent data.
///
/// Player agents are characters controlled by a player and as such, they contain data about the
/// account and character used (name, profession), as well as the squad composition.
///
/// Note that a `Player` is only the player character itself. Any additional entities that are
/// spawned by the player (clones, illusions, banners, ...) are either a [`Character`][Character]
/// or a [`Gadget`][Gadget].
#[derive(Debug, Clone, Hash, PartialEq, Eq, CopyGetters)]
pub struct Player {
    /// The player's profession.
    #[get_copy = "pub"]
    profession: Profession,

    /// The player's elite specialization, if any is equipped.
    #[get_copy = "pub"]
    elite: Option<EliteSpec>,

    character_name: String,

    account_name: String,

    /// The subgroup the player was in.
    #[get_copy = "pub"]
    subgroup: u8,
}

impl Player {
    /// The player's character name.
    pub fn character_name(&self) -> &str {
        &self.character_name
    }

    /// The player's account name.
    ///
    /// This includes the leading colon and the 4-digit denominator.
    pub fn account_name(&self) -> &str {
        &self.account_name
    }
}

/// Gadget-specific agent data.
///
/// Gadgets are entities that are spawned by certain skills. They are mostly inanimate objects that
/// only exist to achieve a certain skill effect.
///
/// Examples of this include the [banners](https://wiki.guildwars2.com/wiki/Banner) spawned by
/// Warriors, but also skill effects like the roots created by
/// [Entangle](https://wiki.guildwars2.com/wiki/Entangle) or the other objects in the arena.
#[derive(Debug, Clone, Hash, PartialEq, Eq, CopyGetters)]
pub struct Gadget {
    /// The id of the gadget.
    ///
    /// Note that gadgets do not have true ids and the id is generated "through a combination of
    /// gadget parameters".
    #[get_copy = "pub"]
    id: u16,
    name: String,
}

impl Gadget {
    /// The name of the gadget.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Character-specific agent data.
///
/// Characters are NPCs such as the bosses themselves, additional mobs that they spawn, but also
/// friendly characters like Mesmer's clones and illusions, Necromancer minions, and so on.
#[derive(Debug, Clone, Hash, PartialEq, Eq, CopyGetters)]
pub struct Character {
    /// The id of the character.
    #[get_copy = "pub"]
    id: u16,
    name: String,
}

impl Character {
    /// The name of the character.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// The type of an agent.
///
/// arcdps differentiates between three types of agents: [`Player`][Player],
/// [`Character`][Character] and [`Gadget`][Gadget]. This enum unifies handling between them by
/// allowing you to pattern match or use one of the accessor methods.
///
/// The main way to obtain a `AgentKind` is by using the [`.kind()`][Agent::kind] method on an
/// [`Agent`][Agent]. In cases where you already have a [`raw::Agent`][raw::Agent] available, you
/// can also use the [`TryFrom`][TryFrom]/[`TryInto`][std::convert::TryInto] traits to convert a
/// `raw::Agent` or `&raw::Agent` to a `AgentKind`:
///
/// ```no_run
/// # use evtclib::{AgentKind, raw};
/// use std::convert::TryInto;
/// // Get a raw::Agent from somewhere
/// let raw_agent: raw::Agent = panic!();
/// // Convert it
/// let agent: AgentKind = raw_agent.try_into().unwrap();
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AgentKind {
    /// The agent is a player.
    ///
    /// The player-specific data is in the included [`Player`][Player] struct.
    Player(Player),
    /// The agent is a gadget.
    ///
    /// The gadget-specific data is in the included [`Gadget`][Gadget] struct.
    Gadget(Gadget),
    /// The agent is a character.
    ///
    /// The character-specific data is in the included [`Character`][Character] struct.
    Character(Character),
}

impl AgentKind {
    fn from_raw_character(raw_agent: &raw::Agent) -> Result<Character, EvtcError> {
        assert!(raw_agent.is_character());
        let name = raw::cstr_up_to_nul(&raw_agent.name).ok_or(EvtcError::InvalidData)?;
        Ok(Character {
            id: raw_agent.prof as u16,
            name: name.to_str()?.to_owned(),
        })
    }

    fn from_raw_gadget(raw_agent: &raw::Agent) -> Result<Gadget, EvtcError> {
        assert!(raw_agent.is_gadget());
        let name = raw::cstr_up_to_nul(&raw_agent.name).ok_or(EvtcError::InvalidData)?;
        Ok(Gadget {
            id: raw_agent.prof as u16,
            name: name.to_str()?.to_owned(),
        })
    }

    fn from_raw_player(raw_agent: &raw::Agent) -> Result<Player, EvtcError> {
        assert!(raw_agent.is_player());
        let character_name = raw::cstr_up_to_nul(&raw_agent.name)
            .ok_or(EvtcError::InvalidData)?
            .to_str()?;
        let account_name = raw::cstr_up_to_nul(&raw_agent.name[character_name.len() + 1..])
            .ok_or(EvtcError::InvalidData)?
            .to_str()?;
        let subgroup = raw_agent.name[character_name.len() + account_name.len() + 2] - b'0';
        let elite = if raw_agent.is_elite == 0 {
            None
        } else {
            Some(
                EliteSpec::from_u32(raw_agent.is_elite)
                    .ok_or(EvtcError::InvalidEliteSpec(raw_agent.is_elite))?,
            )
        };
        Ok(Player {
            profession: Profession::from_u32(raw_agent.prof)
                .ok_or(EvtcError::InvalidProfession(raw_agent.prof))?,
            elite,
            character_name: character_name.to_owned(),
            account_name: account_name.to_owned(),
            subgroup,
        })
    }

    /// Accesses the inner [`Player`][Player] struct, if available.
    pub fn as_player(&self) -> Option<&Player> {
        if let AgentKind::Player(ref player) = *self {
            Some(player)
        } else {
            None
        }
    }

    /// Determines whether this `AgentKind` contains a player.
    pub fn is_player(&self) -> bool {
        self.as_player().is_some()
    }

    /// Accesses the inner [`Gadget`][Gadget] struct, if available.
    pub fn as_gadget(&self) -> Option<&Gadget> {
        if let AgentKind::Gadget(ref gadget) = *self {
            Some(gadget)
        } else {
            None
        }
    }

    /// Determines whether this `AgentKind` contains a gadget.
    pub fn is_gadget(&self) -> bool {
        self.as_gadget().is_some()
    }

    /// Accesses the inner [`Character`][Character] struct, if available.
    pub fn as_character(&self) -> Option<&Character> {
        if let AgentKind::Character(ref character) = *self {
            Some(character)
        } else {
            None
        }
    }

    /// Determines whether this `AgentKind` contains a character.
    pub fn is_character(&self) -> bool {
        self.as_character().is_some()
    }
}

impl TryFrom<raw::Agent> for AgentKind {
    type Error = EvtcError;
    /// Convenience method to avoid manual borrowing.
    ///
    /// Note that this conversion will consume the agent, so if you plan on re-using it, use the
    /// `TryFrom<&raw::Agent>` implementation that works with a reference.
    fn try_from(raw_agent: raw::Agent) -> Result<Self, Self::Error> {
        Self::try_from(&raw_agent)
    }
}

impl TryFrom<&raw::Agent> for AgentKind {
    type Error = EvtcError;

    /// Extract the correct `AgentKind` from the given [raw agent][raw::Agent].
    ///
    /// This automatically discerns between player, gadget and characters.
    ///
    /// Note that in most cases, you probably want to use `Agent::try_from` or even
    /// [`process`][process] instead of this function.
    fn try_from(raw_agent: &raw::Agent) -> Result<Self, Self::Error> {
        if raw_agent.is_character() {
            Ok(AgentKind::Character(AgentKind::from_raw_character(
                raw_agent,
            )?))
        } else if raw_agent.is_gadget() {
            Ok(AgentKind::Gadget(AgentKind::from_raw_gadget(raw_agent)?))
        } else if raw_agent.is_player() {
            Ok(AgentKind::Player(AgentKind::from_raw_player(raw_agent)?))
        } else {
            Err(EvtcError::InvalidData)
        }
    }
}

/// An agent.
///
/// Agents in arcdps are very versatile, as a lot of things end up being an "agent". This includes:
/// * Players
/// * Bosses
/// * Any additional mobs that spawn
/// * Mesmer illusions
/// * Ranger spirits, pets
/// * Guardian spirit weapons
/// * ...
///
/// Generally, you can divide them into three kinds ([`AgentKind`][AgentKind]):
/// * [`Player`][Player]: All players themselves.
/// * [`Character`][Character]: Non-player mobs, including most bosses, "adds" and player-generated
///   characters.
/// * [`Gadget`][Gadget]: Some additional gadgets, such as ley rifts, continuum split, ...
///
/// All of these agents share some common fields, which are the ones accessible in `Agent<Kind>`.
/// The kind can be retrieved using [`.kind()`][Agent::kind], which can be matched on.
///
/// # Obtaining an agent
///
/// The normal way to obtain the agents is to use the [`.agents()`](Log::agents) method on a
/// [`Log`][Log], or one of the other accessor methods (like [`.players()`][Log::players] or
/// [`.agent_by_addr()`][Log::agent_by_addr]).
///
/// In the cases where you already have a [`raw::Agent`][raw::Agent] available, you can also
/// convert it to an [`Agent`][Agent] by using the standard
/// [`TryFrom`][TryFrom]/[`TryInto`][std::convert::TryInto] traits:
///
/// ```no_run
/// # use evtclib::{Agent, raw};
/// use std::convert::TryInto;
/// let raw_agent: raw::Agent = panic!();
/// let agent: Agent = raw_agent.try_into().unwrap();
/// ```
///
/// Note that you can convert references as well, so if you plan on re-using the raw agent
/// afterwards, you should opt for `Agent::try_from(&raw_agent)` instead.
///
/// # The `Kind` parameter
///
/// The type parameter is not actually used and only exists at the type level. It can be used to
/// tag `Agent`s containing a known kind. For example, `Agent<Player>` implements
/// [`.player()`][Agent::player], which returns a `&Player` directly (instead of a
/// `Option<&Player>`). This works because such tagged `Agent`s can only be constructed (safely)
/// using [`.as_player()`][Agent::as_player], [`.as_gadget()`][Agent::as_gadget] or
/// [`.as_character()`][Agent::as_character]. This is useful since functions like
/// [`Log::players`][Log::players], which already filter only players, don't require the consumer
/// to do another check/pattern match for the right agent kind.
///
/// The unit type `()` is used to tag `Agent`s which contain an undetermined type, and it is the
/// default if you write `Agent` without any parameters.
///
/// The downside is that methods which work on `Agent`s theoretically should be generic over
/// `Kind`. An escape hatch is the method [`.erase()`][Agent::erase], which erases the kind
/// information and produces the default `Agent<()>`. Functions/methods that only take `Agent<()>`
/// can therefore be used by any other agent as well.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Getters, CopyGetters)]
// For the reasoning of #[repr(C)] see Agent::transmute.
#[repr(C)]
pub struct Agent<Kind = ()> {
    /// The address of this agent.
    ///
    /// This is not actually the address of the in-memory Rust object, but rather a serialization
    /// detail of arcdps. You should consider this as an opaque number and only compare it to other
    /// agent addresses.
    #[get_copy = "pub"]
    addr: u64,

    /// The kind of this agent.
    #[get = "pub"]
    kind: AgentKind,

    /// The toughness of this agent.
    ///
    /// This is not an absolute number, but a relative indicator that indicates this agent's
    /// toughness relative to the other people in the squad.
    ///
    /// 0 means lowest toughness, 10 means highest toughness.
    #[get_copy = "pub"]
    toughness: i16,

    /// The concentration of this agent.
    ///
    /// This is not an absolute number, but a relative indicator that indicates this agent's
    /// concentration relative to the other people in the squad.
    ///
    /// 0 means lowest concentration, 10 means highest concentration.
    #[get_copy = "pub"]
    concentration: i16,

    /// The healing power of this agent.
    ///
    /// This is not an absolute number, but a relative indicator that indicates this agent's
    /// healing power relative to the other people in the squad.
    ///
    /// 0 means lowest healing power, 10 means highest healing power.
    #[get_copy = "pub"]
    healing: i16,

    /// The condition damage of this agent.
    ///
    /// This is not an absolute number, but a relative indicator that indicates this agent's
    /// condition damage relative to the other people in the squad.
    ///
    /// 0 means lowest condition damage, 10 means highest condition damage.
    #[get_copy = "pub"]
    condition: i16,

    /// The instance ID of this agent.
    #[get_copy = "pub"]
    instance_id: u16,

    /// The timestamp of the first event entry with this agent.
    #[get_copy = "pub"]
    first_aware: u64,

    /// The timestamp of the last event entry with this agent.
    #[get_copy = "pub"]
    last_aware: u64,

    /// The master agent's address.
    #[get_copy = "pub"]
    master_agent: Option<u64>,

    phantom_data: PhantomData<Kind>,
}

impl TryFrom<&raw::Agent> for Agent {
    type Error = EvtcError;

    /// Parse a raw agent.
    fn try_from(raw_agent: &raw::Agent) -> Result<Self, Self::Error> {
        let kind = AgentKind::try_from(raw_agent)?;
        Ok(Agent {
            addr: raw_agent.addr,
            kind,
            toughness: raw_agent.toughness,
            concentration: raw_agent.concentration,
            healing: raw_agent.healing,
            condition: raw_agent.condition,
            instance_id: 0,
            first_aware: 0,
            last_aware: u64::max_value(),
            master_agent: None,
            phantom_data: PhantomData,
        })
    }
}

impl TryFrom<raw::Agent> for Agent {
    type Error = EvtcError;

    /// Convenience method to avoid manual borrowing.
    ///
    /// Note that this conversion will consume the agent, so if you plan on re-using it, use the
    /// `TryFrom<&raw::Agent>` implementation that works with a reference.
    fn try_from(raw_agent: raw::Agent) -> Result<Self, Self::Error> {
        Agent::try_from(&raw_agent)
    }
}

impl<Kind> Agent<Kind> {
    /// Unconditionally change the tagged type.
    #[inline]
    fn transmute<T>(&self) -> &Agent<T> {
        // Beware, unsafe code ahead!
        //
        // What are we doing here?
        // In Agent<T>, T is a marker type that only exists at the type level. There is no actual
        // value of type T being held, instead, we use PhantomData under the hood. This is so we
        // can implement special methods on Agent<Player>, Agent<Gadget> and Agent<Character>,
        // which allows us in some cases to avoid the "second check" (e.g. Log::players() can
        // return Agent<Player>, as the function already makes sure all returned agents are
        // players). This makes the interface more ergonomical, as we can prove to the type checker
        // at compile time that a given Agent has a certain AgentKind.
        //
        // Why is this safe?
        // PhantomData<T> (which is what Agent<T> boils down to) is a zero-sized type, which means
        // it does not actually change the layout of the struct. There is some discussion in [1],
        // which suggests that this is true for #[repr(C)] structs (which Agent is). We can
        // therefore safely transmute from Agent<U> to Agent<T>, for any U and T.
        //
        // Can this lead to unsafety?
        // No, the actual data access is still done through safe rust and a if-let. In the worst
        // case it can lead to an unexpected panic, but the "guarantee" made by T is rather weak in
        // that regard.
        //
        // What are the alternatives?
        // None, as far as I'm aware. Going from Agent<U> to Agent<T> is possible in safe Rust by
        // destructuring the struct, or alternatively by [2] (if it would be implemented). However,
        // when dealing with references, there seems to be no way to safely go from Agent<U> to
        // Agent<T>, even if they share the same layout.
        //
        // [1]: https://www.reddit.com/r/rust/comments/avrbvc/is_it_safe_to_transmute_foox_to_fooy_if_the/
        // [2]: https://github.com/rust-lang/rfcs/pull/2528
        unsafe { &*(self as *const Agent<Kind> as *const Agent<T>) }
    }

    /// Erase any extra information about the contained agent kind.
    #[inline]
    pub fn erase(&self) -> &Agent {
        self.transmute()
    }

    /// Try to convert this `Agent` to an `Agent` representing a `Player`.
    #[inline]
    pub fn as_player(&self) -> Option<&Agent<Player>> {
        if self.kind.is_player() {
            Some(self.transmute())
        } else {
            None
        }
    }

    /// Try to convert this `Agent` to an `Agent` representing a `Gadget`.
    #[inline]
    pub fn as_gadget(&self) -> Option<&Agent<Gadget>> {
        if self.kind.is_gadget() {
            Some(self.transmute())
        } else {
            None
        }
    }

    /// Try to convert this `Agent` to an `Agent` representing a `Character`.
    #[inline]
    pub fn as_character(&self) -> Option<&Agent<Character>> {
        if self.kind.is_character() {
            Some(self.transmute())
        } else {
            None
        }
    }
}

impl Agent<Player> {
    /// Directly access the underlying player data.
    #[inline]
    pub fn player(&self) -> &Player {
        self.kind.as_player().expect("Agent<Player> had no player!")
    }

    /// Shorthand to get the player's account name.
    #[inline]
    pub fn account_name(&self) -> &str {
        self.player().account_name()
    }

    /// Shorthand to get the player's character name.
    #[inline]
    pub fn character_name(&self) -> &str {
        self.player().character_name()
    }

    /// Shorthand to get the player's elite specialization.
    #[inline]
    pub fn elite(&self) -> Option<EliteSpec> {
        self.player().elite()
    }

    /// Shorthand to get the player's profession.
    #[inline]
    pub fn profession(&self) -> Profession {
        self.player().profession()
    }

    /// Shorthand to get the player's subgroup.
    #[inline]
    pub fn subgroup(&self) -> u8 {
        self.player().subgroup()
    }
}

impl Agent<Gadget> {
    /// Directly access the underlying gadget data.
    #[inline]
    pub fn gadget(&self) -> &Gadget {
        self.kind.as_gadget().expect("Agent<Gadget> had no gadget!")
    }

    /// Shorthand to get the gadget's id.
    #[inline]
    pub fn id(&self) -> u16 {
        self.gadget().id()
    }

    /// Shorthand to get the gadget's name.
    #[inline]
    pub fn name(&self) -> &str {
        self.gadget().name()
    }
}

impl Agent<Character> {
    /// Directly access the underlying character data.
    #[inline]
    pub fn character(&self) -> &Character {
        self.kind
            .as_character()
            .expect("Agent<Character> had no character!")
    }

    /// Shorthand to get the character's id.
    #[inline]
    pub fn id(&self) -> u16 {
        self.character().id()
    }

    /// Shorthand to get the character's name.
    #[inline]
    pub fn name(&self) -> &str {
        self.character().name()
    }
}

/// A fully processed log file.
#[derive(Debug, Clone)]
pub struct Log {
    agents: Vec<Agent>,
    events: Vec<Event>,
    boss_id: u16,
}

impl Log {
    /// Return all agents present in this log.
    #[inline]
    pub fn agents(&self) -> &[Agent] {
        &self.agents
    }

    /// Return an agent based on its address.
    pub fn agent_by_addr(&self, addr: u64) -> Option<&Agent> {
        self.agents.iter().find(|a| a.addr == addr)
    }

    /// Return an agent based on the instance ID.
    pub fn agent_by_instance_id(&self, instance_id: u16) -> Option<&Agent> {
        self.agents.iter().find(|a| a.instance_id == instance_id)
    }

    /// Return the master agent of the given agent.
    ///
    /// * `addr` - The address of the agent which to get the master for.
    pub fn master_agent(&self, addr: u64) -> Option<&Agent> {
        self.agent_by_addr(addr)
            .and_then(|a| a.master_agent)
            .and_then(|a| self.agent_by_addr(a))
    }

    /// Return an iterator over all agents that represent player characters.
    pub fn players(&self) -> impl Iterator<Item = &Agent<Player>> {
        self.agents.iter().filter_map(|a| a.as_player())
    }

    /// Return an iterator over all agents that are NPCs.
    pub fn npcs(&self) -> impl Iterator<Item = &Agent<Character>> {
        self.agents.iter().filter_map(|a| a.as_character())
    }

    /// Return the boss agent.
    ///
    /// Be careful with encounters that have multiple boss agents, such as Trio
    /// and Xera.
    pub fn boss(&self) -> &Agent {
        self.npcs()
            .find(|c| c.character().id == self.boss_id)
            .map(Agent::erase)
            .expect("Boss has no agent!")
    }

    /// Return all boss agents.
    ///
    /// This correctly returns multiple agents on encounters where multiple
    /// agents are needed.
    pub fn boss_agents(&self) -> Vec<&Agent> {
        let boss_ids = if self.boss_id == Boss::Xera as u16 {
            vec![self.boss_id, gamedata::XERA_PHASE2_ID]
        } else {
            vec![self.boss_id]
        };
        self.npcs()
            .filter(|c| boss_ids.contains(&c.character().id))
            .map(Agent::erase)
            .collect()
    }

    /// Check whether the given address is a boss agent.
    pub fn is_boss(&self, addr: u64) -> bool {
        self.boss_agents().into_iter().any(|a| a.addr() == addr)
    }

    /// Returns the encounter id.
    #[inline]
    pub fn encounter_id(&self) -> u16 {
        self.boss_id
    }

    /// Returns the encounter, if present.
    ///
    /// Some logs don't have an encounter set or have an ID that is unknown to us (for example, if
    /// people set up arcdps with custom IDs). Therefore, this method can only return the encounter
    /// if we know about it in [`Boss`][Boss].
    #[inline]
    pub fn encounter(&self) -> Option<Boss> {
        Boss::from_u16(self.boss_id)
    }

    /// Return all events present in this log.
    #[inline]
    pub fn events(&self) -> &[Event] {
        &self.events
    }
}

/// Convenience data accessing funtions for [`Log`][Log]s.
///
/// The information that is gathered by those functions is "expensive" to compute, as we have to
/// loop through every event. They are not saved in the header, and instead are implemented using
/// special [`EventKind`][EventKind]s. This is not a limitation of `evtclib`, but rather a result
/// of how arcdps stores the data.
///
/// This also means that those functions are fallible because we cannot guarantee that the special
/// events that we're looking for is actually present in every log file.
///
/// Use those functions only if necessary, and prefer to cache the result if it will be reused!
impl Log {
    /// Get the timestamp of when the log was started.
    ///
    /// The returned value is a unix timestamp in the local time zone.
    ///
    /// If the [`LogStart`][EventKind::LogStart] event cannot be found, this function returns
    /// `None`.
    pub fn local_start_timestamp(&self) -> Option<u32> {
        self.events().iter().find_map(|e| {
            if let EventKind::LogStart {
                local_timestamp, ..
            } = e.kind
            {
                Some(local_timestamp)
            } else {
                None
            }
        })
    }

    /// Get the timestamp of when the log was ended.
    ///
    /// The returned value is a unix timestamp in the local time zone.
    ///
    /// If the [`LogEnd`][EventKind::LogEnd] event cannot be found, this function returns `None`.
    pub fn local_end_timestamp(&self) -> Option<u32> {
        self.events().iter().find_map(|e| {
            if let EventKind::LogEnd {
                local_timestamp, ..
            } = e.kind
            {
                Some(local_timestamp)
            } else {
                None
            }
        })
    }

    /// Check if rewards for this fight have been given out.
    ///
    /// This can be used as an indication whether the fight was successful (`true`) or not
    /// (`false`).
    pub fn was_rewarded(&self) -> bool {
        self.events().iter().any(|e| {
            if let EventKind::Reward { .. } = e.kind {
                true
            } else {
                false
            }
        })
    }
}

/// Main function to turn a low-level [`Evtc`][raw::Evtc] to a high-level [`Log`][Log].
///
/// This function takes an [`Evtc`][raw::Evtc] and does the required type conversions and
/// pre-processing to get a high-level [`Log`][Log]. This pre-processing includes
///
/// * Setting the correct aware times for the agents
/// * Setting the master agents for each agent
/// * Converting all events
///
/// Note that the structures are quite different, so this function does not consume the given
/// [`Evtc`][raw::Evtc].
pub fn process(data: &raw::Evtc) -> Result<Log, EvtcError> {
    // Prepare "augmented" agents
    let mut agents = setup_agents(data)?;
    // Do the first aware/last aware field
    set_agent_awares(data, &mut agents)?;

    // Set the master addr field
    set_agent_masters(data, &mut agents)?;

    let events = data
        .events
        .iter()
        .filter_map(|e| Event::try_from(e).ok())
        .collect();

    Ok(Log {
        agents,
        events,
        boss_id: data.header.combat_id,
    })
}

fn setup_agents(data: &raw::Evtc) -> Result<Vec<Agent>, EvtcError> {
    let mut agents = Vec::with_capacity(data.agents.len());

    for raw_agent in &data.agents {
        agents.push(Agent::try_from(raw_agent)?);
    }
    Ok(agents)
}

fn get_agent_by_addr(agents: &mut [Agent], addr: u64) -> Option<&mut Agent> {
    for agent in agents {
        if agent.addr == addr {
            return Some(agent);
        }
    }
    None
}

fn set_agent_awares(data: &raw::Evtc, agents: &mut [Agent]) -> Result<(), EvtcError> {
    for event in &data.events {
        if event.is_statechange == raw::CbtStateChange::None {
            if let Some(current_agent) = get_agent_by_addr(agents, event.src_agent) {
                current_agent.instance_id = event.src_instid;
                if current_agent.first_aware == 0 {
                    current_agent.first_aware = event.time;
                }
                current_agent.last_aware = event.time;
            }
        }
    }
    Ok(())
}

fn set_agent_masters(data: &raw::Evtc, agents: &mut [Agent]) -> Result<(), EvtcError> {
    for event in &data.events {
        if event.src_master_instid != 0 {
            let mut master_addr = None;
            for agent in &*agents {
                if agent.instance_id == event.src_master_instid
                    && agent.first_aware < event.time
                    && event.time < agent.last_aware
                {
                    master_addr = Some(agent.addr);
                    break;
                }
            }
            if let Some(master_addr) = master_addr {
                if let Some(current_slave) = get_agent_by_addr(agents, event.src_agent) {
                    current_slave.master_agent = Some(master_addr);
                }
            }
        }
    }
    Ok(())
}
