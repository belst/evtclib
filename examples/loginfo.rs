use std::{env, process};

use evtclib::Compression;

fn main() {
    let name = if let Some(name) = env::args().nth(1) {
        name
    } else {
        eprintln!("Expected a file name");
        process::exit(1);
    };

    let compression = if name.ends_with(".zip") || name.ends_with(".zevtc") {
        Compression::Zip
    } else {
        Compression::None
    };

    let log = evtclib::process_file(&name, compression).unwrap();

    println!("Encounter: {:?}", log.encounter());
    println!("Was CM? {}", log.is_cm());
    println!("Players:");
    for player in log.players() {
        println!(
            "{} {} {}",
            player.subgroup(),
            player.account_name(),
            player.character_name()
        );
    }

    println!("Number of recorded events: {}", log.events().len());
}
