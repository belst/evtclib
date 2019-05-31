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

use byteorder::{LittleEndian, ReadBytesExt, LE};
use num_traits::FromPrimitive;
use std::io::{self, ErrorKind, Read};

use super::*;

/// EVTC file header.
#[derive(Clone, Debug, PartialEq, Eq)]
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
#[derive(Clone, Debug)]
pub struct Evtc {
    /// The file header values
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
/// This can speed up parsing for applications which can work with the header.
#[derive(Clone, Debug)]
pub struct PartialEvtc {
    pub header: Header,
    pub skill_count: u32,
    pub agents: Vec<Agent>,
    pub skills: Vec<Skill>,
}

quick_error! {
    #[derive(Debug)]
    pub enum ParseError {
        Io(err: io::Error) {
            from()
            description("io error")
            display("I/O error: {}", err)
            cause(err)
        }
        Utf8Error(err: ::std::string::FromUtf8Error) {
            from()
            description("utf8 decoding error")
            display("UTF-8 decoding error: {}", err)
            cause(err)
        }
        InvalidData {
            from(::std::option::NoneError)
            description("invalid data")
        }
        MalformedHeader {
            description("malformed header")
        }
        UnknownRevision(rev: u8) {
            description("unknown revision")
            display("revision number not known: {}", rev)
        }
        InvalidZip(err: ::zip::result::ZipError) {
            from()
            description("zip error")
            display("Archive error: {}", err)
            cause(err)
        }
    }
}

/// A type indicating the parse result.
pub type ParseResult<T> = Result<T, ParseError>;

/// Parse the header of an evtc file.
///
/// It is expected that the file cursor is at the very first byte of the file.
///
/// * `input` - Input stream.
pub fn parse_header<T: Read>(input: &mut T) -> ParseResult<Header> {
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
pub fn parse_agents<T: Read>(input: &mut T, count: u32) -> ParseResult<Vec<Agent>> {
    let mut result = Vec::with_capacity(count as usize);
    for _ in 0..count {
        result.push(parse_agent(input)?);
    }
    Ok(result)
}

/// Parse a single agent.
///
/// * `input` - Input stream.
pub fn parse_agent<T: Read>(input: &mut T) -> ParseResult<Agent> {
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
pub fn parse_skills<T: Read>(input: &mut T, count: u32) -> ParseResult<Vec<Skill>> {
    let mut result = Vec::with_capacity(count as usize);
    for _ in 0..count {
        result.push(parse_skill(input)?);
    }
    Ok(result)
}

/// Parse a single skill.
///
/// * `input` - Input stream.
pub fn parse_skill<T: Read>(input: &mut T) -> ParseResult<Skill> {
    let id = input.read_i32::<LittleEndian>()?;
    let mut name = [0; 64];
    input.read_exact(&mut name)?;
    Ok(Skill { id, name })
}

/// Parse all combat events.
///
/// * `input` - Input stream.
/// * `parser` - The parse function to use.
pub fn parse_events<T: Read>(input: &mut T, parser: fn(&mut T) -> ParseResult<CbtEvent>) -> ParseResult<Vec<CbtEvent>> {
    let mut result = Vec::new();
    loop {
        let event = parser(input);
        match event {
            Ok(x) => result.push(x),
            Err(ParseError::Io(ref e)) if e.kind() == ErrorKind::UnexpectedEof => return Ok(result),
            Err(e) => return Err(e),
        }
    }
}

/// Parse a single combat event.
///
/// This works for old combat events, i.e. files with revision == 0.
///
/// * `input` - Input stream.
pub fn parse_event_rev0<T: Read>(input: &mut T) -> ParseResult<CbtEvent> {
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
    let is_statechange = CbtStateChange::from_u8(input.read_u8()?)?;
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
        is_statechange,
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
pub fn parse_event_rev1<T: Read>(input: &mut T) -> ParseResult<CbtEvent> {
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
    let is_statechange = CbtStateChange::from_u8(input.read_u8()?)?;
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
        is_statechange,
        is_flanking,
        is_shields,
        is_offcycle,
    })
}



/// Parse a partial EVTC file.
///
/// * `input` - Input stream.
pub fn parse_partial_file<T: Read>(input: &mut T) -> ParseResult<PartialEvtc> {
    let header = parse_header(input)?;
    let agents = parse_agents(input, header.agent_count)?;
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
pub fn finish_parsing<T: Read>(partial: PartialEvtc, input: &mut T) -> ParseResult<Evtc> {
    let events = match partial.header.revision {
        0 => parse_events(input, parse_event_rev0)?,
        1 => parse_events(input, parse_event_rev1)?,
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
pub fn parse_file<T: Read>(input: &mut T) -> ParseResult<Evtc> {
    let partial = parse_partial_file(input)?;
    finish_parsing(partial, input)
}
