//! `evtclib` is a crate aiming to provide utility functions to parse and work
//! with `.evtc` reports generated by arcdps.
//!
//! # Workflow
//!
//! ```no_run
//! # use std::fs::File;
//! // Open some file for processing
//! let mut file = File::open("my_log.evtc").unwrap();
//! // Parse the raw content of the file
//! let raw_log = evtclib::raw::parse_file(&mut file).unwrap();
//! // Process the file to do the nitty-gritty low-level stuff done
//! let log = evtclib::process(&raw_log).unwrap();
//! // Do work on the log
//! ```
//!
//! (Look at the note on "Buffering" in the [parser
//! module](raw/parser/index.html#buffering))

use std::marker::PhantomData;

use getset::{CopyGetters, Getters};
use thiserror::Error;

pub mod raw;

mod event;
pub use event::{Event, EventKind};

pub mod gamedata;
pub use gamedata::Boss;

#[derive(Error, Debug)]
pub enum EvtcError {
    #[error("invalid data has been provided")]
    InvalidData,
    #[error("utf8 decoding error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

/// Player-specific agent data.
#[derive(Debug, Clone, PartialEq, Eq, CopyGetters)]
pub struct Player {
    /// The player's profession.
    #[get_copy = "pub"]
    profession: u32,

    /// The player's elite specialization.
    #[get_copy = "pub"]
    elite: u32,

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
#[derive(Debug, Clone, PartialEq, Eq, CopyGetters)]
pub struct Gadget {
    #[get_copy = "pub"]
    id: u16,
    name: String,
}

impl Gadget {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq, Eq, CopyGetters)]
pub struct Character {
    #[get_copy = "pub"]
    id: u16,
    name: String,
}

impl Character {
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// The type of an agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentKind {
    Player(Player),
    Gadget(Gadget),
    Character(Character),
}

impl AgentKind {
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
#[derive(Debug, Clone, Getters, CopyGetters)]
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

impl Agent {
    /// Parse a raw agent.
    pub fn from_raw(raw_agent: &raw::Agent) -> Result<Agent, EvtcError> {
        let kind = if raw_agent.is_character() || raw_agent.is_gadget() {
            let name = String::from_utf8(
                raw_agent
                    .name
                    .iter()
                    .cloned()
                    .take_while(|c| *c != 0)
                    .collect::<Vec<_>>(),
            )?;
            if raw_agent.is_character() {
                AgentKind::Character(Character {
                    id: raw_agent.prof as u16,
                    name,
                })
            } else {
                AgentKind::Gadget(Gadget {
                    id: raw_agent.prof as u16,
                    name,
                })
            }
        } else if raw_agent.is_player() {
            let first = raw_agent
                .name
                .iter()
                .cloned()
                .take_while(|c| *c != 0)
                .collect::<Vec<_>>();
            let second = raw_agent
                .name
                .iter()
                .cloned()
                .skip(first.len() + 1)
                .take_while(|c| *c != 0)
                .collect::<Vec<_>>();
            let third = raw_agent.name[first.len() + second.len() + 2] - b'0';
            AgentKind::Player(Player {
                profession: raw_agent.prof,
                elite: raw_agent.is_elite,
                character_name: String::from_utf8(first)?,
                account_name: String::from_utf8(second)?,
                subgroup: third,
            })
        } else {
            return Err(EvtcError::InvalidData);
        };

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

impl<Kind> Agent<Kind> {
    /// Unconditionally change the tagged type.
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
    pub fn erase(&self) -> &Agent {
        self.transmute()
    }

    /// Try to convert this `Agent` to an `Agent` representing a `Player`.
    pub fn as_player(&self) -> Option<&Agent<Player>> {
        if self.kind.is_player() {
            Some(self.transmute())
        } else {
            None
        }
    }

    /// Try to convert this `Agent` to an `Agent` representing a `Gadget`.
    pub fn as_gadget(&self) -> Option<&Agent<Gadget>> {
        if self.kind.is_gadget() {
            Some(self.transmute())
        } else {
            None
        }
    }

    /// Try to convert this `Agent` to an `Agent` representing a `Character`.
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
    pub fn player(&self) -> &Player {
        self.kind.as_player().expect("Agent<Player> had no player!")
    }
}

impl Agent<Gadget> {
    /// Directly access the underlying gadget data.
    pub fn gadget(&self) -> &Gadget {
        self.kind.as_gadget().expect("Agent<Gadget> had no gadget!")
    }
}

impl Agent<Character> {
    /// Directly access the underlying character data.
    pub fn character(&self) -> &Character {
        self.kind
            .as_character()
            .expect("Agent<Character> had no character!")
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

    /// Returns the boss/encounter id.
    pub fn boss_id(&self) -> u16 {
        self.boss_id
    }

    /// Return all events present in this log.
    pub fn events(&self) -> &[Event] {
        &self.events
    }
}

pub fn process(data: &raw::Evtc) -> Result<Log, EvtcError> {
    // Prepare "augmented" agents
    let mut agents = setup_agents(data)?;
    // Do the first aware/last aware field
    set_agent_awares(data, &mut agents)?;

    // Set the master addr field
    set_agent_masters(data, &mut agents)?;

    let events = data.events.iter().filter_map(Event::from_raw).collect();

    Ok(Log {
        agents,
        events,
        boss_id: data.header.combat_id,
    })
}

fn setup_agents(data: &raw::Evtc) -> Result<Vec<Agent>, EvtcError> {
    let mut agents = Vec::with_capacity(data.agents.len());

    for raw_agent in &data.agents {
        agents.push(Agent::from_raw(raw_agent)?);
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
