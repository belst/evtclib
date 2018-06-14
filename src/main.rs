extern crate byteorder;
extern crate evtclib;
use byteorder::{ReadBytesExt, BE, LE};
use std::env;
use std::fs::File;

use std::collections::HashMap;
use std::io::BufReader;

// My addr: 5480786193115521456
// My instid: 624
// Samarog: 18003064748228432752

pub fn main() -> Result<(), evtclib::raw::parser::ParseError> {
    println!("Hello World!");
    let mut f = BufReader::new(File::open(env::args().skip(1).next().unwrap())?);

    let result = evtclib::raw::parse_zip(&mut f)?;
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
    let mut boonq =
        evtclib::statistics::boon::BoonQueue::new(5, evtclib::statistics::boon::BoonType::Duration);
    let mut last_time = 0;
    let mut uptime = 0;
    for event in &result.events {
        let shiny = if let Some(c) = evtclib::Event::from_raw(event) {
            c
        } else {
            println!("Failed: {:#?}", event);
            failed += 1;
            continue;
        };
        uptime += boonq.current_stacks() as u64 * (event.time - last_time);
        boonq.simulate(event.time - last_time);
        match shiny.kind {
            EventKind::Physical {
                source_agent_addr: src,
                damage: dmg,
                ..
            } if src == 17963907008649399712 =>
            {
                count += 1;
                damage += dmg as u64;
            }
            EventKind::ConditionTick {
                source_agent_addr: src,
                damage: dmg,
                ..
            } if src == 17963907008649399712 =>
            {
                count += 1;
                damage += dmg as u64;
            }

            EventKind::BuffApplication {
                buff_id: 1187,
                destination_agent_addr: 5480786193115521456,
                duration,
                ..
            } => {
                //println!("{:10} I got might for {}!", shiny.time, duration);
                boonq.add_stack(duration as u64);
            }

            EventKind::BuffRemove {
                source_agent_addr: 5480786193115521456,
                buff_id: 736,
                removal,
                total_duration,
                longest_stack,
                ..
            } => {
                println!(
                    "{:10} Buffremove, removal {:?} dur {:?} longest {:?}",
                    shiny.time, removal, total_duration, longest_stack
                );
            }

            _ => (),
        }
        last_time = event.time;
    }
    println!("Count: {}, Damage: {}", count, damage);
    println!("Failed events: {}", failed);

    let processed = evtclib::process(&result).expect("Failed processing");
    //println!("Me: {:#?}", processed.agent_by_addr(5480786193115521456));
    println!("Players: {:#?}", processed.players().collect::<Vec<_>>());
    let stats = evtclib::statistics::calculate(&processed).unwrap();
    //println!("{:#?}", stats);
    let my_addr = me(&processed);
    let mine = stats.agent_stats.get(&my_addr).expect("My stats not found");

    let my_damage = stats.damage_log.damage(|m| m.source == my_addr && processed.is_boss(m.target));

    let combat_time = (mine.exit_combat - mine.enter_combat) as f32/ 1000.;
    println!("Damages: {:?}", stats.damage_log);
    println!("Combat time: {} ({} till {})", combat_time, mine.enter_combat, mine.exit_combat);
    println!("My boss dps: {:?}", my_damage.0 as f32 / combat_time);

    for boon in evtclib::statistics::gamedata::BOONS {
        let avg = mine
            .boon_log
            .average_stacks(mine.enter_combat, mine.exit_combat, boon.0);
        println!("{}: {}", boon.1, avg);
    }

    //println!("NPCs: {:#?}", processed.npcs().collect::<Vec<_>>());
    println!("Bosses: {:#?}", processed.boss_agents());

    Ok(())
}

fn me(log: &evtclib::Log) -> u64 {
    for agent in log.players() {
        if let evtclib::AgentName::Player { account_name, .. } = agent.name() {
            if account_name == ":Dunje.4863" {
                return *agent.addr();
            }
        }
    }
    0
}
