//! This module contains functions to parse an EVTC file.
//!
//! # Layout
//!
//! The general layout of the EVTC file is as follows:
//!
//! ```raw
//! magic number: b'EVTC'
//! arcdps build: yyyymmdd
//! nullbyte
//! encounter id
//! nullbyte
//! agent count
//! agents
//! skill count
//! skills
//! events
//! ```
//!
//! (refer to
//! [example.cpp](https://www.deltaconnected.com/arcdps/evtc/example.cpp) for
//! the exact data types).
//!
//! The parsing functions mirror the layout of the file and allow you to parse
//! single parts of the data (as long as your file cursor is at the right
//! position).
//!
//! All numbers are stored as little endian.
//!
//! arcdps stores the structs by just byte-dumping them. This means that you
//! have to be careful of the padding. `parse_agent` reads 96 bytes, even though
//! the struct definition only has 92.
//!
//! # Error handling
//!
//! Errors are wrapped in [`ParseError`](enum.ParseError.html). I/O errors are
//! wrapped as `ParseError::Io`. `EOF` is silently swallowed while reading the
//! events, as we expect the events to just go until the end of the file.
//!
//! Compared to the "original" enum definitions, we also add
//! [`IFF::None`](../enum.IFF.html) and
//! [`CbtResult::None`](../enum.CbtResult.html). This makes parsing easier, as
//! we can use those values instead of some other garbage. The other enums
//! already have the `None` variant, and the corresponding byte is zeroed, so
//! there's no problem with those.
//!
//! # Buffering
//!
//! Parsing the `evtc` file does many small reads. If you supply a raw reader,
//! each read requires a system call, and the overhead will be massive. It is
//! advised that you wrap the readers in a `BufReader`, if the underlying reader
//! does not do buffering on its own:
//!
//! ```no_run
//! use std::io::BufReader;
//! use std::fs::File;
//! let mut input = BufReader::new(File::open("log.evtc").unwrap());
//! let parsed = evtclib::raw::parse_file(&mut input);
//! ```
//!
//! ```raw
//! buffered: cargo run --release  0.22s user 0.04s system 94% cpu 0.275 total
//! raw file: cargo run --release  0.79s user 1.47s system 98% cpu 2.279 total
//! ```
//!
//! # Resources
//!
//! * [evtc readme](https://www.deltaconnected.com/arcdps/evtc/README.txt)
//! * [C++ output code](https://www.deltaconnected.com/arcdps/evtc/writeencounter.cpp)

use byteorder::{LittleEndian, ReadBytesExt, LE};
use num_traits::FromPrimitive;
use std::io::{self, ErrorKind, Read};
use thiserror::Error;

use super::*;

/// EVTC file header.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Header {
    /// arcpds build date, as `yyyymmdd` string.
    pub arcdps_build: String,
    /// Target species id.
    pub combat_id: u16,
    /// Agent count.
    pub agent_count: u32,
    /// Evtc revision
    pub revision: u8,
}

/// A completely parsed (raw) EVTC file.
///
/// Note that this struct does not yet do any preprocessing of the events. It is simply a
/// representation of the input file as a structured object.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Evtc {
    /// The file header values.
    pub header: Header,
    /// The skill count.
    pub skill_count: u32,
    /// The actual agents.
    pub agents: Vec<Agent>,
    /// The skills.
    pub skills: Vec<Skill>,
    /// The combat events.
    pub events: Vec<CbtEvent>,
}

/// A partially-parsed EVTC file, containing everything but the events.
///
/// This can speed up parsing for applications which can work with the header, as the event stream
/// is the largest chunk of data that has to be parsed.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct PartialEvtc {
    /// The file header values.
    pub header: Header,
    /// The skill count.
    pub skill_count: u32,
    /// The actual agents.
    pub agents: Vec<Agent>,
    /// The skills.
    pub skills: Vec<Skill>,
}

impl From<Evtc> for PartialEvtc {
    fn from(evtc: Evtc) -> Self {
        Self {
            header: evtc.header,
            skill_count: evtc.skill_count,
            agents: evtc.agents,
            skills: evtc.skills,
        }
    }
}

/// Any error that can occur during parsing.
#[derive(Error, Debug)]
pub enum ParseError {
    /// The error stems from an underlying input/output error.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    /// The error is caused by invalid UTF-8 data in the file.
    ///
    /// Names in the evtc are expected to be encoded with UTF-8.
    #[error("utf8 decoding error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    /// A generic error to signal invalid data has been encountered.
    #[error("invalid data")]
    InvalidData,
    /// The header is malformed.
    ///
    /// This is the error that you get when you try to parse a non-evtc file.
    #[error("malformed header")]
    MalformedHeader,
    /// The revision used by the file is not known.
    #[error("unknown revision: {0}")]
    UnknownRevision(u8),
    /// The event contains a statechange that we don't know about.
    #[error("unknown statechange event: {0}")]
    UnknownStateChange(u8),
    /// The given ZIP archive is invalid.
    #[error("invalid archive: {0}")]
    InvalidZip(#[from] zip::result::ZipError),
}

/// A type indicating the parse result.
pub type ParseResult<T> = Result<T, ParseError>;

/// Parse the header of an evtc file.
///
/// It is expected that the file cursor is at the very first byte of the file.
///
/// * `input` - Input stream.
pub fn parse_header<R: Read>(mut input: R) -> ParseResult<Header> {
    // Make sure the magic number matches
    let mut magic_number = [0; 4];
    input.read_exact(&mut magic_number)?;
    if &magic_number != b"EVTC" {
        return Err(ParseError::MalformedHeader);
    }

    // Read arcdps build date.
    let mut arcdps_build = vec![0; 8];
    input.read_exact(&mut arcdps_build)?;
    let build_string = String::from_utf8(arcdps_build)?;

    // Read revision byte
    let mut revision = [0];
    input.read_exact(&mut revision)?;
    let revision = revision[0];

    // Read combat id.
    let combat_id = input.read_u16::<LittleEndian>()?;

    // Read zero delimiter.
    let mut zero = [0];
    input.read_exact(&mut zero)?;
    if zero != [0] {
        return Err(ParseError::MalformedHeader);
    }

    // Read agent count.
    let agent_count = input.read_u32::<LittleEndian>()?;

    Ok(Header {
        arcdps_build: build_string,
        combat_id,
        agent_count,
        revision,
    })
}

/// Parse the agent array.
///
/// This function expects the cursor to be right at the first byte of the agent
/// array.
///
/// * `input` - Input stream.
/// * `count` - Number of agents (found in the header).
pub fn parse_agents<R: Read>(mut input: R, count: u32) -> ParseResult<Vec<Agent>> {
    let mut result = Vec::with_capacity(count as usize);
    for _ in 0..count {
        result.push(parse_agent(&mut input)?);
    }
    Ok(result)
}

/// Parse a single agent.
///
/// * `input` - Input stream.
pub fn parse_agent<R: Read>(mut input: R) -> ParseResult<Agent> {
    let addr = input.read_u64::<LittleEndian>()?;
    let prof = input.read_u32::<LittleEndian>()?;
    let is_elite = input.read_u32::<LittleEndian>()?;
    let toughness = input.read_i16::<LittleEndian>()?;
    let concentration = input.read_i16::<LittleEndian>()?;
    let healing = input.read_i16::<LittleEndian>()?;
    // First padding.
    input.read_i16::<LittleEndian>()?;
    let condition = input.read_i16::<LittleEndian>()?;
    // Second padding.
    input.read_i16::<LittleEndian>()?;
    let mut name = [0; 64];
    input.read_exact(&mut name)?;

    // The C structure has additional 4 bytes of padding, so that the total size
    // of the struct is at 96 bytes.
    // So far, we've only read 92 bytes, so we need to skip 4 more bytes.
    let mut skip = [0; 4];
    input.read_exact(&mut skip)?;

    Ok(Agent {
        addr,
        prof,
        is_elite,
        toughness,
        concentration,
        healing,
        condition,
        name,
    })
}

/// Parse the skill array.
///
/// * `input` - Input stream.
/// * `count` - Number of skills to parse.
pub fn parse_skills<R: Read>(mut input: R, count: u32) -> ParseResult<Vec<Skill>> {
    let mut result = Vec::with_capacity(count as usize);
    for _ in 0..count {
        result.push(parse_skill(&mut input)?);
    }
    Ok(result)
}

/// Parse a single skill.
///
/// * `input` - Input stream.
pub fn parse_skill<R: Read>(mut input: R) -> ParseResult<Skill> {
    let id = input.read_i32::<LittleEndian>()?;
    let mut name = [0; 64];
    input.read_exact(&mut name)?;
    Ok(Skill { id, name })
}

/// Parse all combat events.
///
/// * `input` - Input stream.
/// * `parser` - The parse function to use.
///
/// The `parser` should be one of [`parse_event_rev0`][parse_event_rev0] or
/// [`parse_event_rev1`][parse_event_rev1], depending on the revision of the file you are dealing
/// with. Note that you might have to pass them as a closure, otherwise the type conversion might
/// not succeed:
///
/// ```no_run
/// # use evtclib::raw::parser::{parse_events, parse_event_rev0};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::fs::File;
/// let file = File::open("the-log.evtc")?;
/// // other parsing here
/// let events = parse_events(file, |i| parse_event_rev0(i))?;
/// # Ok(())
/// # }
/// ```
///
/// If you use one of the higher-level functions, such as [`parse_file`][parse_file] or
/// [`finish_parsing`][finish_parsing], you do not have to concern yourself with that detail.
pub fn parse_events<R: Read>(
    mut input: R,
    parser: fn(&mut R) -> ParseResult<CbtEvent>,
) -> ParseResult<Vec<CbtEvent>> {
    let mut result = Vec::new();
    loop {
        let event = parser(&mut input);
        match event {
            Ok(x) => result.push(x),
            Err(ParseError::UnknownStateChange(_)) => {
                // Ignore unknown statechanges, as advised by arcdps.
            }
            Err(ParseError::Io(ref e)) if e.kind() == ErrorKind::UnexpectedEof => {
                return Ok(result)
            }
            Err(e) => return Err(e),
        }
    }
}

/// Parse a single combat event.
///
/// This works for old combat events, i.e. files with revision == 0.
///
/// * `input` - Input stream.
pub fn parse_event_rev0<R: Read>(mut input: R) -> ParseResult<CbtEvent> {
    let time = input.read_u64::<LittleEndian>()?;
    let src_agent = input.read_u64::<LE>()?;
    let dst_agent = input.read_u64::<LE>()?;
    let value = input.read_i32::<LE>()?;
    let buff_dmg = input.read_i32::<LE>()?;
    let overstack_value = input.read_u16::<LE>()? as u32;
    let skillid = input.read_u16::<LE>()? as u32;
    let src_instid = input.read_u16::<LE>()?;
    let dst_instid = input.read_u16::<LE>()?;
    let src_master_instid = input.read_u16::<LE>()?;

    // We can skip 9 bytes of internal tracking garbage.
    let mut skip = [0; 9];
    input.read_exact(&mut skip)?;

    let iff = IFF::from_u8(input.read_u8()?).unwrap_or(IFF::None);
    let buff = input.read_u8()?;
    let result = CbtResult::from_u8(input.read_u8()?).unwrap_or(CbtResult::None);
    let is_activation = CbtActivation::from_u8(input.read_u8()?).unwrap_or(CbtActivation::None);
    let is_buffremove = CbtBuffRemove::from_u8(input.read_u8()?).unwrap_or(CbtBuffRemove::None);
    let is_ninety = input.read_u8()? != 0;
    let is_fifty = input.read_u8()? != 0;
    let is_moving = input.read_u8()? != 0;
    let statechange = input.read_u8()?;
    let is_statechange =
        CbtStateChange::from_u8(statechange).ok_or(ParseError::UnknownStateChange(statechange));
    let is_flanking = input.read_u8()? != 0;
    let is_shields = input.read_u8()? != 0;

    // Two more bytes of internal tracking garbage.
    input.read_u16::<LE>()?;

    Ok(CbtEvent {
        time,
        src_agent,
        dst_agent,
        value,
        buff_dmg,
        overstack_value,
        skillid,
        src_instid,
        dst_instid,
        src_master_instid,
        dst_master_instid: 0,
        iff,
        buff,
        result,
        is_activation,
        is_buffremove,
        is_ninety,
        is_fifty,
        is_moving,
        is_statechange: is_statechange?,
        is_flanking,
        is_shields,
        is_offcycle: false,
    })
}

/// Parse a single combat event.
///
/// This works for new combat events, i.e. files with revision == 1.
///
/// * `input` - Input stream.
pub fn parse_event_rev1<R: Read>(mut input: R) -> ParseResult<CbtEvent> {
    let time = input.read_u64::<LittleEndian>()?;
    let src_agent = input.read_u64::<LE>()?;
    let dst_agent = input.read_u64::<LE>()?;
    let value = input.read_i32::<LE>()?;
    let buff_dmg = input.read_i32::<LE>()?;
    let overstack_value = input.read_u32::<LE>()?;
    let skillid = input.read_u32::<LE>()?;
    let src_instid = input.read_u16::<LE>()?;
    let dst_instid = input.read_u16::<LE>()?;
    let src_master_instid = input.read_u16::<LE>()?;
    let dst_master_instid = input.read_u16::<LE>()?;

    let iff = IFF::from_u8(input.read_u8()?).unwrap_or(IFF::None);
    let buff = input.read_u8()?;
    let result = CbtResult::from_u8(input.read_u8()?).unwrap_or(CbtResult::None);
    let is_activation = CbtActivation::from_u8(input.read_u8()?).unwrap_or(CbtActivation::None);
    let is_buffremove = CbtBuffRemove::from_u8(input.read_u8()?).unwrap_or(CbtBuffRemove::None);
    let is_ninety = input.read_u8()? != 0;
    let is_fifty = input.read_u8()? != 0;
    let is_moving = input.read_u8()? != 0;
    let statechange = input.read_u8()?;
    let is_statechange =
        CbtStateChange::from_u8(statechange).ok_or(ParseError::UnknownStateChange(statechange));
    let is_flanking = input.read_u8()? != 0;
    let is_shields = input.read_u8()? != 0;
    let is_offcycle = input.read_u8()? != 0;

    // Four more bytes of internal tracking garbage.
    input.read_u32::<LE>()?;

    Ok(CbtEvent {
        time,
        src_agent,
        dst_agent,
        value,
        buff_dmg,
        overstack_value,
        skillid,
        src_instid,
        dst_instid,
        src_master_instid,
        dst_master_instid,
        iff,
        buff,
        result,
        is_activation,
        is_buffremove,
        is_ninety,
        is_fifty,
        is_moving,
        is_statechange: is_statechange?,
        is_flanking,
        is_shields,
        is_offcycle,
    })
}

/// Parse a partial EVTC file.
///
/// * `input` - Input stream.
pub fn parse_partial_file<R: Read>(mut input: R) -> ParseResult<PartialEvtc> {
    let header = parse_header(&mut input)?;
    let agents = parse_agents(&mut input, header.agent_count)?;
    let skill_count = input.read_u32::<LittleEndian>()?;
    let skills = parse_skills(input, skill_count)?;

    Ok(PartialEvtc {
        header,
        skill_count,
        agents,
        skills,
    })
}

/// Finish a partial EVTC by reading the events.
///
/// * `partial` - The partial EVTC.
/// * `input` - The input stream.
#[allow(clippy::redundant_closure)]
pub fn finish_parsing<R: Read>(partial: PartialEvtc, input: R) -> ParseResult<Evtc> {
    // The following closures seem redundant, but they are needed to convice Rust that we can
    // actually use parse_event_rev* here. That is because we require a lifetime bound of
    //   for<'r> fn(&'r mut R) -> ParseResult
    // which we cannot get by just plugging in parse_event_rev*.
    let events = match partial.header.revision {
        0 => parse_events(input, |r| parse_event_rev0(r))?,
        1 => parse_events(input, |r| parse_event_rev1(r))?,
        x => return Err(ParseError::UnknownRevision(x)),
    };

    Ok(Evtc {
        header: partial.header,
        skill_count: partial.skill_count,
        agents: partial.agents,
        skills: partial.skills,
        events,
    })
}

/// Parse a complete EVTC file.
///
/// * `input` - Input stream.
pub fn parse_file<R: Read>(mut input: R) -> ParseResult<Evtc> {
    let partial = parse_partial_file(&mut input)?;
    finish_parsing(partial, input)
}
