//! Private module to contain the processing functions.

use std::{
    convert::TryFrom,
    fs::File,
    io::{BufReader, Read, Seek},
    path::Path,
};

use super::{raw, Agent, Event, EvtcError, Log};

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
    // We sort the agents so we can do a binary search later in get_agent_by_addr. The order is not
    // really defined or important anyway, so we can just choose whatever works best here.
    agents.sort_by_key(Agent::addr);
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

/// Indicates the given compression method for the file.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Compression {
    /// No compression was used.
    None,
    /// The file is wrapped in a zip archive.
    Zip,
}

/// Convenience function to process a given stream directly.
///
/// This is a shorthand for using [`raw::parse_file`][raw::parse_file] followed by
/// [`process`][process].
///
/// The [`Seek`][Seek] bound is needed for zip compressed archives. If you have a reader that does
/// not support seeking, you can use [`raw::parse_file`][raw::parse_file] directly instead.
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::io::Cursor;
/// use evtclib::Compression;
/// let data = Cursor::new(vec![]);
/// let log = evtclib::process_stream(data, Compression::None)?;
/// # Ok(()) }
/// ```
pub fn process_stream<R: Read + Seek>(
    input: R,
    compression: Compression,
) -> Result<Log, EvtcError> {
    let evtc = match compression {
        Compression::None => raw::parse_file(input)?,
        Compression::Zip => raw::parse_zip(input)?,
    };
    process(&evtc)
}

/// Convenience function to process a given file directly.
///
/// This is a shorthand for opening the file and then using [`process_stream`][process_stream] with
/// it. This function automatically wraps the raw file in a buffered reader, to ensure the best
/// reading performance.
///
/// If you need more fine-grained control, consider using [`process_stream`][process_stream] or
/// [`raw::parse_file`][raw::parse_file] followed by [`process`][process] instead.
///
/// ```no_run
/// # use evtclib::Compression;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let log = evtclib::process_file("logfile.zevtc", Compression::Zip)?;
/// # Ok(()) }
/// ```
pub fn process_file<P: AsRef<Path>>(path: P, compression: Compression) -> Result<Log, EvtcError> {
    let file = File::open(path).map_err(Into::<raw::ParseError>::into)?;
    let buffered = BufReader::new(file);
    process_stream(buffered, compression)
}

fn setup_agents(data: &raw::Evtc) -> Result<Vec<Agent>, EvtcError> {
    data.agents.iter().map(Agent::try_from).collect()
}

fn get_agent_by_addr(agents: &mut [Agent], addr: u64) -> Option<&mut Agent> {
    let pos = agents.binary_search_by_key(&addr, Agent::addr).ok()?;
    Some(&mut agents[pos])
}

fn set_agent_awares(data: &raw::Evtc, agents: &mut [Agent]) -> Result<(), EvtcError> {
    for event in &data.events {
        if event.is_statechange == raw::CbtStateChange::None {
            if let Some(current_agent) = get_agent_by_addr(agents, event.src_agent) {
                current_agent.set_instance_id(event.src_instid);
                if current_agent.first_aware() == 0 {
                    current_agent.set_first_aware(event.time);
                }
                current_agent.set_last_aware(event.time);
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
                if agent.instance_id() == event.src_master_instid
                    && agent.first_aware() < event.time
                    && event.time < agent.last_aware()
                {
                    master_addr = Some(agent.addr());
                    break;
                }
            }
            if let Some(master_addr) = master_addr {
                if let Some(current_slave) = get_agent_by_addr(agents, event.src_agent) {
                    current_slave.set_master_agent(Some(master_addr));
                }
            }
        }
    }
    Ok(())
}
