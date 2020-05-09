evtclib
=======

[![Latest Version](https://img.shields.io/crates/v/evtclib.svg)](https://crates.io/crates/evtclib)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/evtclib)
[![License](https://img.shields.io/crates/l/evtclib.svg)](https://opensource.org/licenses/MIT)

`evtclib` is a Rust library that allows you to parse `.evtc` files, as
generated by the [arcdps addon](https://www.deltaconnected.com/arcdps/) for the
Guild Wars 2 video game.

Features:

* A low-level parsing interface with structs mimicking the arcdps C structs.
* A high-level interface, intended for consumption within Rust applications.
* Support for reading zipped evtc files (`.evtc.zip` or `.zevtc`).
* Backwards compatible for older revisions of the evtc format.

`evtclib` is currently in beta-stage. Not all evtc events are supported, and
the API is not yet set in stone.

Example
-------

```rust
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a log
    let log = evtclib::process_file("Skorvald/20200421-183243.evtc")?;
    // Do work on the log
    for player in log.players() {
        println!("Player {} participated!", player.account_name());
    }
    Ok(())
}
```

License
-------

This project is licensed under the MIT license (`LICENSE` or
https://opensource.org/licenses/MIT).
