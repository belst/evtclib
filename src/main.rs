extern crate byteorder;
extern crate evtclib;
use byteorder::{ReadBytesExt, BE, LE};
use std::fs::File;

use std::io::{Seek, SeekFrom};

pub fn main() -> Result<(), evtclib::raw::parser::ParseError> {
    println!("Hello World!");
    let mut f = File::open("material/Samarog.evtc")?;

    let result = evtclib::raw::parse_file(&mut f)?;

    Ok(())
}
