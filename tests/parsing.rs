//! Tests that test parsing a complete log file and comparing some basic information.

use std::fs::File;
use std::io::BufReader;

use evtclib::{EliteSpec::*, Encounter, GameMode::*, Profession::*};

macro_rules! test {
    (name: $name:ident, log: $log:literal, boss: $boss:expr, mode: $mode:expr, players: $players:expr,) => {
        #[test]
        fn $name() {
            let mut file = BufReader::new(File::open(format!("tests/{}", $log)).unwrap());
            let log = evtclib::raw::parse_zip(&mut file).expect("parsing zip failed");
            let log = evtclib::process(&log).expect("processing log failed");
            assert_eq!(log.encounter(), Some($boss));
            assert!(!log.is_generic());
            assert_eq!(log.game_mode(), Some($mode));

            let players = $players;

            assert_eq!(log.players().count(), players.len());
            for (subgroup, account_name, character_name, profession, elite_spec) in players {
                let player = log
                    .players()
                    .find(|p| &p.player().account_name() == account_name)
                    .expect(&format!("did not find player {}", account_name))
                    .player();
                assert_eq!(player.subgroup(), *subgroup);
                assert_eq!(player.character_name(), *character_name);
                assert_eq!(player.profession(), *profession);
                assert_eq!(player.elite(), *elite_spec);
            }

            // We don't want to assert the correct outcome here (yet?), but at least ensure we have
            // analyzer's ready that produce some outcome.
            assert!(log.analyzer().is_some());
            assert!(log.analyzer().unwrap().outcome().is_some());
        }
    };
}

// Wing 1 tests

test! {
    name: parse_vale_guardian,
    log: "logs/vg-20200421.zevtc",
    boss: Encounter::ValeGuardian,
    mode: Raid,
    players: &[
        (4, ":AliceWindwalker.6238", "Fafnarin Sunseeker", Warrior, Some(Berserker)),
        (4, ":Gellalli.6580", "Germi N", Revenant, Some(Renegade)),
        (4, ":HqrKATzPOLSKI.7896", "Haqush", Guardian, Some(Dragonhunter)),
        (4, ":Sora.8732", "Riríchíyo", Guardian, Some(Firebrand)),
        (4, ":justice.8392", "Ipman D", Guardian, Some(Dragonhunter)),
        (5, ":Diabound.5473", "Yaru Lanayru", Thief, Some(Deadeye)),
        (5, ":Dunje.4863", "Emma Hydes", Elementalist, Some(Weaver)),
        (5, ":Straimer.1093", "Revenge On The Meta", Revenant, Some(Renegade)),
        (5, ":neko.9741", "Mordrem Cat", Ranger, Some(Druid)),
        (5, ":xyoz.6710", "Xaphwen", Mesmer, Some(Chronomancer)),
    ],
}

test! {
    name: parse_gorseval,
    log: "logs/gorseval-20200421.zevtc",
    boss: Encounter::Gorseval,
    mode: Raid,
    players: &[
        (4, ":AliceWindwalker.6238", "Fafnarin Sunseeker", Warrior, Some(Berserker)),
        (4, ":Gellalli.6580", "Germi N", Revenant, Some(Renegade)),
        (4, ":HqrKATzPOLSKI.7896", "Haqush", Guardian, Some(Dragonhunter)),
        (4, ":Sora.8732", "Riríchíyo", Guardian, Some(Firebrand)),
        (4, ":justice.8392", "Ipman D", Guardian, Some(Dragonhunter)),
        (5, ":Diabound.5473", "Yaru Lanayru", Thief, Some(Deadeye)),
        (5, ":Dunje.4863", "Emma Hydes", Elementalist, Some(Weaver)),
        (5, ":Straimer.1093", "I Want Smite Back", Guardian, Some(Dragonhunter)),
        (5, ":neko.9741", "Mordrem Cat", Ranger, Some(Druid)),
        (5, ":xyoz.6710", "Xaphwen", Mesmer, Some(Chronomancer)),
    ],
}

test! {
    name: parse_sabetha,
    log: "logs/sabetha-20200421.zevtc",
    boss: Encounter::Sabetha,
    mode: Raid,
    players: &[
        (4, ":AliceWindwalker.6238", "Fafnarin Sunseeker", Warrior, Some(Berserker)),
        (4, ":Gellalli.6580", "Germi N", Revenant, Some(Renegade)),
        (4, ":HqrKATzPOLSKI.7896", "Haqush", Guardian, Some(Firebrand)),
        (4, ":Sora.8732", "Riríchíyo", Guardian, Some(Firebrand)),
        (4, ":justice.8392", "Ipman D", Guardian, Some(Dragonhunter)),
        (5, ":Diabound.5473", "Yaru Lanayru", Thief, Some(Deadeye)),
        (5, ":Dunje.4863", "Thank You Exorcist", Necromancer, Some(Reaper)),
        (5, ":Straimer.1093", "Feel My Epidemic", Necromancer, Some(Reaper)),
        (5, ":neko.9741", "Mordrem Cat", Ranger, Some(Druid)),
        (5, ":xyoz.6710", "Xaphwen", Mesmer, Some(Chronomancer)),
    ],
}

// Wing 2 tests

test! {
    name: parse_slothasor,
    log: "logs/slothasor-20200420.zevtc",
    boss: Encounter::Slothasor,
    mode: Raid,
    players: &[
        (2, ":Basafrass.4138", "Miss Mary J", Guardian, Some(Dragonhunter)),
        (2, ":Gellalli.6580", "Gellalli V", Guardian, Some(Dragonhunter)),
        (2, ":Straimer.1093", "I Want Smite Back", Guardian, Some(Dragonhunter)),
        (2, ":safarissPL.2476", "Archangel Of Fury", Guardian, Some(Firebrand)),
        (2, ":surbiff.4912", "Nellie Mellie", Elementalist, Some(Tempest)),
        (3, ":Archmage.3405", "Archmage The Sage", Elementalist, Some(Weaver)),
        (3, ":Dunje.4863", "Pallida Howhite", Warrior, None),
        (3, ":MooRecords.8096", "Mc Me", Mesmer, Some(Chronomancer)),
        (3, ":neko.9741", "Mordrem Cat", Ranger, Some(Druid)),
        (3, ":xyoz.6710", "Takathy", Revenant, Some(Renegade)),
    ],
}

test! {
    name: parse_bandit_trio,
    log: "logs/trio-20210501.zevtc",
    boss: Encounter::BanditTrio,
    mode: Raid,
    players: &[
        (2, ":Dunje.4863", "Pallida Howhite", Warrior, Some(Berserker)),
        (2, ":Straimer.1093", "Deepfreeze Myself", Elementalist, Some(Weaver)),
        (2, ":Alninio.5964", "Daedhur", Necromancer, Some(Reaper)),
        (2, ":Luigi.8076", "Phantasmal Ficus", Mesmer, Some(Chronomancer)),
        (2, ":Subi.8014", "Juvenile Subi", Ranger, Some(Druid)),
        (3, ":xyoz.6710", "Xaphy", Engineer, Some(Holosmith)),
        (3, ":Ashe.2473", "Dust Of Stance", Revenant, Some(Renegade)),
        (3, ":neko.9741", "Cat Of Jormag", Engineer, Some(Holosmith)),
        (3, ":Snake.9125", "Matis Dorei", Guardian, Some(Firebrand)),
        (3, ":eupneo.1036", "Tormented Flame", Revenant, Some(Renegade)),
    ],
}

test! {
    name: parse_matthias,
    log: "logs/matthias-20200421.zevtc",
    boss: Encounter::Matthias,
    mode: Raid,
    players: &[
        (2, ":Basafrass.4138", "Miss Mary J", Guardian, Some(Dragonhunter)),
        (2, ":Gellalli.6580", "Germi N", Revenant, Some(Renegade)),
        (2, ":Straimer.1093", "Deepfreeze Myself", Elementalist, Some(Tempest)),
        (2, ":safarissPL.2476", "Archangel Of Fury", Guardian, Some(Firebrand)),
        (2, ":surbiff.4912", "The Biff", Guardian, Some(Firebrand)),
        (3, ":Archmage.3405", "Archmage The Sage", Elementalist, Some(Weaver)),
        (3, ":Dunje.4863", "Zraraelia Vey", Necromancer, Some(Scourge)),
        (3, ":Speeaaakmaan.8974", "Gravîty Well", Mesmer, Some(Mirage)),
        (3, ":neko.9741", "Mordrem Cat", Ranger, Some(Druid)),
        (3, ":xyoz.6710", "Xaphwen", Mesmer, Some(Chronomancer)),
    ],
}

// Wing 3 tests

test! {
    name: parse_keep_construct,
    log: "logs/kc-20200426.zevtc",
    boss: Encounter::KeepConstruct,
    mode: Raid,
    players: &[
        (3, ":Bomaga.2106", "Krupniczek", Guardian, Some(Dragonhunter)),
        (3, ":Buddy Christ.1758", "Block Buddy", Guardian, Some(Dragonhunter)),
        (3, ":LucieMillerx.8650", "Ànemóne", Ranger, Some(Soulbeast)),
        (3, ":Straimer.1093", "Revenge On The Meta", Revenant, Some(Renegade)),
        (3, ":neko.9741", "Star Guardian Neko", Guardian, Some(Firebrand)),
        (4, ":DavDav.4765", "Séraphie Mangor", Guardian, Some(Dragonhunter)),
        (4, ":Dunje.4863", "Godric Gobbledygook", Mesmer, Some(Chronomancer)),
        (4, ":Gellalli.6580", "Germi X", Ranger, Some(Druid)),
        (4, ":Odila.7842", "Nas Tia", Necromancer, Some(Reaper)),
        (4, ":Shanadodoo.6983", "Blila Blumenkind", Warrior, Some(Berserker)),
    ],
}

test! {
    name: parse_xera,
    log: "logs/xera-20200415.zevtc",
    boss: Encounter::Xera,
    mode: Raid,
    players: &[
        (2, ":Marcoliveira.7526", "Flamed Horns", Guardian, Some(Dragonhunter)),
        (2, ":Marvin.4612", "Necro Rp", Necromancer, Some(Reaper)),
        (2, ":Speeaaakmaan.8974", "I Carry Kits", Engineer, Some(Holosmith)),
        (2, ":neko.9741", "Star Guardian Neko", Guardian, Some(Firebrand)),
        (2, ":nowere.4583", "Feijoca Slayer", Revenant, Some(Renegade)),
        (3, ":Dunje.4863", "Pallida Howhite", Warrior, Some(Berserker)),
        (3, ":Fiamma.3746", "Charrdiano", Guardian, Some(Dragonhunter)),
        (3, ":Gellalli.6580", "Germi X", Ranger, Some(Druid)),
        (3, ":Straimer.1093", "Deepfreeze Myself", Elementalist, Some(Weaver)),
        (3, ":xyoz.6710", "Xaphwen", Mesmer, Some(Chronomancer)),
    ],
}

// Wing 4 tests

test! {
    name: parse_cairn,
    log: "logs/cairn-20200426.zevtc",
    boss: Encounter::Cairn,
    mode: Raid,
    players: &[
        (3, ":Bomaga.2106", "Krupniczek", Guardian, Some(Dragonhunter)),
        (3, ":Buddy Christ.1758", "Block Buddy", Guardian, Some(Firebrand)),
        (3, ":Dunje.4863", "Padme Amidada", Guardian, Some(Firebrand)),
        (3, ":LucieMillerx.8650", "Ànemóne", Ranger, Some(Soulbeast)),
        (3, ":Straimer.1093", "Revenge On The Meta", Revenant, Some(Renegade)),
        (4, ":Gellalli.6580", "Germi X", Ranger, Some(Druid)),
        (4, ":Odila.7842", "Nas Tia", Necromancer, Some(Reaper)),
        (4, ":Redi.8461", "X Ratiopharm X", Guardian, Some(Dragonhunter)),
        (4, ":Shanadodoo.6983", "Blila Blumenkind", Warrior, Some(Berserker)),
        (4, ":neko.9741", "General Nekobi", Mesmer, Some(Chronomancer)),
    ],
}

test! {
    name: parse_mursaat_overseer,
    log: "logs/mo-20200426.zevtc",
    boss: Encounter::MursaatOverseer,
    mode: Raid,
    players: &[
        (3, ":Bomaga.2106", "Krupniczek", Guardian, Some(Dragonhunter)),
        (3, ":Buddy Christ.1758", "Block Buddy", Guardian, Some(Dragonhunter)),
        (3, ":Dunje.4863", "Padme Amidada", Guardian, Some(Firebrand)),
        (3, ":LucieMillerx.8650", "Ànemóne", Ranger, Some(Soulbeast)),
        (3, ":Straimer.1093", "Revenge On The Meta", Revenant, Some(Renegade)),
        (4, ":Gellalli.6580", "Germi X", Ranger, Some(Druid)),
        (4, ":Morality Vi.4512", "Mowality", Necromancer, Some(Scourge)),
        (4, ":Odila.7842", "Nas Tia", Necromancer, Some(Reaper)),
        (4, ":Shanadodoo.6983", "Blila Blumenkind", Warrior, Some(Berserker)),
        (4, ":neko.9741", "General Nekobi", Mesmer, Some(Chronomancer)),
    ],
}

test! {
    name: parse_samarog,
    log: "logs/samarog-20200426.zevtc",
    boss: Encounter::Samarog,
    mode: Raid,
    players: &[
        (3, ":Bomaga.2106", "Krupniczek", Guardian, Some(Dragonhunter)),
        (3, ":Buddy Christ.1758", "Block Buddy", Guardian, Some(Dragonhunter)),
        (3, ":Dunje.4863", "Padme Amidada", Guardian, Some(Firebrand)),
        (3, ":LucieMillerx.8650", "Ànemóne", Ranger, Some(Soulbeast)),
        (3, ":Straimer.1093", "Revenge On The Meta", Revenant, Some(Renegade)),
        (4, ":Gellalli.6580", "Germi X", Ranger, Some(Druid)),
        (4, ":Morality Vi.4512", "Bat San", Guardian, Some(Dragonhunter)),
        (4, ":Odila.7842", "Nas Tia", Necromancer, Some(Reaper)),
        (4, ":Shanadodoo.6983", "Blila Blumenkind", Warrior, Some(Berserker)),
        (4, ":neko.9741", "General Nekobi", Mesmer, Some(Chronomancer)),
    ],
}

test! {
    name: parse_deimos,
    log: "logs/deimos-20200428.zevtc",
    boss: Encounter::Deimos,
    mode: Raid,
    players: &[
        (2, ":CrusaderCody.6935", "Cody Quickfire", Guardian, Some(Firebrand)),
        (2, ":Mrperfect.5213", "Hanna Kowalski", Revenant, Some(Renegade)),
        (2, ":Oliber.5142", "Shoe Of Life", Guardian, Some(Dragonhunter)),
        (2, ":Skapti Bardakson.1750", "Halfdan Coldwalker", Warrior, Some(Berserker)),
        (3, ":DevilsAngel.5734", "Devilsangels", Warrior, Some(Berserker)),
        (3, ":KINGLEO.1862", "Scrubslayerr", Warrior, Some(Berserker)),
        (3, ":Natallya.9845", "Althellya", Mesmer, Some(Chronomancer)),
        (3, ":XaSeRLP.1832", "Anina Landru", Ranger, Some(Druid)),
        (3, ":chinchiyo.6794", "Charles Penguin", Thief, Some(Daredevil)),
        (4, ":Dunje.4863", "Maho Shiina", Revenant, Some(Herald)),
    ],
}

// Wing 5 tests

test! {
    name: parse_desmina,
    log: "logs/desmina-20200425.zevtc",
    boss: Encounter::SoullessHorror,
    mode: Raid,
    players: &[
        (3, ":AliceWindwalker.6238", "Fafnarin Sunseeker", Warrior, Some(Berserker)),
        (3, ":Dunje.4863", "Godric Gobbledygook", Mesmer, Some(Chronomancer)),
        (3, ":Gellalli.6580", "Germi X", Ranger, Some(Druid)),
        (3, ":Jonny.5860", "Razihel Toursad", Revenant, Some(Renegade)),
        (3, ":Straimer.1093", "Feel My Epidemic", Necromancer, Some(Scourge)),
        (4, ":BakedSnail.7063", "Wibbly Also Wobbly", Mesmer, Some(Mirage)),
        (4, ":ShionSan.1637", "Nilvalen Feel", Mesmer, Some(Mirage)),
        (4, ":Tanylyla.6397", "Tany Phalanx", Warrior, Some(Berserker)),
        (4, ":neko.9741", "Syberia Nótt", Elementalist, Some(Tempest)),
        (4, ":xyoz.6710", "Xaphwen", Mesmer, Some(Chronomancer)),
    ],
}

test! {
    name: parse_river,
    log: "logs/river-20210412.zevtc",
    boss: Encounter::RiverOfSouls,
    mode: Raid,
    players: &[
        (1, ":Baragos.2384", "Cicadania", Mesmer, Some(Chronomancer)),
        (1, ":Jupp.4570", "Aldwor", Guardian, Some(Firebrand)),
        (2, ":Dunje.4863", "Pallida Howhite", Warrior, Some(Berserker)),
        (2, ":Taniniver BlindDragon.9503", "Dragon Kills You", Necromancer, Some(Scourge)),
        (2, ":neko.9741", "Mordrem Cat", Ranger, Some(Druid)),
        (2, ":Ricola.5183", "Glühstrumpf", Mesmer, Some(Chronomancer)),
        (2, ":Faboss.2534", "Faboss Sensei", Revenant, Some(Renegade)),
        (3, ":Glahs.2549", "Nala", Ranger, Some(Druid)),
        (3, ":xyoz.6710", "Xaphwen", Mesmer, Some(Chronomancer)),
        (3, ":Straimer.1093", "Deepfreeze Myself", Elementalist, Some(Tempest)),
    ],
}

test! {
    name: parse_broken_king,
    log: "logs/broken-king-20211115.zevtc",
    boss: Encounter::BrokenKing,
    mode: Raid,
    players: &[
        (1, ":Dunje.4863", "Pallida Howhite", Warrior, Some(Berserker)),
        (1, ":Straimer.1093", "I Want Smite Back", Guardian, Some(Dragonhunter)),
        (1, ":Taniniver BlindDragon.9503", "Dragon Kills You", Necromancer, Some(Scourge)),
        (1, ":xyoz.6710", "Xaphwen", Mesmer, Some(Chronomancer)),
        (2, ":Jupp.4570", "Aldwor", Guardian, Some(Firebrand)),
        (2, ":Kiki.9576", "Spooky Kiki", Necromancer, Some(Scourge)),
        (2, ":Rajnesh.4526", "I Rajnesh I", Revenant, Some(Renegade)),
        (2, ":TheMakNoon.5071", "Twosouls M", Mesmer, Some(Chronomancer)),
        (2, ":Timothy.5829", "Annegret On Frenzy", Ranger, Some(Druid)),
        (3, ":neko.9741", "Syberia Nótt", Elementalist, Some(Tempest)),
    ],
}

test! {
    name: parse_eater,
    log: "logs/eater-20211115.zevtc",
    boss: Encounter::EaterOfSouls,
    mode: Raid,
    players: &[
        (1, ":Dunje.4863", "Pallida Howhite", Warrior, Some(Berserker)),
        (1, ":Straimer.1093", "I Want Smite Back", Guardian, Some(Dragonhunter)),
        (1, ":Taniniver BlindDragon.9503", "Dragon Kills You", Necromancer, Some(Scourge)),
        (1, ":Timothy.5829", "Annegret On Frenzy", Ranger, Some(Druid)),
        (1, ":xyoz.6710", "Xaphwen", Mesmer, Some(Chronomancer)),
        (2, ":Jupp.4570", "Aldwor", Guardian, Some(Firebrand)),
        (2, ":Kiki.9576", "Spooky Kiki", Necromancer, Some(Scourge)),
        (2, ":Rajnesh.4526", "I Rajnesh I", Revenant, Some(Renegade)),
        (2, ":TheMakNoon.5071", "Twosouls M", Mesmer, Some(Chronomancer)),
        (2, ":neko.9741", "Syberia Nótt", Elementalist, Some(Tempest)),
    ],
}

test! {
    name: parse_eyes,
    log: "logs/eyes-20211115.zevtc",
    boss: Encounter::StatueOfDarkness,
    mode: Raid,
    players: &[
        (1, ":Dunje.4863", "Pallida Howhite", Warrior, Some(Berserker)),
        (1, ":Straimer.1093", "I Want Smite Back", Guardian, Some(Dragonhunter)),
        (1, ":Taniniver BlindDragon.9503", "Dragon Kills You", Necromancer, Some(Scourge)),
        (1, ":Timothy.5829", "Annegret On Frenzy", Ranger, Some(Druid)),
        (1, ":xyoz.6710", "Xaphwen", Mesmer, Some(Chronomancer)),
        (2, ":Jupp.4570", "Aldwor", Guardian, Some(Firebrand)),
        (2, ":Kiki.9576", "Spooky Kiki", Necromancer, Some(Scourge)),
        (2, ":Rajnesh.4526", "I Rajnesh I", Revenant, Some(Renegade)),
        (2, ":TheMakNoon.5071", "Twosouls M", Mesmer, Some(Chronomancer)),
        (2, ":neko.9741", "Syberia Nótt", Elementalist, Some(Tempest)),
    ],
}

test! {
    name: parse_dhuum,
    log: "logs/dhuum-20200428.zevtc",
    boss: Encounter::VoiceInTheVoid,
    mode: Raid,
    players: &[
        (1, ":DaZzius.4753", "Amestye Aëther", Mesmer, Some(Chronomancer)),
        (1, ":Dunje.4863", "Maho Shiina", Revenant, Some(Renegade)),
        (1, ":LuthienTinuviel.1082", "Aurevyrn", Guardian, Some(Firebrand)),
        (1, ":RedEaGle.2097", "Alyccya", Ranger, Some(Druid)),
        (1, ":neko.9741", "General Nekobi", Mesmer, Some(Chronomancer)),
        (2, ":Gildraen.5432", "Eären Than", Mesmer, Some(Mirage)),
        (2, ":Kylangel.8409", "Shénmi", Mesmer, Some(Chronomancer)),
        (2, ":NotIlly.7684", "Illy Warbringer", Warrior, Some(Berserker)),
        (2, ":Raelag Arkhen.6347", "Rælag Arkhen", Revenant, Some(Renegade)),
        (2, ":hiemen.2584", "Hiémen", Revenant, Some(Renegade)),
    ],
}

// Wing 6 tests

test! {
    name: parse_conjured_amalgamate,
    log: "logs/ca-20200426.zevtc",
    boss: Encounter::ConjuredAmalgamate,
    mode: Raid,
    players: &[
        (3, ":Admiral Aka Inu.4962", "Großadmiral Aka Inu", Warrior, Some(Berserker)),
        (3, ":Dunje.4863", "Irodo", Elementalist, Some(Weaver)),
        (3, ":Lopoeo.1594", "Glücklich Und Satt", Mesmer, Some(Chronomancer)),
        (3, ":Straimer.1093", "Feel My Epidemic", Necromancer, Some(Reaper)),
        (3, ":neko.9741", "Mordrem Cat", Ranger, Some(Druid)),
        (4, ":Cyen Lazarus.4170", "Krom Bearmaster", Guardian, Some(Dragonhunter)),
        (4, ":Faedoiel.8576", "Faedoiel", Mesmer, Some(Chronomancer)),
        (4, ":chinchiyo.6794", "Charles Penguin", Thief, Some(Daredevil)),
        (4, ":predatorkilla.2391", "Predatorrkilla", Guardian, Some(Dragonhunter)),
        (4, ":sokeenoppa.5384", "Sincis", Thief, Some(Daredevil)),
    ],
}

test! {
    name: parse_largos_twins,
    log: "logs/largos-20200426.zevtc",
    boss: Encounter::TwinLargos,
    mode: Raid,
    players: &[
        (3, ":Cyen Lazarus.4170", "Cyen Blackarrow", Ranger, Some(Druid)),
        (3, ":Dunje.4863", "Godric Gobbledygook", Mesmer, Some(Mirage)),
        (3, ":Lopoeo.1594", "Glücklich Und Satt", Mesmer, Some(Chronomancer)),
        (3, ":Spirit Of Kingdom.4731", "Gungunil", Warrior, Some(Berserker)),
        (3, ":Straimer.1093", "The Meta Is A Líe", Mesmer, Some(Mirage)),
        (4, ":Faedoiel.8576", "Faedoiel", Mesmer, Some(Chronomancer)),
        (4, ":chinchiyo.6794", "Faedlewynn", Revenant, Some(Renegade)),
        (4, ":neko.9741", "Neko Shadowdancer", Mesmer, Some(Mirage)),
        (4, ":predatorkilla.2391", "Illusionzkilla", Mesmer, Some(Mirage)),
        (4, ":sokeenoppa.5384", "O Kng", Elementalist, Some(Tempest)),
    ],
}

test! {
    name: parse_qadim,
    log: "logs/qadim-20200427.zevtc",
    boss: Encounter::Qadim,
    mode: Raid,
    players: &[
        (3, ":Cyen Lazarus.4170", "Cyen Blackarrow", Ranger, Some(Druid)),
        (3, ":Lopoeo.1594", "Glücklich Und Satt", Mesmer, Some(Chronomancer)),
        (3, ":Straimer.1093", "Feel My Epidemic", Necromancer, Some(Reaper)),
        (3, ":WONBO.3265", "Wonbo", Warrior, Some(Berserker)),
        (3, ":neko.9741", "General Nekobi", Mesmer, Some(Chronomancer)),
        (4, ":Dunje.4863", "Clepta Sophia", Thief, Some(Deadeye)),
        (4, ":chinchiyo.6794", "Charles Penguin", Thief, Some(Daredevil)),
        (4, ":predatorkilla.2391", "Predatorrkilla", Guardian, Some(Dragonhunter)),
        (4, ":sokeenoppa.5384", "I Will Carry Fit", Necromancer, Some(Scourge)),
        (4, ":xXCOOLXx.2176", "Rakirah Boonmancer", Mesmer, Some(Chronomancer)),
    ],
}

// Wing 7 tests

test! {
    name: parse_adina,
    log: "logs/adina-20200427.zevtc",
    boss: Encounter::CardinalAdina,
    mode: Raid,
    players: &[
        (3, ":Arkady.3768", "Just Pakly", Engineer, Some(Holosmith)),
        (3, ":Dunje.4863", "Peter Party", Ranger, Some(Soulbeast)),
        (3, ":Vooriden.3927", "Ashbüry", Ranger, Some(Druid)),
        (3, ":WONBO.3265", "Jaínece Crâiser", Mesmer, Some(Chronomancer)),
        (3, ":xyoz.6710", "Xaphy", Engineer, Some(Holosmith)),
        (5, ":AliceWindwalker.6238", "Fafnarin Sunseeker", Warrior, Some(Berserker)),
        (5, ":Gellalli.6580", "Germi N", Revenant, Some(Renegade)),
        (5, ":Straimer.1093", "I Want Smite Back", Guardian, Some(Dragonhunter)),
        (5, ":TruePacman.8703", "Little Pac", Guardian, Some(Dragonhunter)),
        (5, ":neko.9741", "Star Guardian Neko", Guardian, Some(Firebrand)),
    ],
}

test! {
    name: parse_sabir,
    log: "logs/sabir-20200427.zevtc",
    boss: Encounter::CardinalSabir,
    mode: Raid,
    players: &[
        (3, ":Arkady.3768", "Just Pakly", Engineer, Some(Holosmith)),
        (3, ":Dunje.4863", "Emma Hydes", Elementalist, Some(Weaver)),
        (3, ":Vooriden.3927", "Ashbüry", Ranger, Some(Druid)),
        (3, ":WONBO.3265", "Jaínece Crâiser", Mesmer, Some(Chronomancer)),
        (3, ":xyoz.6710", "Xaphwen", Mesmer, Some(Chronomancer)),
        (5, ":AliceWindwalker.6238", "Fafnarin Sunseeker", Warrior, Some(Berserker)),
        (5, ":Gellalli.6580", "Germi N", Revenant, Some(Renegade)),
        (5, ":Straimer.1093", "Revenge On The Meta", Revenant, Some(Renegade)),
        (5, ":TruePacman.8703", "Tolin Halfhand", Revenant, Some(Renegade)),
        (5, ":neko.9741", "Star Guardian Neko", Guardian, Some(Firebrand)),
    ],
}

test! {
    name: parse_qadim_the_peerless,
    log: "logs/qadimp-20200427.zevtc",
    boss: Encounter::QadimThePeerless,
    mode: Raid,
    players: &[
        (3, ":AliceWindwalker.6238", "Fafnarin Sunseeker", Warrior, Some(Berserker)),
        (3, ":Arkady.3768", "Just Pakly", Engineer, Some(Holosmith)),
        (3, ":Vooriden.3927", "Ashbüry", Ranger, Some(Druid)),
        (3, ":WONBO.3265", "Jaínece Crâiser", Mesmer, Some(Chronomancer)),
        (3, ":xyoz.6710", "Xaphwen", Mesmer, Some(Chronomancer)),
        (5, ":Dunje.4863", "Clepta Sophia", Thief, Some(Deadeye)),
        (5, ":Gellalli.6580", "Germi N", Revenant, Some(Renegade)),
        (5, ":Straimer.1093", "Feel My Epidemic", Necromancer, Some(Scourge)),
        (5, ":TruePacman.8703", "Smayser", Necromancer, Some(Scourge)),
        (5, ":neko.9741", "Star Guardian Neko", Guardian, Some(Firebrand)),
    ],
}

// Training area

test! {
    name: parse_standard_kitty_golem,
    log: "logs/standard-golem-20211112.zevtc",
    boss: Encounter::StandardKittyGolem,
    mode: Golem,
    players: &[
        (1, ":Dunje.4863", "Ai Higashi", Guardian, Some(Dragonhunter)),
    ],
}

test! {
    name: parse_medium_kitty_golem,
    log: "logs/medium-golem-20211112.zevtc",
    boss: Encounter::MediumKittyGolem,
    mode: Golem,
    players: &[
        (1, ":Dunje.4863", "Ai Higashi", Guardian, Some(Dragonhunter)),
    ],
}

test! {
    name: parse_large_kitty_golem,
    log: "logs/large-golem-20211112.zevtc",
    boss: Encounter::LargeKittyGolem,
    mode: Golem,
    players: &[
        (1, ":Dunje.4863", "Ai Higashi", Guardian, Some(Dragonhunter)),
    ],
}

// 100 CM tests

test! {
    name: parse_ai,
    log: "logs/ai-20200922.zevtc",
    boss: Encounter::Ai,
    mode: Fractal,
    players: &[
        (0, ":Dunje.4863", "Padme Amidada", Guardian, Some(Firebrand)),
        (0, ":Speeaaakmaan.8974", "Damage Modifiers", Guardian, Some(Firebrand)),
        (0, ":Gellalli.6580", "Germi Burns", Guardian, Some(Firebrand)),
        (0, ":tokageroh.7521", "Jason Redwood", Revenant, Some(Renegade)),
        (0, ":xyoz.6710", "Xaphnira", Guardian, Some(Firebrand)),
    ],
}

// 99 CM tests

test! {
    name: parse_skorvald,
    log: "logs/skorvald-20200427.zevtc",
    boss: Encounter::Skorvald,
    mode: Fractal,
    players: &[
        (0, ":Dunje.4863", "Jane Whiskerlisp", Revenant, Some(Renegade)),
        (0, ":Gellalli.6580", "Germi X", Ranger, Some(Soulbeast)),
        (0, ":Speeaaakmaan.8974", "Damage Modifiers", Guardian, Some(Firebrand)),
        (0, ":Straimer.1093", "Deepfreeze Myself", Elementalist, Some(Weaver)),
        (0, ":neko.9741", "Took Fgs For Banner", Warrior, Some(Berserker)),
    ],
}

test! {
    name: parse_artsariiv,
    log: "logs/artsariiv-20200427.zevtc",
    boss: Encounter::Artsariiv,
    mode: Fractal,
    players: &[
        (0, ":Dunje.4863", "Jane Whiskerlisp", Revenant, Some(Renegade)),
        (0, ":Gellalli.6580", "Germi X", Ranger, Some(Soulbeast)),
        (0, ":Speeaaakmaan.8974", "Damage Modifiers", Guardian, Some(Firebrand)),
        (0, ":Straimer.1093", "Deepfreeze Myself", Elementalist, Some(Weaver)),
        (0, ":neko.9741", "Took Fgs For Banner", Warrior, Some(Berserker)),
    ],
}

test! {
    name: parse_arkk,
    log: "logs/arkk-20200427.zevtc",
    boss: Encounter::Arkk,
    mode: Fractal,
    players: &[
        (0, ":Dunje.4863", "Jane Whiskerlisp", Revenant, Some(Renegade)),
        (0, ":Gellalli.6580", "Germi X", Ranger, Some(Soulbeast)),
        (0, ":Speeaaakmaan.8974", "Damage Modifiers", Guardian, Some(Firebrand)),
        (0, ":Straimer.1093", "Deepfreeze Myself", Elementalist, Some(Weaver)),
        (0, ":neko.9741", "Took Fgs For Banner", Warrior, Some(Berserker)),
    ],
}

// 98 CM tests

test! {
    name: parse_mama,
    log: "logs/mama-20200427.zevtc",
    boss: Encounter::MAMA,
    mode: Fractal,
    players: &[
        (0, ":Dunje.4863", "Jane Whiskerlisp", Revenant, Some(Renegade)),
        (0, ":Gellalli.6580", "Germi X", Ranger, Some(Soulbeast)),
        (0, ":Speeaaakmaan.8974", "Damage Modifiers", Guardian, Some(Firebrand)),
        (0, ":Straimer.1093", "Deepfreeze Myself", Elementalist, Some(Weaver)),
        (0, ":neko.9741", "Took Fgs For Banner", Warrior, Some(Berserker)),
    ],
}

test! {
    name: parse_siax,
    log: "logs/siax-20200427.zevtc",
    boss: Encounter::Siax,
    mode: Fractal,
    players: &[
        (0, ":Dunje.4863", "Jane Whiskerlisp", Revenant, Some(Renegade)),
        (0, ":Gellalli.6580", "Germi X", Ranger, Some(Soulbeast)),
        (0, ":Speeaaakmaan.8974", "Damage Modifiers", Guardian, Some(Firebrand)),
        (0, ":Straimer.1093", "Deepfreeze Myself", Elementalist, Some(Weaver)),
        (0, ":neko.9741", "Took Fgs For Banner", Warrior, Some(Berserker)),
    ],
}

test! {
    name: parse_ensolyss,
    log: "logs/ensolyss-20200427.zevtc",
    boss: Encounter::Ensolyss,
    mode: Fractal,
    players: &[
        (0, ":Dunje.4863", "Jane Whiskerlisp", Revenant, Some(Renegade)),
        (0, ":Gellalli.6580", "Germi X", Ranger, Some(Soulbeast)),
        (0, ":Speeaaakmaan.8974", "Damage Modifiers", Guardian, Some(Firebrand)),
        (0, ":Straimer.1093", "Deepfreeze Myself", Elementalist, Some(Weaver)),
        (0, ":neko.9741", "Took Fgs For Banner", Warrior, Some(Berserker)),
    ],
}

// Strike mission tests

test! {
    name: parse_icebrood,
    log: "logs/icebrood-20200424.zevtc",
    boss: Encounter::IcebroodConstruct,
    mode: Strike,
    players: &[
        (3, ":Dunje.4863", "Thank You Exorcist", Necromancer, Some(Reaper)),
        (3, ":Speeaaakmaan.8974", "Damage Modifiers", Guardian, Some(Firebrand)),
        (3, ":Straimer.1093", "Revenge On The Meta", Revenant, Some(Renegade)),
        (3, ":Vario.4875", "Baelgar Skyfire", Elementalist, Some(Tempest)),
        (3, ":xChriS.5213", "I Am Raponz", Revenant, Some(Renegade)),
        (4, ":Blaz.8675", "Guesthelper", Ranger, Some(Druid)),
        (4, ":Friedensschreck.2573", "Lucy Brent", Mesmer, Some(Chronomancer)),
        (4, ":Gellalli.6580", "Germi J", Necromancer, Some(Reaper)),
        (4, ":Nydena.4371", "Lanori", Warrior, Some(Berserker)),
        (4, ":Xion.5790", "Underwater Nab", Engineer, Some(Holosmith)),
    ],
}

test! {
    name: parse_kodan_brothers,
    log: "logs/kodans-20200424.zevtc",
    boss: Encounter::SuperKodanBrothers,
    mode: Strike,
    players: &[
        (3, ":Gellalli.6580", "Germi J", Necromancer, Some(Scourge)),
        (3, ":Speeaaakmaan.8974", "Damage Modifiers", Guardian, Some(Firebrand)),
        (3, ":Straimer.1093", "Revenge On The Meta", Revenant, Some(Renegade)),
        (3, ":Vario.4875", "Baelgar Skyfire", Elementalist, Some(Tempest)),
        (3, ":xChriS.5213", "I Am Raponz", Revenant, Some(Renegade)),
        (4, ":Blaz.8675", "Guesthelper", Ranger, Some(Druid)),
        (4, ":Dunje.4863", "Thank You Exorcist", Necromancer, Some(Reaper)),
        (4, ":Friedensschreck.2573", "Lucy Brent", Mesmer, Some(Chronomancer)),
        (4, ":Nydena.4371", "Lanori", Warrior, Some(Berserker)),
        (4, ":Ouren.7530", "Pip The Dragon", Guardian, Some(Firebrand)),
    ],
}

test! {
    name: parse_fraenir_of_jormag,
    log: "logs/fraenir-20200424.zevtc",
    boss: Encounter::FraenirOfJormag,
    mode: Strike,
    players: &[
        (3, ":Dunje.4863", "Thank You Exorcist", Necromancer, Some(Reaper)),
        (3, ":Speeaaakmaan.8974", "Damage Modifiers", Guardian, Some(Firebrand)),
        (3, ":Straimer.1093", "Revenge On The Meta", Revenant, Some(Renegade)),
        (3, ":Vario.4875", "Baelgar Skyfire", Elementalist, Some(Tempest)),
        (3, ":xChriS.5213", "I Am Raponz", Revenant, Some(Renegade)),
        (4, ":Blaz.8675", "Guesthelper", Ranger, Some(Druid)),
        (4, ":Friedensschreck.2573", "Lucy Brent", Mesmer, Some(Chronomancer)),
        (4, ":Gellalli.6580", "Germi J", Necromancer, Some(Scourge)),
        (4, ":Nydena.4371", "Lanori", Warrior, Some(Berserker)),
        (4, ":Xion.5790", "Underwater Nab", Engineer, Some(Holosmith)),
    ],
}

test! {
    name: parse_boneskinner,
    log: "logs/boneskinner-20200424.zevtc",
    boss: Encounter::Boneskinner,
    mode: Strike,
    players: &[
        (3, ":Gellalli.6580", "Germi J", Necromancer, Some(Scourge)),
        (3, ":Speeaaakmaan.8974", "Damage Modifiers", Guardian, Some(Firebrand)),
        (3, ":Straimer.1093", "Revenge On The Meta", Revenant, Some(Renegade)),
        (3, ":Vario.4875", "Baelgar Skyfire", Elementalist, Some(Tempest)),
        (3, ":xChriS.5213", "I Am Raponz", Revenant, Some(Renegade)),
        (4, ":Blaz.8675", "Guesthelper", Ranger, Some(Druid)),
        (4, ":Dunje.4863", "Thank You Exorcist", Necromancer, Some(Reaper)),
        (4, ":Friedensschreck.2573", "Lucy Brent", Mesmer, Some(Chronomancer)),
        (4, ":Nydena.4371", "Lanori", Warrior, Some(Berserker)),
        (4, ":Ouren.7530", "Pip The Dragon", Guardian, Some(Firebrand)),
    ],
}

test! {
    name: parse_whisper_of_jormag,
    log: "logs/whisper-20200424.zevtc",
    boss: Encounter::WhisperOfJormag,
    mode: Strike,
    players: &[
        (3, ":Gellalli.6580", "Germi J", Necromancer, Some(Scourge)),
        (3, ":Speeaaakmaan.8974", "Damage Modifiers", Guardian, Some(Firebrand)),
        (3, ":Straimer.1093", "Revenge On The Meta", Revenant, Some(Renegade)),
        (3, ":Vario.4875", "Baelgar Skyfire", Elementalist, Some(Tempest)),
        (3, ":xChriS.5213", "I Am Raponz", Revenant, Some(Renegade)),
        (4, ":Blaz.8675", "Guesthelper", Ranger, Some(Druid)),
        (4, ":Dunje.4863", "Thank You Exorcist", Necromancer, Some(Reaper)),
        (4, ":Friedensschreck.2573", "Lucy Brent", Mesmer, Some(Chronomancer)),
        (4, ":Nydena.4371", "Lanori", Warrior, Some(Berserker)),
        (4, ":Ouren.7530", "Pip The Dragon", Guardian, Some(Firebrand)),
    ],
}

// Various tests

test! {
    name: parse_old_cairn_log,
    log: "logs/old-cairn-20180321.evtc.zip",
    boss: Encounter::Cairn,
    mode: Raid,
    players: &[
        (1, ":Medejz.1679", "Nuerha", Guardian, Some(Firebrand)),
        (1, ":ONEVA.5860", "Berserkoala", Revenant, Some(Renegade)),
        (1, ":Speeaaakmaan.8974", "I Block Eggs", Mesmer, Some(Chronomancer)),
        (1, ":Villhelma.6829", "Viseria", Ranger, Some(Druid)),
        (1, ":gaga.3014", "Hankahn", Warrior, Some(Berserker)),
        (2, ":Arcanis.4602", "Ivy Deathcloud", Necromancer, Some(Scourge)),
        (2, ":Dunje.4863", "Zraraelia Vey", Necromancer, Some(Scourge)),
        (2, ":Gellalli.6580", "Germi X", Ranger, Some(Druid)),
        (2, ":Gizmo.1635", "Oleg Deathbringer", Necromancer, Some(Scourge)),
        (2, ":flumbum.4068", "Xynaliba", Mesmer, Some(Chronomancer)),
    ],
}
