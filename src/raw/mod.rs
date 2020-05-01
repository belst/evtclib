//! This module defines raw types that correspond 1:1 to the C types used in
//! [arcdps](https://www.deltaconnected.com/arcdps/evtc/README.txt).
//!
//! It is not advised to use those types and functions, as dealing with all the
//! low-level details can be quite tedious. Instead, use the higher-level
//! functions whenever possible.
mod types;

use zip::ZipArchive;

pub use self::types::{
    Agent, CbtActivation, CbtBuffRemove, CbtCustomSkill, CbtEvent, CbtResult, CbtStateChange,
    Language, Skill, IFF,
};

pub mod parser;

pub use self::parser::{parse_file, Evtc, ParseError, ParseResult};

use std::ffi::CStr;
use std::io::{BufReader, Read, Seek};

/// Parse a complete log that was compressed as a zip file.
pub fn parse_zip<R: Read + Seek>(input: R) -> ParseResult<Evtc> {
    let mut archive = ZipArchive::new(input)?;
    let mut file = BufReader::new(archive.by_index(0)?);
    parse_file(&mut file)
}

/// Return a [`CStr`][CStr] up to the first nul byte.
///
/// This is different to [`CStr::from_bytes_with_nul`][CStr::from_bytes_with_nul] in that it stops
/// at the first nul byte instead of raising an error.
///
/// If the slice does not end with a nul byte, this function returns `None`.
pub fn cstr_up_to_nul(bytes: &[u8]) -> Option<&CStr> {
    let index = bytes.iter().position(|c| *c == 0)?;
    CStr::from_bytes_with_nul(&bytes[..index + 1]).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cstr_up_to_nul_terminated() {
        let bytes = b"foo\0";
        let cstr = cstr_up_to_nul(bytes).unwrap();
        assert_eq!(cstr.to_bytes(), b"foo");
    }

    #[test]
    fn test_cstr_up_to_nul_multiple() {
        let bytes = b"foo\0\0\0";
        let cstr = cstr_up_to_nul(bytes).unwrap();
        assert_eq!(cstr.to_bytes(), b"foo");

        let bytes = b"foo\0bar\0\0";
        let cstr = cstr_up_to_nul(bytes).unwrap();
        assert_eq!(cstr.to_bytes(), b"foo");
    }

    #[test]
    fn test_cstr_up_to_nul_unterminated() {
        let bytes = b"foo";
        let cstr = cstr_up_to_nul(bytes);
        assert!(cstr.is_none());
    }
}
