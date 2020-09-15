//! This module contains some low-level game data, such as different boss IDs.
use num_derive::FromPrimitive;
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};
use thiserror::Error;

/// Enum containing all bosses with their IDs.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive)]
pub enum Boss {
    // Wing 1
    ValeGuardian = 0x3C4E,
    Gorseval = 0x3C45,
    Sabetha = 0x3C0F,

    // Wing 2
    Slothasor = 0x3EFB,
    Matthias = 0x3EF3,

    // Wing 3
    KeepConstruct = 0x3F6B,
    /// Xera ID for phase 1.
    ///
    /// This is only half of Xera's ID, as there will be a second agent for the
    /// second phase. This agent will have another ID, see
    /// [`XERA_PHASE2_ID`](constant.XERA_PHASE2_ID.html).
    Xera = 0x3F76,

    // Wing 4
    Cairn = 0x432A,
    MursaatOverseer = 0x4314,
    Samarog = 0x4324,
    Deimos = 0x4302,

    // Wing 5
    SoullessHorror = 0x4D37,
    Dhuum = 0x4BFA,

    // Wing 6
    ConjuredAmalgamate = 0xABC6,
    /// This is the ID of Nikare, as that is what the Twin Largos logs are identified by.
    ///
    /// If you want Nikare specifically, consider using [`NIKARE_ID`][NIKARE_ID], and similarly, if
    /// you need Kenut, you can use [`KENUT_ID`][KENUT_ID].
    LargosTwins = 0x5271,
    Qadim = 0x51C6,

    // Wing 7
    CardinalAdina = 0x55F6,
    CardinalSabir = 0x55CC,
    QadimThePeerless = 0x55F0,

    // 100 CM
    Skorvald = 0x44E0,
    Artsariiv = 0x461D,
    Arkk = 0x455F,

    // 99 CM
    MAMA = 0x427D,
    Siax = 0x4284,
    Ensolyss = 0x4234,

    // Strike missions
    IcebroodConstruct = 0x568A,
    /// This is the ID of the Voice of the Fallen.
    ///
    /// The strike mission itself contains two bosses, the Voice of the Fallen and the Claw of the
    /// Fallen. Consider using either [`VOICE_OF_THE_FALLEN_ID`][VOICE_OF_THE_FALLEN_ID] or
    /// [`CLAW_OF_THE_FALLEN_ID`][CLAW_OF_THE_FALLEN_ID] if you refer to one of those bosses
    /// specifically.
    VoiceOfTheFallen = 0x5747,
    FraenirOfJormag = 0x57DC,
    Boneskinner = 0x57F9,
    WhisperOfJormag = 0x58B7,
}

/// Error for when converting a string to the boss fails.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Error)]
#[error("Invalid boss identifier: {0}")]
pub struct ParseBossError(String);

impl FromStr for Boss {
    type Err = ParseBossError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        match &lower as &str {
            "vg" | "vale guardian" => Ok(Boss::ValeGuardian),
            "gorse" | "gorseval" => Ok(Boss::Gorseval),
            "sab" | "sabetha" => Ok(Boss::Sabetha),

            "sloth" | "slothasor" => Ok(Boss::Slothasor),
            "matthias" => Ok(Boss::Matthias),

            "kc" | "keep construct" => Ok(Boss::KeepConstruct),
            "xera" => Ok(Boss::Xera),

            "cairn" => Ok(Boss::Cairn),
            "mo" | "mursaat overseer" => Ok(Boss::MursaatOverseer),
            "sam" | "sama" | "samarog" => Ok(Boss::Samarog),
            "deimos" => Ok(Boss::Deimos),

            "desmina" | "sh" | "soulless horror" => Ok(Boss::SoullessHorror),
            "dhuum" => Ok(Boss::Dhuum),

            "ca" | "conjured amalgamate" => Ok(Boss::ConjuredAmalgamate),
            "largos" | "twins" | "largos twins" => Ok(Boss::LargosTwins),
            "qadim" => Ok(Boss::Qadim),

            "adina" | "cardinal adina" => Ok(Boss::CardinalAdina),
            "sabir" | "cardinal sabir" => Ok(Boss::CardinalSabir),
            "qadimp" | "peerless qadim" | "qadim the peerless" => Ok(Boss::QadimThePeerless),

            "skorvald" => Ok(Boss::Skorvald),
            "artsariiv" => Ok(Boss::Artsariiv),
            "arkk" => Ok(Boss::Arkk),

            "mama" => Ok(Boss::MAMA),
            "siax" => Ok(Boss::Siax),
            "ensolyss" | "ensolyss of the endless torment" => Ok(Boss::Ensolyss),

            "icebrood" | "icebrood construct" => Ok(Boss::IcebroodConstruct),
            "kodans" | "super kodan brothers" => Ok(Boss::VoiceOfTheFallen),
            "fraenir" | "fraenir of jormag" => Ok(Boss::FraenirOfJormag),
            "boneskinner" => Ok(Boss::Boneskinner),
            "whisper" | "whisper of jormag" => Ok(Boss::WhisperOfJormag),

            _ => Err(ParseBossError(s.to_owned())),
        }
    }
}

impl Display for Boss {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            Boss::ValeGuardian => "Vale Guardian",
            Boss::Gorseval => "Gorseval",
            Boss::Sabetha => "Sabetha",
            Boss::Slothasor => "Slothasor",
            Boss::Matthias => "Matthias Gabrel",
            Boss::KeepConstruct => "Keep Construct",
            Boss::Xera => "Xera",
            Boss::Cairn => "Cairn the Indomitable",
            Boss::MursaatOverseer => "Mursaat Overseer",
            Boss::Samarog => "Samarog",
            Boss::Deimos => "Deimos",
            Boss::SoullessHorror => "Soulless Horror",
            Boss::Dhuum => "Dhuum",
            Boss::ConjuredAmalgamate => "Conjured Amalgamate",
            Boss::LargosTwins => "Twin Largos",
            Boss::Qadim => "Qadim",
            Boss::CardinalAdina => "Cardinal Adina",
            Boss::CardinalSabir => "Cardinal Sabir",
            Boss::QadimThePeerless => "Qadim the Peerless",
            Boss::Skorvald => "Skorvald the Shattered",
            Boss::Artsariiv => "Artsariiv",
            Boss::Arkk => "Arkk",
            Boss::MAMA => "MAMA",
            Boss::Siax => "Siax the Corrupted",
            Boss::Ensolyss => "Ensolyss of the Endless Torment",
            Boss::IcebroodConstruct => "Icebrood Construct",
            Boss::VoiceOfTheFallen => "Super Kodan Brothers",
            Boss::FraenirOfJormag => "Fraenir of Jormag",
            Boss::Boneskinner => "Boneskinner",
            Boss::WhisperOfJormag => "Whisper of Jormag",
        };
        write!(f, "{}", name)
    }
}

/// ID for Xera in the second phase.
///
/// The original Xera will despawn, and after the tower phase, a separate spawn
/// will take over. This new Xera will have this ID. Care needs to be taken when
/// calculating boss damage on this encounter, as both Xeras have to be taken
/// into account.
pub const XERA_PHASE2_ID: u16 = 0x3F9E;

/// The ID of Nikare in the Twin Largos fight.
pub const NIKARE_ID: u16 = Boss::LargosTwins as u16;
/// The ID of Kenut in the Twin Largos fight.
pub const KENUT_ID: u16 = 21089;

/// The ID of the Voice of the Fallen.
pub const VOICE_OF_THE_FALLEN_ID: u16 = Boss::VoiceOfTheFallen as u16;
/// The ID of the Claw of the Fallen.
pub const CLAW_OF_THE_FALLEN_ID: u16 = 22481;

/// Error for when converting a string to a profession fails.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
#[error("Invalid profession identifier: {0}")]
pub struct ParseProfessionError(String);

/// An in-game profession.
///
/// This only contains the 9 base professions. For elite specializations, see
/// [`EliteSpec`][EliteSpec].
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive)]
pub enum Profession {
    Guardian = 1,
    Warrior = 2,
    Engineer = 3,
    Ranger = 4,
    Thief = 5,
    Elementalist = 6,
    Mesmer = 7,
    Necromancer = 8,
    Revenant = 9,
}

impl FromStr for Profession {
    type Err = ParseProfessionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_lowercase() as &str {
            "guardian" => Ok(Profession::Guardian),
            "warrior" => Ok(Profession::Warrior),
            "engineer" => Ok(Profession::Engineer),
            "ranger" => Ok(Profession::Ranger),
            "thief" => Ok(Profession::Thief),
            "elementalist" => Ok(Profession::Elementalist),
            "mesmer" => Ok(Profession::Mesmer),
            "necromancer" => Ok(Profession::Necromancer),
            "revenant" => Ok(Profession::Revenant),

            _ => Err(ParseProfessionError(s.to_owned())),
        }
    }
}

impl Display for Profession {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            Profession::Guardian => "Guardian",
            Profession::Warrior => "Warrior",
            Profession::Engineer => "Engineer",
            Profession::Ranger => "Ranger",
            Profession::Thief => "Thief",
            Profession::Elementalist => "Elementalist",
            Profession::Mesmer => "Mesmer",
            Profession::Necromancer => "Necromancer",
            Profession::Revenant => "Revenant",
        };
        write!(f, "{}", name)
    }
}

/// Error for when converting a string to an elite specialization fails.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
#[error("Invalid elite specialization identifier: {0}")]
pub struct ParseEliteSpecError(String);

/// All possible elite specializations.
///
/// Note that the numeric value of the enum variants correspond to the specialization ID in the API
/// as well. See [the official wiki](https://wiki.guildwars2.com/wiki/API:2/specializations) for
/// more information regarding the API usage.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive)]
pub enum EliteSpec {
    // Heart of Thorns elites:
    Dragonhunter = 27,
    Berserker = 18,
    Scrapper = 43,
    Druid = 5,
    Daredevil = 7,
    Tempest = 48,
    Chronomancer = 40,
    Reaper = 34,
    Herald = 52,

    // Path of Fire elites:
    Firebrand = 62,
    Spellbreaker = 61,
    Holosmith = 57,
    Soulbeast = 55,
    Deadeye = 58,
    Weaver = 56,
    Mirage = 59,
    Scourge = 60,
    Renegade = 63,
}

impl FromStr for EliteSpec {
    type Err = ParseEliteSpecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_lowercase() as &str {
            "dragonhunter" => Ok(EliteSpec::Dragonhunter),
            "berserker" => Ok(EliteSpec::Berserker),
            "scrapper" => Ok(EliteSpec::Scrapper),
            "druid" => Ok(EliteSpec::Druid),
            "daredevil" => Ok(EliteSpec::Daredevil),
            "tempest" => Ok(EliteSpec::Tempest),
            "chronomancer" => Ok(EliteSpec::Chronomancer),
            "reaper" => Ok(EliteSpec::Reaper),
            "herald" => Ok(EliteSpec::Herald),

            "firebrand" => Ok(EliteSpec::Firebrand),
            "spellbreaker" => Ok(EliteSpec::Spellbreaker),
            "holosmith" => Ok(EliteSpec::Holosmith),
            "soulbeast" => Ok(EliteSpec::Soulbeast),
            "deadeye" => Ok(EliteSpec::Deadeye),
            "weaver" => Ok(EliteSpec::Weaver),
            "mirage" => Ok(EliteSpec::Mirage),
            "scourge" => Ok(EliteSpec::Scourge),
            "renegade" => Ok(EliteSpec::Renegade),

            _ => Err(ParseEliteSpecError(s.to_owned())),
        }
    }
}

impl Display for EliteSpec {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            EliteSpec::Dragonhunter => "Dragonhunter",
            EliteSpec::Berserker => "Berserker",
            EliteSpec::Scrapper => "Scrapper",
            EliteSpec::Druid => "Druid",
            EliteSpec::Daredevil => "Daredevil",
            EliteSpec::Tempest => "Tempest",
            EliteSpec::Chronomancer => "Chronomancer",
            EliteSpec::Reaper => "Reaper",
            EliteSpec::Herald => "Herald",
            EliteSpec::Firebrand => "Firebrand",
            EliteSpec::Spellbreaker => "Spellbreaker",
            EliteSpec::Holosmith => "Holosmith",
            EliteSpec::Soulbeast => "Soulbeast",
            EliteSpec::Deadeye => "Deadeye",
            EliteSpec::Weaver => "Weaver",
            EliteSpec::Mirage => "Mirage",
            EliteSpec::Scourge => "Scourge",
            EliteSpec::Renegade => "Renegade",
        };
        write!(f, "{}", name)
    }
}

impl EliteSpec {
    /// Return the profession that this elite specialization belongs to.
    ///
    /// This value is hardcoded (and not expected to change), and does not require a network
    /// connection or API access.
    pub fn profession(self) -> Profession {
        use EliteSpec::*;
        match self {
            Dragonhunter | Firebrand => Profession::Guardian,
            Berserker | Spellbreaker => Profession::Warrior,
            Scrapper | Holosmith => Profession::Engineer,
            Druid | Soulbeast => Profession::Ranger,
            Daredevil | Deadeye => Profession::Thief,
            Tempest | Weaver => Profession::Elementalist,
            Chronomancer | Mirage => Profession::Mesmer,
            Reaper | Scourge => Profession::Necromancer,
            Herald | Renegade => Profession::Revenant,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boss_parsing_ok() {
        use Boss::*;
        let tests: &[(&'static str, Boss)] = &[
            ("vg", ValeGuardian),
            ("VG", ValeGuardian),
            ("vale guardian", ValeGuardian),
            ("Vale Guardian", ValeGuardian),
            ("gorse", Gorseval),
            ("Gorse", Gorseval),
            ("gorseval", Gorseval),
            ("Gorseval", Gorseval),
            ("sab", Sabetha),
            ("sabetha", Sabetha),
            ("Sabetha", Sabetha),
            ("sloth", Slothasor),
            ("slothasor", Slothasor),
            ("Slothasor", Slothasor),
            ("matthias", Matthias),
            ("Matthias", Matthias),
            ("kc", KeepConstruct),
            ("KC", KeepConstruct),
            ("keep construct", KeepConstruct),
            ("Keep Construct", KeepConstruct),
            ("xera", Xera),
            ("Xera", Xera),
            ("cairn", Cairn),
            ("Cairn", Cairn),
            ("mo", MursaatOverseer),
            ("MO", MursaatOverseer),
            ("mursaat overseer", MursaatOverseer),
            ("Mursaat Overseer", MursaatOverseer),
            ("samarog", Samarog),
            ("Samarog", Samarog),
            ("deimos", Deimos),
            ("Deimos", Deimos),
            ("sh", SoullessHorror),
            ("soulless horror", SoullessHorror),
            ("desmina", SoullessHorror),
            ("Desmina", SoullessHorror),
            ("dhuum", Dhuum),
            ("Dhuum", Dhuum),
            ("ca", ConjuredAmalgamate),
            ("conjured amalgamate", ConjuredAmalgamate),
            ("Conjured Amalgamate", ConjuredAmalgamate),
            ("largos", LargosTwins),
            ("twins", LargosTwins),
            ("largos twins", LargosTwins),
            ("qadim", Qadim),
            ("Qadim", Qadim),
            ("adina", CardinalAdina),
            ("cardinal adina", CardinalAdina),
            ("Cardinal Adina", CardinalAdina),
            ("sabir", CardinalSabir),
            ("cardinal sabir", CardinalSabir),
            ("Cardinal Sabir", CardinalSabir),
            ("qadimp", QadimThePeerless),
            ("qadim the peerless", QadimThePeerless),
            ("Qadim The Peerless", QadimThePeerless),
            ("skorvald", Skorvald),
            ("Skorvald", Skorvald),
            ("artsariiv", Artsariiv),
            ("Artsariiv", Artsariiv),
            ("arkk", Arkk),
            ("Arkk", Arkk),
            ("mama", MAMA),
            ("MAMA", MAMA),
            ("siax", Siax),
            ("SIAX", Siax),
            ("ensolyss", Ensolyss),
            ("Ensolyss", Ensolyss),
            ("Ensolyss of the Endless Torment", Ensolyss),
            ("icebrood", IcebroodConstruct),
            ("Icebrood Construct", IcebroodConstruct),
            ("fraenir", FraenirOfJormag),
            ("Fraenir of Jormag", FraenirOfJormag),
            ("boneskinner", Boneskinner),
            ("kodans", VoiceOfTheFallen),
            ("whisper", WhisperOfJormag),
            ("Whisper of Jormag", WhisperOfJormag),
        ];

        for (input, expected) in tests {
            assert_eq!(
                input.parse(),
                Ok(*expected),
                "parsing input {:?} failed",
                input
            );
        }
    }

    #[test]
    fn test_boss_parsing_err() {
        let tests = &[
            "",
            "vga",
            "VGA",
            "foovg",
            "valeguardian",
            "ValeGuardian",
            "slotha",
            "slot",
            "slothasora",
            "cardinal",
        ];
        for test in tests {
            assert!(test.parse::<Boss>().is_err());
        }
    }

    #[test]
    fn test_profession_parsing_ok() {
        use Profession::*;
        let tests: &[(&'static str, Profession)] = &[
            ("guardian", Guardian),
            ("Guardian", Guardian),
            ("warrior", Warrior),
            ("Warrior", Warrior),
            ("revenant", Revenant),
            ("Revenant", Revenant),
            ("thief", Thief),
            ("Thief", Thief),
            ("engineer", Engineer),
            ("Engineer", Engineer),
            ("ranger", Ranger),
            ("Ranger", Ranger),
            ("mesmer", Mesmer),
            ("Mesmer", Mesmer),
            ("elementalist", Elementalist),
            ("Elementalist", Elementalist),
            ("necromancer", Necromancer),
            ("Necromancer", Necromancer),
        ];

        for (input, expected) in tests {
            assert_eq!(
                input.parse(),
                Ok(*expected),
                "parsing input {:?} failed",
                input
            );
        }
    }

    #[test]
    fn test_profession_parsing_err() {
        let tests = &["", "guardiann", "gu", "thiefthief"];
        for test in tests {
            assert!(test.parse::<Profession>().is_err());
        }
    }

    #[test]
    fn test_elite_spec_parsing_ok() {
        use EliteSpec::*;
        let tests: &[(&'static str, EliteSpec)] = &[
            ("dragonhunter", Dragonhunter),
            ("Dragonhunter", Dragonhunter),
            ("firebrand", Firebrand),
            ("Firebrand", Firebrand),
            ("berserker", Berserker),
            ("Berserker", Berserker),
            ("spellbreaker", Spellbreaker),
            ("Spellbreaker", Spellbreaker),
            ("herald", Herald),
            ("Herald", Herald),
            ("renegade", Renegade),
            ("Renegade", Renegade),
            ("daredevil", Daredevil),
            ("Daredevil", Daredevil),
            ("deadeye", Deadeye),
            ("Deadeye", Deadeye),
            ("scrapper", Scrapper),
            ("Scrapper", Scrapper),
            ("holosmith", Holosmith),
            ("Holosmith", Holosmith),
            ("druid", Druid),
            ("Druid", Druid),
            ("soulbeast", Soulbeast),
            ("Soulbeast", Soulbeast),
            ("tempest", Tempest),
            ("Tempest", Tempest),
            ("weaver", Weaver),
            ("Weaver", Weaver),
            ("chronomancer", Chronomancer),
            ("Chronomancer", Chronomancer),
            ("mirage", Mirage),
            ("Mirage", Mirage),
            ("reaper", Reaper),
            ("Reaper", Reaper),
            ("scourge", Scourge),
            ("Scourge", Scourge),
        ];

        for (input, expected) in tests {
            assert_eq!(
                input.parse(),
                Ok(*expected),
                "parsing input {:?} failed",
                input
            );
        }
    }

    #[test]
    fn test_elite_spec_parsing_err() {
        let tests = &[
            "",
            "dragonhunterr",
            "dragonhunt",
            "berserkerberserker",
            "berserk",
        ];
        for test in tests {
            assert!(test.parse::<EliteSpec>().is_err());
        }
    }
}
