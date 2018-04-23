#![feature(try_trait)]
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate num_derive;
extern crate byteorder;
extern crate num_traits;

pub mod raw;

mod event;
pub use event::{Event, EventKind};

quick_error! {
    #[derive(Debug)]
    pub enum EvtcError {
        InvalidData {
            description("invalid data has been provided")
        }
        Utf8Error(err: ::std::string::FromUtf8Error) {
            from()
            description("utf8 decoding error")
            display("UTF-8 decoding error: {}", err)
            cause(err)
        }
    }
}

/// The type of an agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentKind {
    Player { profession: u32, elite: u32 },
    Gadget(u16),
    Character(u16),
}

/// Name of an agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentName {
    Single(String),
    Player {
        character_name: String,
        account_name: String,
        subgroup: u8,
    },
}

/// An agent.
#[derive(Debug, Clone)]
pub struct Agent {
    addr: u64,
    kind: AgentKind,
    toughness: i16,
    concentration: i16,
    healing: i16,
    condition: i16,
    name: AgentName,
    instance_id: u16,
    first_aware: u64,
    last_aware: u64,
    master_agent: Option<u64>,
}

/// A fully processed log file.
#[derive(Debug, Clone)]
pub struct Log {
    agents: Vec<Agent>,
}

pub fn process(data: &raw::Evtc) -> Result<Log, EvtcError> {
    // Prepare "augmented" agents
    let mut agents = setup_agents(data)?;

    // Do the first aware/last aware field
    set_agent_awares(data, &mut agents)?;

    // Set the master addr field
    set_agent_masters(data, &mut agents)?;

    for agent in &agents {
        if let AgentKind::Player { .. } = agent.kind {
            println!("Agent: {:#?}", agent);
        }
    }

    panic!();
}

fn setup_agents(data: &raw::Evtc) -> Result<Vec<Agent>, EvtcError> {
    let mut agents = Vec::with_capacity(data.agents.len());

    for raw_agent in &data.agents {
        let kind = if raw_agent.is_character() {
            AgentKind::Character(raw_agent.prof as u16)
        } else if raw_agent.is_gadget() {
            AgentKind::Gadget(raw_agent.prof as u16)
        } else if raw_agent.is_player() {
            AgentKind::Player {
                profession: raw_agent.prof,
                elite: raw_agent.is_elite,
            }
        } else {
            return Err(EvtcError::InvalidData);
        };

        let name = if raw_agent.is_player() {
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
            AgentName::Player {
                character_name: String::from_utf8(first)?,
                account_name: String::from_utf8(second)?,
                subgroup: third,
            }
        } else {
            let name = raw_agent
                .name
                .iter()
                .cloned()
                .take_while(|c| *c != 0)
                .collect::<Vec<_>>();
            AgentName::Single(String::from_utf8(name)?)
        };

        let agent = Agent {
            addr: raw_agent.addr,
            kind,
            toughness: raw_agent.toughness,
            concentration: raw_agent.concentration,
            healing: raw_agent.healing,
            condition: raw_agent.condition,
            name,
            instance_id: 0,
            first_aware: 0,
            last_aware: u64::max_value(),
            master_agent: None,
        };

        agents.push(agent);
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
                if agent.instance_id == event.src_master_instid && agent.first_aware < event.time
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
