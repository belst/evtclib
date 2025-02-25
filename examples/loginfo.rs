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
    if let Some(analyzer) = log.analyzer() {
        println!("Fight outcome: {:?}", analyzer.outcome());
    } else {
        println!("No analyzer available for this fight.");
    }
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

    for error in log.errors() {
        println!("Error in log: {}", error);
    }
}
