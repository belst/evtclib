extern crate byteorder;
extern crate evtclib;
use byteorder::{ReadBytesExt, BE, LE};
use std::fs::File;

use std::io::BufReader;
use std::collections::HashMap;

// My addr: 5480786193115521456
// My instid: 624
// Samarog: 18003064748228432752

pub fn main() -> Result<(), evtclib::raw::parser::ParseError> {
    println!("Hello World!");
    let mut f = BufReader::new(File::open("material/Samarog.evtc")?);

    let result = evtclib::raw::parse_file(&mut f)?;
/*
    for agent in result.agents.iter().filter(|a| a.is_player()) {
        println!("Agent: {:?}", agent);
    }

    let mut damage: HashMap<u16, u64> = HashMap::new();
    let mut count = 0;
    for event in result.events.iter() {
        if event.is_statechange == evtclib::raw::CbtStateChange::None {
            if (event.dst_agent != 0 && event.dst_instid == 0) || (event.dst_instid != 0 && event.dst_agent == 0) {
                println!("{:#?}", event);
            }
        }
        let must_take = if event.src_instid == 624 && event.skillid == 19426 && (event.value == 287 || event.buff_dmg == 287) {
            println!("Event in question: {:#?}", event);
            true
        } else { false };
        let mut taken = false;
        if event.src_instid == 624 || event.src_master_instid == 624 {
            //for target in result.agents.iter().filter(|a| a.is_character()) {
                if event.iff == evtclib::raw::IFF::Foe && event.dst_agent != 0 {
                    if event.is_statechange == evtclib::raw::CbtStateChange::None && event.is_buffremove == evtclib::raw::CbtBuffRemove::None {
                        let dmg = if event.buff == 1 && event.buff_dmg != 0 {
                            event.buff_dmg
                        } else if event.buff == 0 && event.value != 0 {
                            event.value
                        } else if [5, 6, 7].contains(&(event.result as u32)) { event.value }
                        else {
                            if must_take && !taken {
                                panic!("Failing event: {:#?}", event);
                            };
                            continue;
                        };
                        println!("{} {} {}", event.src_agent, event.skillid, dmg);
                        *damage.entry(event.skillid).or_insert(0) += dmg as u64;
                        count += 1;
                        taken = true;
                    }
                }
            //}
        }
        if must_take && !taken {
            panic!("Failing event: {:#?}", event);
        }
    }
    println!("Damage: {:#?}, Total: {}, Count: {}", damage, damage.values().sum::<u64>(), count);
    println!("Event count: {}", result.events.len());
    println!("Events for me: {}", result.events.iter().filter(|e| e.src_instid == 624).count());
*/
    //let processed = evtclib::process(&result);
    use evtclib::EventKind;
    let mut count = 0;
    let mut damage = 0;
    let mut failed = 0;
    for event in &result.events {
        let shiny = if let Some(c) = evtclib::Event::from_raw(event) {
            c
        } else {
            println!("Failed: {:#?}", event);
            failed += 1;
            continue
        };
        match &shiny.kind {
            &EventKind::Physical { source_agent_addr: src, damage: dmg, .. } if src == 5480786193115521456 => { count += 1; damage += dmg as u64; },
            &EventKind::ConditionTick { source_agent_addr: src, damage: dmg, .. } if src == 5480786193115521456 => { count += 1; damage += dmg as u64; },
            _ => (),
        }
    }
    println!("Count: {}, Damage: {}", count, damage);
    println!("Failed events: {}", failed);

    Ok(())
}
