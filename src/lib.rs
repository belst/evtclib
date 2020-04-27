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

use thiserror::Error;
use getset::{CopyGetters, Getters};

pub mod raw;

mod event;
pub use event::{Event, EventKind};

pub mod gamedata;
pub use gamedata::Boss;

/// A macro that returns `true` when the given expression matches the pattern.
///
/// ```rust
/// assert!(matches!(Some(4), Some(_)));
/// assert!(!matches!(Some(2), None));
/// ```
macro_rules! matches {
    ($expression:expr, $($pattern:pat)|*) => (
        match $expression {
            $($pattern)|+ => true,
            _ => false,
        }
    );
    ($expression:expr, $pattern:pat if $condi:expr) => (
        match $expression {
            $pattern if $condi => true,
            _ => false,
        }
    );
}

#[derive(Error, Debug)]
pub enum EvtcError {
    #[error("invalid data has been provided")]
    InvalidData,
    #[error("utf8 decoding error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

/// The type of an agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentKind {
    Player {
        profession: u32,
        elite: u32,
        character_name: String,
        account_name: String,
        subgroup: u8,
    },
    Gadget(u16, String),
    Character(u16, String),
}

/// An agent.
#[derive(Debug, Clone, Getters, CopyGetters)]
pub struct Agent {
    #[get_copy = "pub"]
    addr: u64,
    #[get = "pub"]
    kind: AgentKind,
    #[get_copy = "pub"]
    toughness: i16,
    #[get_copy = "pub"]
    concentration: i16,
    #[get_copy = "pub"]
    healing: i16,
    #[get_copy = "pub"]
    condition: i16,
    #[get_copy = "pub"]
    instance_id: u16,
    #[get_copy = "pub"]
    first_aware: u64,
    #[get_copy = "pub"]
    last_aware: u64,
    #[get_copy = "pub"]
    master_agent: Option<u64>,
}

impl Agent {
    /// Parse a raw agent.
    pub fn from_raw(raw_agent: &raw::Agent) -> Result<Agent, EvtcError> {
        let kind = if raw_agent.is_character() || raw_agent.is_gadget() {
            let name = String::from_utf8(raw_agent
                .name
                .iter()
                .cloned()
                .take_while(|c| *c != 0)
                .collect::<Vec<_>>())?;
            if raw_agent.is_character() {
                AgentKind::Character(raw_agent.prof as u16, name)
            } else {
                AgentKind::Gadget(raw_agent.prof as u16, name)
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
            AgentKind::Player {
                profession: raw_agent.prof,
                elite: raw_agent.is_elite,
                character_name: String::from_utf8(first)?,
                account_name: String::from_utf8(second)?,
                subgroup: third,
            }
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
        })
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
    pub fn players(&self) -> impl Iterator<Item = &Agent> {
        self.agents
            .iter()
            .filter(|a| matches!(a.kind, AgentKind::Player { .. }))
    }

    /// Return an iterator over all agents that are NPCs.
    pub fn npcs(&self) -> impl Iterator<Item = &Agent> {
        self.agents
            .iter()
            .filter(|a| matches!(a.kind, AgentKind::Character(..)))
    }

    /// Return the boss agent.
    ///
    /// Be careful with encounters that have multiple boss agents, such as Trio
    /// and Xera.
    pub fn boss(&self) -> &Agent {
        self.npcs()
            .find(|n| matches!(n.kind, AgentKind::Character(x, _) if x == self.boss_id))
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
            .filter(|a| matches!(a.kind, AgentKind::Character(x, _) if boss_ids.contains(&x)))
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
