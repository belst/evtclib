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
//! `evtclib` provides two convenience functions to obtain a [`Log`][Log]:
//!
//! If you have a stream (that is, something that is [`Read`][std::io::Read] +
//! [`Seek`][std::io::Seek]), you can use [`process_stream`][process_stream] to obtain a
//! [`Log`][Log] by reading from the stream.
//!
//! If your evtc is saved in a file, you can use [`process_file`][process_file] to obtain a [`Log`]
//! from it. This will also ensure that the buffering is set up correctly, to avoid unnecessary
//! system calls.
//!
//! Both of those functions require the reader to be seekable, as that is what we need for zip
//! archive support. If you cannot provide that, or if you need finer grained control for other
//! reasons, you can use either [`raw::parse_file`][raw::parse_file] or
//! [`raw::parse_zip`][raw::parse_zip] to obtain the low-level [`Evtc`][raw::Evtc] structure, and
//! then turn it into a [`Log`][Log] by using [`process`][process]:
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use evtclib::{Compression, Log};
//! use std::fs::File;
//! // Preferred:
//! let log: Log = evtclib::process_file("my_log.evtc", Compression::None)?;
//!
//! // If you have a stream:
//! let file = File::open("my_log.evtc")?;
//! let log: Log = evtclib::process_stream(file, Compression::None)?;
//!
//! // If you really need to do it manually:
//! // Open a file for processing
//! let file = File::open("my_log.evtc")?;
//! // Parse the raw content of the file
//! let raw_log = evtclib::raw::parse_file(file)?;
//! // Process the file to do the nitty-gritty low-level stuff done
//! let log: Log = evtclib::process(&raw_log)?;
//!
//! // In all cases, you can now do work with the log
//! for player in log.players() {
//!     println!("Player {} participated!", player.account_name());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! Make sure to take a look at the note on "Buffering" in the [parser
//! module](raw/parser/index.html#buffering) in order to increase the speed of your application.
//!
//! # Writing evtc Files
//!
//! Currently, `evtclib` does not provide a way to output or modify evtc files. This is for two
//! reasons:
//!
//! * The only sensible source for logs is the arcdps addon itself, most applications only consume
//! them.
//! * The library was needed for reading support, and writing support has never been a priority.
//!
//! While there are legitimate use cases for writing/modification support, they are currently not
//! implemented (but might be in a future version).

use thiserror::Error;

pub mod raw;

mod agent;
pub use agent::{Agent, AgentKind, Character, Gadget, Player};

pub mod event;
pub use event::{Event, EventKind};

mod processing;
pub use processing::{process, process_file, process_stream, Compression};

pub mod gamedata;
pub use gamedata::{EliteSpec, Encounter, Profession};

pub mod analyzers;
pub use analyzers::{Analyzer, Outcome};

/// Any error that can occur during the processing of evtc files.
#[derive(Error, Debug)]
pub enum EvtcError {
    /// Error for underlying parser errors.
    ///
    /// This should never be returned from [`process`][process], only from
    /// [`process_stream`][process_stream] and [`process_file`][process_file].
    #[error("the file could not be parsed: {0}")]
    ParseError(#[from] raw::ParseError),
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

/// A fully processed log file.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        self.agents.iter().find(|a| a.addr() == addr)
    }

    /// Return an agent based on the instance ID.
    pub fn agent_by_instance_id(&self, instance_id: u16) -> Option<&Agent> {
        self.agents.iter().find(|a| a.instance_id() == instance_id)
    }

    /// Return the master agent of the given agent.
    ///
    /// * `addr` - The address of the agent which to get the master for.
    pub fn master_agent(&self, addr: u64) -> Option<&Agent> {
        self.agent_by_addr(addr)
            .and_then(|a| a.master_agent())
            .and_then(|a| self.agent_by_addr(a))
    }

    /// Return an iterator over all agents that represent player characters.
    pub fn players(&self) -> impl Iterator<Item = &Agent<Player>> {
        self.agents.iter().filter_map(|a| a.as_player())
    }

    /// Return an iterator over all agents that are characters.
    pub fn characters(&self) -> impl Iterator<Item = &Agent<Character>> {
        self.agents.iter().filter_map(|a| a.as_character())
    }

    /// Return an iterator over all agents that are gadgets.
    pub fn gadgets(&self) -> impl Iterator<Item = &Agent<Gadget>> {
        self.agents.iter().filter_map(|a| a.as_gadget())
    }

    /// Return the boss agent.
    ///
    /// Be careful with encounters that have multiple boss agents, such as Trio
    /// and Xera.
    pub fn boss(&self) -> &Agent {
        self.characters()
            .find(|c| c.character().id() == self.boss_id)
            .map(Agent::erase)
            .expect("Boss has no agent!")
    }

    /// Return all boss agents.
    ///
    /// This correctly returns multiple agents on encounters where multiple
    /// agents are needed.
    pub fn boss_agents(&self) -> Vec<&Agent> {
        let bosses = self
            .encounter()
            .map(Encounter::bosses)
            .unwrap_or(&[] as &[_]);
        self.characters()
            .filter(|c| bosses.iter().any(|boss| *boss as u16 == c.character().id()))
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
    /// if we know about it in [`Encounter`].
    #[inline]
    pub fn encounter(&self) -> Option<Encounter> {
        Encounter::from_header_id(self.boss_id)
    }

    /// Return an analyzer suitable to analyze the given log.
    pub fn analyzer<'s>(&'s self) -> Option<Box<dyn Analyzer + 's>> {
        analyzers::for_log(self)
    }

    /// Return all events present in this log.
    #[inline]
    pub fn events(&self) -> &[Event] {
        &self.events
    }

    /// Returns the timespan of the log in milliseconds.
    ///
    /// The timespan is the time between the first registered event and the last registered event,
    /// measured in milliseconds.
    ///
    /// Note that this does not necessarily equate to the fight/encounter duration, as arcdps
    /// starts logging as soon as you enter combat, but some bosses are still invulnerable (e.g.
    /// Ensolyss). It does however give a good idea and is cheap to compute.
    ///
    /// In the rare occassions that a log does not have any events, this function will return 0.
    pub fn span(&self) -> u64 {
        let first = self.events().first().map(Event::time).unwrap_or(0);
        let last = self.events().last().map(Event::time).unwrap_or(0);
        last - first
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
    /// Check whether the fight was done with challenge mote activated.
    ///
    /// This function always returns `false` if
    /// * The fight was done without CM
    /// * The fight does not have a CM
    /// * We cannot determine whether the CM was active
    /// * The boss is not known
    pub fn is_cm(&self) -> bool {
        self.analyzer().map(|a| a.is_cm()).unwrap_or(false)
    }

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
            } = e.kind()
            {
                Some(*local_timestamp)
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
            } = e.kind()
            {
                Some(*local_timestamp)
            } else {
                None
            }
        })
    }

    /// Check if rewards for this fight have been given out.
    ///
    /// This can be used as an indication whether the fight was successful (`true`) or not
    /// (`false`).
    ///
    /// If you want to properly determine whether a fight was successful, check the
    /// [`Analyzer::outcome`][Analyzer::outcome] method, which does more sophisticated checks
    /// (dependent on the boss).
    pub fn was_rewarded(&self) -> bool {
        self.events()
            .iter()
            .any(|e| matches!(e.kind(), EventKind::Reward { .. }))
    }

    /// Returns all error strings that were captured.
    ///
    /// If no errors were encountered, an empty vec is returned.
    ///
    /// Note that those are errors reported verbatim by arcdps, nothing that evtclib
    /// produces/interprets.
    pub fn errors(&self) -> Vec<&str> {
        self.events()
            .iter()
            .filter_map(|e| {
                if let EventKind::Error { ref text } = e.kind() {
                    Some(text as &str)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Returns the game's build id.
    ///
    /// If no build id was found, `None` is returned.
    pub fn build_id(&self) -> Option<u64> {
        for event in self.events() {
            if let EventKind::Build { build } = event.kind() {
                return Some(*build);
            }
        }
        None
    }
}
