//! This module contains some low-level game data, such as different boss IDs.
use num_derive::FromPrimitive;
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};
use thiserror::Error;

/// Enum containing all encounters with their IDs.
///
/// An encounter is a fight or event for which a log can exist. An encounter consists of no, one or
/// multiple bosses. Most encounters map 1:1 to a boss (like Vale Guardian), however there are some
/// encounters with multiple bosses (like Twin Largos), and even encounters without bosses (like
/// the River of Souls, currently not implemented.).
///
/// This enum is non-exhaustive to ensure that future encounters can be added without
/// inducing a breaking change.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive)]
#[non_exhaustive]
#[repr(u16)]
pub enum Encounter {
    // Wing 1
    ValeGuardian = Boss::ValeGuardian as u16,
    Gorseval = Boss::Gorseval as u16,
    Sabetha = Boss::Sabetha as u16,

    // Wing 2
    Slothasor = Boss::Slothasor as u16,
    Matthias = Boss::Matthias as u16,

    // Wing 3
    KeepConstruct = Boss::KeepConstruct as u16,
    Xera = Boss::Xera as u16,

    // Wing 4
    Cairn = Boss::Cairn as u16,
    MursaatOverseer = Boss::MursaatOverseer as u16,
    Samarog = Boss::Samarog as u16,
    Deimos = Boss::Deimos as u16,

    // Wing 5
    SoullessHorror = Boss::SoullessHorror as u16,
    VoiceInTheVoid = Boss::Dhuum as u16,

    // Wing 6
    ConjuredAmalgamate = Boss::ConjuredAmalgamate as u16,
    TwinLargos = Boss::Nikare as u16,
    Qadim = Boss::Qadim as u16,

    // Wing 7
    CardinalAdina = Boss::CardinalAdina as u16,
    CardinalSabir = Boss::CardinalSabir as u16,
    QadimThePeerless = Boss::QadimThePeerless as u16,

    // 100 CM (Sunqua Peak)
    Ai = Boss::Ai as u16,

    // 99 CM (Shattered Observatory)
    Skorvald = Boss::Skorvald as u16,
    Artsariiv = Boss::Artsariiv as u16,
    Arkk = Boss::Arkk as u16,

    // 98 CM (Nightmare)
    MAMA = Boss::MAMA as u16,
    Siax = Boss::Siax as u16,
    Ensolyss = Boss::Ensolyss as u16,

    // Strike missions
    IcebroodConstruct = Boss::IcebroodConstruct as u16,
    /// Internal name for the "Voice of the Fallen and Claw of the Fallen" strike mission.
    SuperKodanBrothers = Boss::VoiceOfTheFallen as u16,
    FraenirOfJormag = Boss::FraenirOfJormag as u16,
    Boneskinner = Boss::Boneskinner as u16,
    WhisperOfJormag = Boss::WhisperOfJormag as u16,
}

impl Encounter {
    /// Return all possible bosses that can appear in this encounter.
    ///
    /// This returns the possible boss IDs, not actual agents. For a similar function check
    /// [`Log::boss_agents`][crate::Log::boss_agents].
    ///
    /// Note that not all of them have to be present in a log, for example if the encounter stopped
    /// before all of them spawned.
    pub fn bosses(self) -> &'static [Boss] {
        match self {
            Encounter::ValeGuardian => &[Boss::ValeGuardian],
            Encounter::Gorseval => &[Boss::Gorseval],
            Encounter::Sabetha => &[Boss::Sabetha],
            Encounter::Slothasor => &[Boss::Slothasor],
            Encounter::Matthias => &[Boss::Matthias],
            Encounter::KeepConstruct => &[Boss::KeepConstruct],
            Encounter::Xera => &[Boss::Xera, Boss::Xera2],
            Encounter::Cairn => &[Boss::Cairn],
            Encounter::MursaatOverseer => &[Boss::MursaatOverseer],
            Encounter::Samarog => &[Boss::Samarog],
            Encounter::Deimos => &[Boss::Deimos],
            Encounter::SoullessHorror => &[Boss::SoullessHorror],
            Encounter::VoiceInTheVoid => &[Boss::Dhuum],
            Encounter::ConjuredAmalgamate => &[Boss::ConjuredAmalgamate],
            Encounter::TwinLargos => &[Boss::Nikare, Boss::Kenut],
            Encounter::Qadim => &[Boss::Qadim],
            Encounter::CardinalAdina => &[Boss::CardinalAdina],
            Encounter::CardinalSabir => &[Boss::CardinalSabir],
            Encounter::QadimThePeerless => &[Boss::QadimThePeerless],
            Encounter::Ai => &[Boss::Ai],
            Encounter::Skorvald => &[Boss::Skorvald],
            Encounter::Artsariiv => &[Boss::Artsariiv],
            Encounter::Arkk => &[Boss::Arkk],
            Encounter::MAMA => &[Boss::MAMA],
            Encounter::Siax => &[Boss::Siax],
            Encounter::Ensolyss => &[Boss::Ensolyss],
            Encounter::IcebroodConstruct => &[Boss::IcebroodConstruct],
            Encounter::SuperKodanBrothers => &[Boss::VoiceOfTheFallen, Boss::ClawOfTheFallen],
            Encounter::FraenirOfJormag => &[Boss::FraenirOfJormag],
            Encounter::Boneskinner => &[Boss::Boneskinner],
            Encounter::WhisperOfJormag => &[Boss::WhisperOfJormag],
        }
    }
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

/// Enum containing all boss IDs.
///
/// For a high-level event categorization, take a look at the [`Encounter`] enum. The IDs listed
/// here are for a more fine-grained control, e.g. if you specifically need to differentiate
/// between Nikare and Kenut in the Twin Largos encounter.
///
/// This enum is non-exhaustive to ensure that future bosses can be added without
/// inducing a breaking change.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive)]
#[non_exhaustive]
#[repr(u16)]
pub enum Boss {
    // Wing 1
    /// Vale Guardian, first boss of Spirit Vale.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Vale_Guardian)
    ValeGuardian = 0x3C4E,
    /// Gorseval, second boss of Spirit Vale.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Gorseval_the_Multifarious)
    Gorseval = 0x3C45,
    /// Sabetha, third boss of Spirit Vale.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Sabetha_the_Saboteur)
    Sabetha = 0x3C0F,

    // Wing 2
    /// Slothasor, first boss of Salvation Pass.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Slothasor)
    Slothasor = 0x3EFB,
    /// Matthias, third boss of Salvation Pass.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Matthias_Gabrel)
    Matthias = 0x3EF3,

    // Wing 3
    /// Keep Construct, second boss of the Stronghold of the Faithful.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Keep_Construct)
    KeepConstruct = 0x3F6B,
    /// Xera, third boss of the Stronghold of the Faithful.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Xera)
    Xera = 0x3F76,
    /// ID for Xera in the second phase.
    ///
    /// The original Xera will despawn, and after the tower phase, a separate spawn will take over.
    /// This new Xera will have [`Boss::Xera2`] as ID. Care needs to be taken when calculating boss
    /// damage on this encounter, as both Xeras have to be taken into account.
    Xera2 = 0x3F9E,

    // Wing 4
    /// Cairn, first boss of the Bastion of the Penitent.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Cairn_the_Indomitable)
    Cairn = 0x432A,
    /// Mursaat Overseer, second boss of the Bastion of the Penitent.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Mursaat_Overseer)
    MursaatOverseer = 0x4314,
    /// Samarog, third boss of the Bastion of the Penitent.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Samarog)
    Samarog = 0x4324,
    /// Deimos, fourth boss of the Bastion of the Penitent.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Deimos)
    Deimos = 0x4302,

    // Wing 5
    /// Soulless Horror, first boss of the Hall of Chains.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Soulless_Horror)
    SoullessHorror = 0x4D37,
    /// Dhuum, second boss of the Hall of Chains.
    ///
    /// The encounter to this boss is called [Voice in the Void][Encounter::VoiceInTheVoid].
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Dhuum)
    Dhuum = 0x4BFA,

    // Wing 6
    /// Conjured Amalgamate, first boss of Mythwright Gambit.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Conjured_Amalgamate)
    ConjuredAmalgamate = 0xABC6,
    /// Nikare, part of the [Twin Largos][Encounter::TwinLargos] encounter in Mythwright Gamit.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Nikare)
    Nikare = 0x5271,
    /// Kenut, part of the [Twin Largos][Encounter::TwinLargos] encounter in Mythwright Gamit.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Nikare)
    Kenut = 0x5261,
    /// Qadim, third boss in Mythwright Gambit.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Qadim)
    Qadim = 0x51C6,

    // Wing 7
    /// Cardinal Adina, one of the first two bosses in the Key of Ahdashim.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Cardinal_Adina)
    CardinalAdina = 0x55F6,
    /// Cardinal Sabir, one of the first two bosses in the Key of Ahdashim.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Cardinal_Sabir)
    CardinalSabir = 0x55CC,
    /// Qadim the Peerless, third boss in the Key of Ahdashim.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Qadim_the_Peerless)
    QadimThePeerless = 0x55F0,

    // 100 CM (Sunqua Peak)
    /// Ai, Keeper of the Peak, boss of the Sunqua Peak CM fractal.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Ai,_Keeper_of_the_Peak)
    Ai = 0x5AD6,

    // 99 CM (Shattered Observatory)
    /// Skorvald the Shattered, first boss in the Shattered Observatory.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Skorvald_the_Shattered)
    Skorvald = 0x44E0,
    /// Artsariiv, second boss in the Shattered Observatory CM.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Artsariiv)
    Artsariiv = 0x461D,
    /// Arkk, third boss in the Shattered Observatory.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Arkk)
    Arkk = 0x455F,

    // 98 CM (Nightmare)
    /// MAMA, first boss in the Nightmare CM fractal.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/MAMA)
    MAMA = 0x427D,
    /// Siax the Corrupted, second boss in the Nightmare CM fractal.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Siax_the_Corrupted)
    Siax = 0x4284,
    /// Ensolyss of the Endless Torment, third boss in the Nightmare CM fractal.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Ensolyss_of_the_Endless_Torment)
    Ensolyss = 0x4234,

    // Strike missions
    /// Legendary Icebrood Construct, boss of the Shiverpeaks Pass strike mission.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Legendary_Icebrood_Construct)
    IcebroodConstruct = 0x568A,
    /// The Voice of the Fallen, part of the Voice of the Fallen and Claw of the Fallen strike
    /// mission.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Voice_of_the_Fallen)
    VoiceOfTheFallen = 0x5747,
    /// The Claw of the Fallen, part of the Voice of the Fallen and Claw of the Fallen strike
    /// mission.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Claw_of_the_Fallen)
    ClawOfTheFallen = 0x57D1,
    /// The Fraenir of Jormag, boss of the Fraenir of Jormag strike mission.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Fraenir_of_Jormag)
    FraenirOfJormag = 0x57DC,
    /// The Boneskinner, boss of the Boneskinner strike mission.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Boneskinner)
    Boneskinner = 0x57F9,
    /// The Whisper of Jormag, boss of the Whisper of Jormag strike mission.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Whisper_of_Jormag)
    WhisperOfJormag = 0x58B7,
}

impl Boss {
    /// Get the encounter ID in which this boss can appear.
    ///
    /// This is the counterpart to [`Encounter::bosses`].
    pub fn encounter(self) -> Encounter {
        match self {
            Boss::ValeGuardian => Encounter::ValeGuardian,
            Boss::Gorseval => Encounter::Gorseval,
            Boss::Sabetha => Encounter::Sabetha,
            Boss::Slothasor => Encounter::Slothasor,
            Boss::Matthias => Encounter::Matthias,
            Boss::KeepConstruct => Encounter::KeepConstruct,
            Boss::Xera => Encounter::Xera,
            Boss::Xera2 => Encounter::Xera,
            Boss::Cairn => Encounter::Cairn,
            Boss::MursaatOverseer => Encounter::MursaatOverseer,
            Boss::Samarog => Encounter::Samarog,
            Boss::Deimos => Encounter::Deimos,
            Boss::SoullessHorror => Encounter::SoullessHorror,
            Boss::Dhuum => Encounter::VoiceInTheVoid,
            Boss::ConjuredAmalgamate => Encounter::ConjuredAmalgamate,
            Boss::Nikare => Encounter::TwinLargos,
            Boss::Kenut => Encounter::TwinLargos,
            Boss::Qadim => Encounter::Qadim,
            Boss::CardinalAdina => Encounter::CardinalAdina,
            Boss::CardinalSabir => Encounter::CardinalSabir,
            Boss::QadimThePeerless => Encounter::QadimThePeerless,
            Boss::Ai => Encounter::Ai,
            Boss::Skorvald => Encounter::Skorvald,
            Boss::Artsariiv => Encounter::Artsariiv,
            Boss::Arkk => Encounter::Arkk,
            Boss::MAMA => Encounter::MAMA,
            Boss::Siax => Encounter::Siax,
            Boss::Ensolyss => Encounter::Ensolyss,
            Boss::IcebroodConstruct => Encounter::IcebroodConstruct,
            Boss::VoiceOfTheFallen => Encounter::SuperKodanBrothers,
            Boss::ClawOfTheFallen => Encounter::SuperKodanBrothers,
            Boss::FraenirOfJormag => Encounter::FraenirOfJormag,
            Boss::Boneskinner => Encounter::Boneskinner,
            Boss::WhisperOfJormag => Encounter::WhisperOfJormag,
        }
    }
}

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
