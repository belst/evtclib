//! This module contains some low-level game data, such as different boss IDs.
use num_derive::FromPrimitive;
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};
use thiserror::Error;

/// Enum containing all encounters with their IDs.
///
/// An encounter is a fight or event, consisting of no, one or multiple bosses. Most encounters map
/// 1:1 to a boss (like Vale Guardian), however there are some encounters with multiple bosses
/// (like Twin Largos), and even encounters without bosses (like the River of Souls).
///
/// This enum is non-exhaustive to ensure that future added encounters can be added without
/// inducing a breaking change.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive)]
#[non_exhaustive]
pub enum Encounter {
    // Wing 1
    ValeGuardian = 0x3C4E,
    Gorseval = 0x3C45,
    Sabetha = 0x3C0F,

    // Wing 2
    Slothasor = 0x3EFB,
    Matthias = 0x3EF3,

    // Wing 3
    KeepConstruct = 0x3F6B,
    Xera = 0x3F76,

    // Wing 4
    Cairn = 0x432A,
    MursaatOverseer = 0x4314,
    Samarog = 0x4324,
    Deimos = 0x4302,

    // Wing 5
    SoullessHorror = 0x4D37,
    VoiceInTheVoid = 0x4BFA,

    // Wing 6
    ConjuredAmalgamate = 0xABC6,
    TwinLargos = 0x5271,
    Qadim = 0x51C6,

    // Wing 7
    CardinalAdina = 0x55F6,
    CardinalSabir = 0x55CC,
    QadimThePeerless = 0x55F0,

    // 100 CM (Sunqua Peak)
    Ai = 0x5AD6,

    // 99 CM (Shattered Observatory)
    Skorvald = 0x44E0,
    Artsariiv = 0x461D,
    Arkk = 0x455F,

    // 98 CM (Nightmare)
    MAMA = 0x427D,
    Siax = 0x4284,
    Ensolyss = 0x4234,

    // Strike missions
    IcebroodConstruct = 0x568A,
    SuperKodanBrothers = 0x5747,
    FraenirOfJormag = 0x57DC,
    Boneskinner = 0x57F9,
    WhisperOfJormag = 0x58B7,
}

/// Error for when converting a string to an encounter fails.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Error)]
#[error("Invalid encounter identifier: {0}")]
pub struct ParseEncounterError(String);

impl FromStr for Encounter {
    type Err = ParseEncounterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        match &lower as &str {
            "vg" | "vale guardian" => Ok(Encounter::ValeGuardian),
            "gorse" | "gorseval" => Ok(Encounter::Gorseval),
            "sab" | "sabetha" => Ok(Encounter::Sabetha),

            "sloth" | "slothasor" => Ok(Encounter::Slothasor),
            "matthias" => Ok(Encounter::Matthias),

            "kc" | "keep construct" => Ok(Encounter::KeepConstruct),
            "xera" => Ok(Encounter::Xera),

            "cairn" => Ok(Encounter::Cairn),
            "mo" | "mursaat overseer" => Ok(Encounter::MursaatOverseer),
            "sam" | "sama" | "samarog" => Ok(Encounter::Samarog),
            "deimos" => Ok(Encounter::Deimos),

            "desmina" | "sh" | "soulless horror" => Ok(Encounter::SoullessHorror),
            "dhuum" | "voice in the void" => Ok(Encounter::VoiceInTheVoid),

            "ca" | "conjured amalgamate" => Ok(Encounter::ConjuredAmalgamate),
            "largos" | "twins" | "largos twins" => Ok(Encounter::TwinLargos),
            "qadim" => Ok(Encounter::Qadim),

            "adina" | "cardinal adina" => Ok(Encounter::CardinalAdina),
            "sabir" | "cardinal sabir" => Ok(Encounter::CardinalSabir),
            "qadimp" | "peerless qadim" | "qadim the peerless" => Ok(Encounter::QadimThePeerless),

            "ai" | "ai keeper of the peak" => Ok(Encounter::Ai),

            "skorvald" => Ok(Encounter::Skorvald),
            "artsariiv" => Ok(Encounter::Artsariiv),
            "arkk" => Ok(Encounter::Arkk),

            "mama" => Ok(Encounter::MAMA),
            "siax" => Ok(Encounter::Siax),
            "ensolyss" | "ensolyss of the endless torment" => Ok(Encounter::Ensolyss),

            "icebrood" | "icebrood construct" => Ok(Encounter::IcebroodConstruct),
            "kodans" | "super kodan brothers" => Ok(Encounter::SuperKodanBrothers),
            "fraenir" | "fraenir of jormag" => Ok(Encounter::FraenirOfJormag),
            "boneskinner" => Ok(Encounter::Boneskinner),
            "whisper" | "whisper of jormag" => Ok(Encounter::WhisperOfJormag),

            _ => Err(ParseEncounterError(s.to_owned())),
        }
    }
}

impl Display for Encounter {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            Encounter::ValeGuardian => "Vale Guardian",
            Encounter::Gorseval => "Gorseval",
            Encounter::Sabetha => "Sabetha",
            Encounter::Slothasor => "Slothasor",
            Encounter::Matthias => "Matthias Gabrel",
            Encounter::KeepConstruct => "Keep Construct",
            Encounter::Xera => "Xera",
            Encounter::Cairn => "Cairn the Indomitable",
            Encounter::MursaatOverseer => "Mursaat Overseer",
            Encounter::Samarog => "Samarog",
            Encounter::Deimos => "Deimos",
            Encounter::SoullessHorror => "Soulless Horror",
            Encounter::VoiceInTheVoid => "Voice in the Void",
            Encounter::ConjuredAmalgamate => "Conjured Amalgamate",
            Encounter::TwinLargos => "Twin Largos",
            Encounter::Qadim => "Qadim",
            Encounter::CardinalAdina => "Cardinal Adina",
            Encounter::CardinalSabir => "Cardinal Sabir",
            Encounter::QadimThePeerless => "Qadim the Peerless",
            Encounter::Ai => "Ai Keeper of the Peak",
            Encounter::Skorvald => "Skorvald the Shattered",
            Encounter::Artsariiv => "Artsariiv",
            Encounter::Arkk => "Arkk",
            Encounter::MAMA => "MAMA",
            Encounter::Siax => "Siax the Corrupted",
            Encounter::Ensolyss => "Ensolyss of the Endless Torment",
            Encounter::IcebroodConstruct => "Icebrood Construct",
            Encounter::SuperKodanBrothers => "Super Kodan Brothers",
            Encounter::FraenirOfJormag => "Fraenir of Jormag",
            Encounter::Boneskinner => "Boneskinner",
            Encounter::WhisperOfJormag => "Whisper of Jormag",
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
pub const NIKARE_ID: u16 = Encounter::TwinLargos as u16;
/// The ID of Kenut in the Twin Largos fight.
pub const KENUT_ID: u16 = 21089;

/// The ID of the Voice of the Fallen.
pub const VOICE_OF_THE_FALLEN_ID: u16 = Encounter::SuperKodanBrothers as u16;
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
    fn test_encounter_parsing_ok() {
        use Encounter::*;
        let tests: &[(&'static str, Encounter)] = &[
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
            ("dhuum", VoiceInTheVoid),
            ("Dhuum", VoiceInTheVoid),
            ("ca", ConjuredAmalgamate),
            ("conjured amalgamate", ConjuredAmalgamate),
            ("Conjured Amalgamate", ConjuredAmalgamate),
            ("largos", TwinLargos),
            ("twins", TwinLargos),
            ("largos twins", TwinLargos),
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
            ("kodans", SuperKodanBrothers),
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
    fn test_encounter_parsing_err() {
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
            assert!(test.parse::<Encounter>().is_err());
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
