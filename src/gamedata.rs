//! This module contains some low-level game data, such as different boss IDs.
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};
use thiserror::Error;

/// The game mode in which a log was produced.
///
/// Note that the distinction made here is relatively arbitrary, but hopefully still useful. In
/// Guild Wars 2 terms, there is no clear definition of what a "game mode" is.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameMode {
    /// The log is from a raid encounter.
    Raid,
    /// The log is from a fractal fight.
    Fractal,
    /// The log is from a strike mission.
    Strike,
    /// The log is from a training golem.
    Golem,
    /// The log is from a world-versus-world fight.
    WvW,
}

/// Error for when converting a string to a game mode fails.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Error)]
#[error("Invalid encounter identifier: {0}")]
pub struct ParseGameModeError(String);

impl FromStr for GameMode {
    type Err = ParseGameModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        match &lower as &str {
            "raid" => Ok(GameMode::Raid),
            "fractal" => Ok(GameMode::Fractal),
            "strike" => Ok(GameMode::Strike),
            "golem" => Ok(GameMode::Golem),
            "wvw" => Ok(GameMode::WvW),

            _ => Err(ParseGameModeError(s.to_owned())),
        }
    }
}

static DRAGONVOID_IDS: &[u16] = &[Encounter::Dragonvoid as u16, 0xA9E0, 0x5F37];

/// Enum containing all encounters with their IDs.
///
/// An encounter is a fight or event for which a log can exist. An encounter consists of no, one or
/// multiple bosses. Most encounters map 1:1 to a boss (like Vale Guardian), however there are some
/// encounters with multiple bosses (like Twin Largos), and even encounters without bosses (like
/// the River of Souls).
///
/// Note that the meaning of "encounter" in the Guild Wars 2 Wiki is not the same as what
/// [`Encounter`] represents. In many cases, they match, however there are some encounters which
/// have no associated [`Encounter`] (like Spirit Run or Escort) and some cases where multiple
/// [`Encounter`]s exist for a single encounter (like the Statues of Grenth encounter in the Hall
/// of Chains).
///
/// This enum is non-exhaustive to ensure that future encounters can be added without
/// inducing a breaking change.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// The "Protect the caged prisoners" event in Salvation Pass.
    ///
    /// Consists of [`Boss::Berg`], [`Boss::Zane`] and [`Boss::Narella`].
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Protect_the_caged_prisoners)
    // Berg is the first encounter, which is why the logs are saved as "Berg".
    BanditTrio = Boss::Berg as u16,
    Matthias = Boss::Matthias as u16,

    // Wing 3
    KeepConstruct = Boss::KeepConstruct as u16,
    /// The "Traverse the Twisted Castle" encounter, between Keep Construct and Xera.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Traverse_the_Twisted_Castle)
    TwistedCastle = 0x3F77,
    Xera = Boss::Xera as u16,

    // Wing 4
    Cairn = Boss::Cairn as u16,
    MursaatOverseer = Boss::MursaatOverseer as u16,
    Samarog = Boss::Samarog as u16,
    Deimos = Boss::Deimos as u16,

    // Wing 5
    SoullessHorror = Boss::SoullessHorror as u16,
    /// The River of Souls is the Desmina escort event and an encounter that does not have a boss.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Traverse_the_River_of_Souls)
    RiverOfSouls = 0x4D74,
    BrokenKing = Boss::BrokenKing as u16,
    EaterOfSouls = Boss::EaterOfSouls as u16,
    /// The Statue of Darkness consists of killing the Eye of Judgment and the Eye of Fate.
    ///
    /// Colloquially known as just "eyes".
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Statue_of_Darkness)
    StatueOfDarkness = Boss::EyeOfJudgment as u16,

    VoiceInTheVoid = Boss::Dhuum as u16,

    // Wing 6
    ConjuredAmalgamate = Boss::ConjuredAmalgamate as u16,
    TwinLargos = Boss::Nikare as u16,
    Qadim = Boss::Qadim as u16,

    // Wing 7
    CardinalAdina = Boss::CardinalAdina as u16,
    CardinalSabir = Boss::CardinalSabir as u16,
    QadimThePeerless = Boss::QadimThePeerless as u16,

    // Training area
    StandardKittyGolem = Boss::StandardKittyGolem as u16,
    MediumKittyGolem = Boss::MediumKittyGolem as u16,
    LargeKittyGolem = Boss::LargeKittyGolem as u16,

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
    CaptainMaiTrin = Boss::CaptainMaiTrin as u16,
    Ankka = Boss::Ankka as u16,
    MinisterLi = Boss::MinisterLi as u16,
    Dragonvoid = 0x0562,
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
            Encounter::BanditTrio => &[Boss::Berg, Boss::Zane, Boss::Narella],
            Encounter::Matthias => &[Boss::Matthias],
            Encounter::KeepConstruct => &[Boss::KeepConstruct],
            Encounter::TwistedCastle => &[],
            Encounter::Xera => &[Boss::Xera, Boss::Xera2],
            Encounter::Cairn => &[Boss::Cairn],
            Encounter::MursaatOverseer => &[Boss::MursaatOverseer],
            Encounter::Samarog => &[Boss::Samarog],
            Encounter::Deimos => &[Boss::Deimos],
            Encounter::SoullessHorror => &[Boss::SoullessHorror],
            Encounter::RiverOfSouls => &[],
            Encounter::BrokenKing => &[Boss::BrokenKing],
            Encounter::EaterOfSouls => &[Boss::EaterOfSouls],
            Encounter::StatueOfDarkness => &[Boss::EyeOfJudgment, Boss::EyeOfFate],
            Encounter::VoiceInTheVoid => &[Boss::Dhuum],
            Encounter::ConjuredAmalgamate => &[Boss::ConjuredAmalgamate],
            Encounter::TwinLargos => &[Boss::Nikare, Boss::Kenut],
            Encounter::Qadim => &[Boss::Qadim],
            Encounter::CardinalAdina => &[Boss::CardinalAdina],
            Encounter::CardinalSabir => &[Boss::CardinalSabir],
            Encounter::QadimThePeerless => &[Boss::QadimThePeerless],
            Encounter::StandardKittyGolem => &[Boss::StandardKittyGolem],
            Encounter::MediumKittyGolem => &[Boss::MediumKittyGolem],
            Encounter::LargeKittyGolem => &[Boss::LargeKittyGolem],
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
            Encounter::CaptainMaiTrin => &[Boss::CaptainMaiTrin],
            Encounter::Ankka => &[Boss::Ankka],
            Encounter::MinisterLi => &[Boss::MinisterLi],
            Encounter::Dragonvoid => &[],
        }
    }

    /// Converts a combat ID as given in the arcdps header into the correct encounter.
    ///
    /// This properly takes care of encounters with multiple bosses or which could be saved as
    /// multiple bosses.
    ///
    /// ```
    /// # use evtclib::gamedata::Encounter;
    /// assert_eq!(Encounter::from_header_id(0x3C4E), Some(Encounter::ValeGuardian));
    /// assert_eq!(Encounter::from_header_id(0x5261), Some(Encounter::TwinLargos));
    /// ```
    #[inline]
    pub fn from_header_id(id: u16) -> Option<Encounter> {
        // For the encounters without boss, we do it manually.
        match id {
            _ if id == Encounter::TwistedCastle as u16 => Some(Encounter::TwistedCastle),
            _ if id == Encounter::RiverOfSouls as u16 => Some(Encounter::RiverOfSouls),
            _ if DRAGONVOID_IDS.contains(&id) => Some(Encounter::Dragonvoid),
            _ => Boss::from_u16(id).map(Boss::encounter),
        }
    }

    /// Returns the game mode of the encounter.
    ///
    /// This is one of [`GameMode::Raid`], [`GameMode::Fractal`], [`GameMode::Golem`] or
    /// [`GameMode::Strike`].
    pub fn game_mode(self) -> GameMode {
        use Encounter::*;
        match self {
            MAMA | Siax | Ensolyss | Skorvald | Artsariiv | Arkk | Ai => GameMode::Fractal,

            ValeGuardian | Gorseval | Sabetha | Slothasor | BanditTrio | Matthias
            | KeepConstruct | TwistedCastle | Xera | Cairn | MursaatOverseer | Samarog | Deimos
            | SoullessHorror | RiverOfSouls | BrokenKing | EaterOfSouls | StatueOfDarkness
            | VoiceInTheVoid | ConjuredAmalgamate | TwinLargos | Qadim | CardinalAdina
            | CardinalSabir | QadimThePeerless => GameMode::Raid,

            IcebroodConstruct | Boneskinner | SuperKodanBrothers | FraenirOfJormag
            | WhisperOfJormag | CaptainMaiTrin | Ankka | MinisterLi | Dragonvoid => {
                GameMode::Strike
            }

            StandardKittyGolem | MediumKittyGolem | LargeKittyGolem => GameMode::Golem,
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
        // Parsing an encounter is in most cases the same as parsing a boss, as the encounters map
        // 1:1 to a boss. For the special cases where the encounter as such has a specific name
        // (such as Twin Largos), this parses strictly more bosses (so "Kenut" would be parsed as
        // Encounter::TwinLargos, which is fine). The special cases are then added later (so that
        // "Twin Largos" also is parsed as Encounter::TwinLargos).
        if let Ok(boss) = Boss::from_str(s) {
            return Ok(boss.encounter());
        }
        let lower = s.to_lowercase();
        match &lower as &str {
            "trio" | "bandit trio" => Ok(Encounter::BanditTrio),
            "tc" | "twisted castle" => Ok(Encounter::TwistedCastle),
            "river" | "river of souls" => Ok(Encounter::RiverOfSouls),
            "eyes" | "statue of darkness" => Ok(Encounter::StatueOfDarkness),
            "largos" | "twins" | "largos twins" | "twin largos" => Ok(Encounter::TwinLargos),
            "kodans" | "super kodan brothers" => Ok(Encounter::SuperKodanBrothers),
            "dragonvoid" | "the dragonvoid" => Ok(Encounter::Dragonvoid),

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
            Encounter::BanditTrio => "Bandit Trio",
            Encounter::Matthias => "Matthias Gabrel",
            Encounter::KeepConstruct => "Keep Construct",
            Encounter::TwistedCastle => "Twisted Castle",
            Encounter::Xera => "Xera",
            Encounter::Cairn => "Cairn the Indomitable",
            Encounter::MursaatOverseer => "Mursaat Overseer",
            Encounter::Samarog => "Samarog",
            Encounter::Deimos => "Deimos",
            Encounter::SoullessHorror => "Soulless Horror",
            Encounter::RiverOfSouls => "River of Souls",
            Encounter::BrokenKing => "Broken King",
            Encounter::EaterOfSouls => "Eater of Souls",
            Encounter::StatueOfDarkness => "Statue of Darkness",
            Encounter::VoiceInTheVoid => "Voice in the Void",
            Encounter::ConjuredAmalgamate => "Conjured Amalgamate",
            Encounter::TwinLargos => "Twin Largos",
            Encounter::Qadim => "Qadim",
            Encounter::CardinalAdina => "Cardinal Adina",
            Encounter::CardinalSabir => "Cardinal Sabir",
            Encounter::QadimThePeerless => "Qadim the Peerless",
            Encounter::StandardKittyGolem => "Standard Kitty Golem",
            Encounter::MediumKittyGolem => "Medium Kitty Golem",
            Encounter::LargeKittyGolem => "Large Kitty Golem",
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
            Encounter::CaptainMaiTrin => "Captain Mai Trin",
            Encounter::Ankka => "Ankka",
            Encounter::MinisterLi => "Minister Li",
            Encounter::Dragonvoid => "The Dragonvoid",
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// Berg, part of the "Bandit Trio" encounter.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Berg)
    Berg = 0x3ED8,
    /// Zane, part of the "Bandit Trio" encounter.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Zane)
    Zane = 0x3F09,
    /// Narella, part of the "Bandit Trio" encounter.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Narella)
    Narella = 0x3EFD,
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
    /// Broken King, part of the Statues of Grenth event in the Hall of Chains.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Broken_King)
    BrokenKing = 0x4CEB,
    /// Eater of Souls, part of the Statues of Grenth event in the Hall of Chains.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Eater_of_Souls_(Hall_of_Chains))
    EaterOfSouls = 0x4C50,
    /// The Eye of Judgment, part of the Statue of Darkness event in the Hall of Chains.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Eye_of_Judgment)
    EyeOfJudgment = 0x4CC3,
    /// The Eye of Fate, part of the Statue of Darkness event in the Hall of Chains.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Eye_of_Fate)
    EyeOfFate = 0x4D84,
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

    // The training area
    /// The standard training golem, available in the Special Forces Training Area.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Standard_Kitty_Golem)
    StandardKittyGolem = 0x3F47,
    /// The medium training golem, available in the Special Forces Training Area.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Medium_Kitty_Golem)
    MediumKittyGolem = 0x4CBD,
    /// The large kitty golem available in the Special Forces Training Area.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Large_Kitty_Golem)
    LargeKittyGolem = 0x4CDC,

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
    /// Captain Mai Trin, boss of the Aetherblade Hideout strike mission.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Strike_Mission:_Aetherblade_Hideout)
    CaptainMaiTrin = 0x5DE1,
    /// Ankka, boss in the Xunlai Jade Junkyard.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Strike_Mission:_Xunlai_Jade_Junkyard)
    Ankka = 0x5D95,
    /// Minister Li, boss in the Kaineng Overlook.
    ///
    /// [Guild Wars 2 Wiki](https://wiki.guildwars2.com/wiki/Strike_Mission:_Kaineng_Overlook)
    MinisterLi = 0x5FA5,
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
            Boss::Berg | Boss::Zane | Boss::Narella => Encounter::BanditTrio,
            Boss::Matthias => Encounter::Matthias,
            Boss::KeepConstruct => Encounter::KeepConstruct,
            Boss::Xera => Encounter::Xera,
            Boss::Xera2 => Encounter::Xera,
            Boss::Cairn => Encounter::Cairn,
            Boss::MursaatOverseer => Encounter::MursaatOverseer,
            Boss::Samarog => Encounter::Samarog,
            Boss::Deimos => Encounter::Deimos,
            Boss::SoullessHorror => Encounter::SoullessHorror,
            Boss::BrokenKing => Encounter::BrokenKing,
            Boss::EaterOfSouls => Encounter::EaterOfSouls,
            Boss::EyeOfJudgment => Encounter::StatueOfDarkness,
            Boss::EyeOfFate => Encounter::StatueOfDarkness,
            Boss::Dhuum => Encounter::VoiceInTheVoid,
            Boss::ConjuredAmalgamate => Encounter::ConjuredAmalgamate,
            Boss::Nikare => Encounter::TwinLargos,
            Boss::Kenut => Encounter::TwinLargos,
            Boss::Qadim => Encounter::Qadim,
            Boss::CardinalAdina => Encounter::CardinalAdina,
            Boss::CardinalSabir => Encounter::CardinalSabir,
            Boss::QadimThePeerless => Encounter::QadimThePeerless,
            Boss::StandardKittyGolem => Encounter::StandardKittyGolem,
            Boss::MediumKittyGolem => Encounter::MediumKittyGolem,
            Boss::LargeKittyGolem => Encounter::LargeKittyGolem,
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
            Boss::CaptainMaiTrin => Encounter::CaptainMaiTrin,
            Boss::Ankka => Encounter::Ankka,
            Boss::MinisterLi => Encounter::MinisterLi,
        }
    }
}

/// Error for when converting a string to an encounter fails.
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
            "berg" => Ok(Boss::Berg),
            "zane" => Ok(Boss::Zane),
            "narella" => Ok(Boss::Narella),
            "matthias" => Ok(Boss::Matthias),

            "kc" | "keep construct" => Ok(Boss::KeepConstruct),
            "xera" => Ok(Boss::Xera),

            "cairn" => Ok(Boss::Cairn),
            "mo" | "mursaat overseer" => Ok(Boss::MursaatOverseer),
            "sam" | "sama" | "samarog" => Ok(Boss::Samarog),
            "deimos" => Ok(Boss::Deimos),

            "desmina" | "sh" | "soulless horror" => Ok(Boss::SoullessHorror),
            "broken king" => Ok(Boss::BrokenKing),
            "eater" | "eater of souls" => Ok(Boss::EaterOfSouls),
            "eye of judgment" => Ok(Boss::EyeOfJudgment),
            "eye of fate" => Ok(Boss::EyeOfFate),
            "dhuum" | "voice in the void" => Ok(Boss::Dhuum),

            "ca" | "conjured amalgamate" => Ok(Boss::ConjuredAmalgamate),
            "nikare" => Ok(Boss::Nikare),
            "kenut" => Ok(Boss::Kenut),
            "qadim" => Ok(Boss::Qadim),

            "adina" | "cardinal adina" => Ok(Boss::CardinalAdina),
            "sabir" | "cardinal sabir" => Ok(Boss::CardinalSabir),
            "qadimp" | "peerless qadim" | "qadim the peerless" => Ok(Boss::QadimThePeerless),

            "standard golem" | "standard kitty golem" => Ok(Boss::StandardKittyGolem),
            "medium golem" | "medium kitty golem" => Ok(Boss::MediumKittyGolem),
            "large golem" | "large kitty golem" => Ok(Boss::LargeKittyGolem),

            "ai" | "ai keeper of the peak" => Ok(Boss::Ai),

            "skorvald" => Ok(Boss::Skorvald),
            "artsariiv" => Ok(Boss::Artsariiv),
            "arkk" => Ok(Boss::Arkk),

            "mama" => Ok(Boss::MAMA),
            "siax" => Ok(Boss::Siax),
            "ensolyss" | "ensolyss of the endless torment" => Ok(Boss::Ensolyss),

            "icebrood" | "icebrood construct" => Ok(Boss::IcebroodConstruct),
            "voice" | "voice of the fallen" => Ok(Boss::VoiceOfTheFallen),
            "claw" | "claw of the fallen" => Ok(Boss::ClawOfTheFallen),
            "fraenir" | "fraenir of jormag" => Ok(Boss::FraenirOfJormag),
            "boneskinner" => Ok(Boss::Boneskinner),
            "whisper" | "whisper of jormag" => Ok(Boss::WhisperOfJormag),
            "captain mai trin" | "mai trin" | "mai" => Ok(Boss::CaptainMaiTrin),
            "ankka" => Ok(Boss::Ankka),
            "minister li" | "li" => Ok(Boss::MinisterLi),

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
            Boss::Berg => "Berg",
            Boss::Zane => "Zane",
            Boss::Narella => "Narella",
            Boss::Matthias => "Matthias Gabrel",
            Boss::KeepConstruct => "Keep Construct",
            Boss::Xera => "Xera",
            Boss::Xera2 => "Xera",
            Boss::Cairn => "Cairn the Indomitable",
            Boss::MursaatOverseer => "Mursaat Overseer",
            Boss::Samarog => "Samarog",
            Boss::Deimos => "Deimos",
            Boss::SoullessHorror => "Soulless Horror",
            Boss::BrokenKing => "Broken King",
            Boss::EaterOfSouls => "Eater of Souls",
            Boss::EyeOfJudgment => "Eye of Judgment",
            Boss::EyeOfFate => "Eye of Fate",
            Boss::Dhuum => "Dhuum",
            Boss::ConjuredAmalgamate => "Conjured Amalgamate",
            Boss::Nikare => "Nikare",
            Boss::Kenut => "Kenut",
            Boss::Qadim => "Qadim",
            Boss::CardinalAdina => "Cardinal Adina",
            Boss::CardinalSabir => "Cardinal Sabir",
            Boss::QadimThePeerless => "Qadim the Peerless",
            Boss::StandardKittyGolem => "Standard Kitty Golem",
            Boss::MediumKittyGolem => "Medium Kitty Golem",
            Boss::LargeKittyGolem => "Large Kitty Golem",
            Boss::Ai => "Ai Keeper of the Peak",
            Boss::Skorvald => "Skorvald the Shattered",
            Boss::Artsariiv => "Artsariiv",
            Boss::Arkk => "Arkk",
            Boss::MAMA => "MAMA",
            Boss::Siax => "Siax the Corrupted",
            Boss::Ensolyss => "Ensolyss of the Endless Torment",
            Boss::IcebroodConstruct => "Icebrood Construct",
            Boss::VoiceOfTheFallen => "Voice of the Fallen",
            Boss::ClawOfTheFallen => "Claw of the Fallen",
            Boss::FraenirOfJormag => "Fraenir of Jormag",
            Boss::Boneskinner => "Boneskinner",
            Boss::WhisperOfJormag => "Whisper of Jormag",
            Boss::CaptainMaiTrin => "Captain Mai Trin",
            Boss::Ankka => "Ankka",
            Boss::MinisterLi => "Minister Li",
        };
        write!(f, "{}", name)
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

    // End of Dragons elites:
    Willbender = 65,
    Bladesworn = 68,
    Mechanist = 70,
    Untamed = 72,
    Specter = 71,
    Catalyst = 67,
    Virtuoso = 66,
    Harbinger = 64,
    Vindicator = 69,
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

            "willbender" => Ok(EliteSpec::Willbender),
            "bladesworn" => Ok(EliteSpec::Bladesworn),
            "mechanist" => Ok(EliteSpec::Mechanist),
            "untamed" => Ok(EliteSpec::Untamed),
            "specter" => Ok(EliteSpec::Specter),
            "catalyst" => Ok(EliteSpec::Catalyst),
            "virtuoso" => Ok(EliteSpec::Virtuoso),
            "harbinger" => Ok(EliteSpec::Harbinger),
            "vindicator" => Ok(EliteSpec::Vindicator),

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
            EliteSpec::Willbender => "Willbender",
            EliteSpec::Bladesworn => "Bladesworn",
            EliteSpec::Mechanist => "Mechanist",
            EliteSpec::Untamed => "Untamed",
            EliteSpec::Specter => "Specter",
            EliteSpec::Catalyst => "Catalyst",
            EliteSpec::Virtuoso => "Virtuoso",
            EliteSpec::Harbinger => "Harbinger",
            EliteSpec::Vindicator => "Vindicator",
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
            Dragonhunter | Firebrand | Willbender => Profession::Guardian,
            Berserker | Spellbreaker | Bladesworn => Profession::Warrior,
            Scrapper | Holosmith | Mechanist => Profession::Engineer,
            Druid | Soulbeast | Untamed => Profession::Ranger,
            Daredevil | Deadeye | Specter => Profession::Thief,
            Tempest | Weaver | Catalyst => Profession::Elementalist,
            Chronomancer | Mirage | Virtuoso => Profession::Mesmer,
            Reaper | Scourge | Harbinger => Profession::Necromancer,
            Herald | Renegade | Vindicator => Profession::Revenant,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamemode_parsing_ok() {
        use GameMode::*;
        let tests: &[(&'static str, GameMode)] = &[
            ("raid", Raid),
            ("Raid", Raid),
            ("fractal", Fractal),
            ("Fractal", Fractal),
            ("strike", Strike),
            ("Strike", Strike),
            ("golem", Golem),
            ("Golem", Golem),
            ("wvw", WvW),
            ("WvW", WvW),
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
            ("trio", BanditTrio),
            ("bandit trio", BanditTrio),
            ("Trio", BanditTrio),
            ("berg", BanditTrio),
            ("zane", BanditTrio),
            ("narella", BanditTrio),
            ("matthias", Matthias),
            ("Matthias", Matthias),
            ("kc", KeepConstruct),
            ("KC", KeepConstruct),
            ("keep construct", KeepConstruct),
            ("Keep Construct", KeepConstruct),
            ("tc", TwistedCastle),
            ("TC", TwistedCastle),
            ("twisted castle", TwistedCastle),
            ("Twisted Castle", TwistedCastle),
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
            ("river", RiverOfSouls),
            ("River", RiverOfSouls),
            ("river of souls", RiverOfSouls),
            ("broken king", BrokenKing),
            ("Broken King", BrokenKing),
            ("eater", EaterOfSouls),
            ("eater of souls", EaterOfSouls),
            ("Eater of Souls", EaterOfSouls),
            ("eyes", StatueOfDarkness),
            ("statue of darkness", StatueOfDarkness),
            ("Statue of Darkness", StatueOfDarkness),
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
            ("Ai", Ai),
            ("ai", Ai),
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
            ("captain mai trin", CaptainMaiTrin),
            ("Captain Mai Trin", CaptainMaiTrin),
            ("mai trin", CaptainMaiTrin),
            ("Mai Trin", CaptainMaiTrin),
            ("ankka", Ankka),
            ("Ankka", Ankka),
            ("minister li", MinisterLi),
            ("Minister Li", MinisterLi),
            ("li", MinisterLi),
            ("Li", MinisterLi),
            ("dragonvoid", Dragonvoid),
            ("Dragonvoid", Dragonvoid),
            ("the dragonvoid", Dragonvoid),
            ("The Dragonvoid", Dragonvoid),
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
            ("berg", Berg),
            ("Berg", Berg),
            ("zane", Zane),
            ("Zane", Zane),
            ("narella", Narella),
            ("Narella", Narella),
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
            ("broken king", BrokenKing),
            ("Broken King", BrokenKing),
            ("eater", EaterOfSouls),
            ("eater of souls", EaterOfSouls),
            ("Eater of Souls", EaterOfSouls),
            ("eye of judgment", EyeOfJudgment),
            ("Eye of Judgment", EyeOfJudgment),
            ("eye of fate", EyeOfFate),
            ("Eye of Fate", EyeOfFate),
            ("dhuum", Dhuum),
            ("Dhuum", Dhuum),
            ("ca", ConjuredAmalgamate),
            ("conjured amalgamate", ConjuredAmalgamate),
            ("Conjured Amalgamate", ConjuredAmalgamate),
            ("kenut", Kenut),
            ("Kenut", Kenut),
            ("nikare", Nikare),
            ("Nikare", Nikare),
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
            ("Ai", Ai),
            ("ai", Ai),
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
            ("claw", ClawOfTheFallen),
            ("Claw", ClawOfTheFallen),
            ("Claw of the Fallen", ClawOfTheFallen),
            ("voice", VoiceOfTheFallen),
            ("Voice", VoiceOfTheFallen),
            ("Voice of the Fallen", VoiceOfTheFallen),
            ("whisper", WhisperOfJormag),
            ("Whisper of Jormag", WhisperOfJormag),
            ("captain mai trin", CaptainMaiTrin),
            ("Captain Mai Trin", CaptainMaiTrin),
            ("mai trin", CaptainMaiTrin),
            ("Mai Trin", CaptainMaiTrin),
            ("ankka", Ankka),
            ("Ankka", Ankka),
            ("minister li", MinisterLi),
            ("Minister Li", MinisterLi),
            ("li", MinisterLi),
            ("Li", MinisterLi),
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
            // The following are encounters, make sure we don't parse them as bosses.
            "twins",
            "kodans",
            "twin largos",
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
            ("willbender", Willbender),
            ("Willbender", Willbender),
            ("berserker", Berserker),
            ("Berserker", Berserker),
            ("spellbreaker", Spellbreaker),
            ("Spellbreaker", Spellbreaker),
            ("bladesworn", Bladesworn),
            ("Bladesworn", Bladesworn),
            ("herald", Herald),
            ("Herald", Herald),
            ("renegade", Renegade),
            ("Renegade", Renegade),
            ("vindicator", Vindicator),
            ("Vindicator", Vindicator),
            ("daredevil", Daredevil),
            ("Daredevil", Daredevil),
            ("deadeye", Deadeye),
            ("Deadeye", Deadeye),
            ("specter", Specter),
            ("Specter", Specter),
            ("scrapper", Scrapper),
            ("Scrapper", Scrapper),
            ("holosmith", Holosmith),
            ("Holosmith", Holosmith),
            ("mechanist", Mechanist),
            ("Mechanist", Mechanist),
            ("druid", Druid),
            ("Druid", Druid),
            ("soulbeast", Soulbeast),
            ("Soulbeast", Soulbeast),
            ("untamed", Untamed),
            ("Untamed", Untamed),
            ("tempest", Tempest),
            ("Tempest", Tempest),
            ("weaver", Weaver),
            ("Weaver", Weaver),
            ("catalyst", Catalyst),
            ("Catalyst", Catalyst),
            ("chronomancer", Chronomancer),
            ("Chronomancer", Chronomancer),
            ("mirage", Mirage),
            ("Mirage", Mirage),
            ("virtuoso", Virtuoso),
            ("Virtuoso", Virtuoso),
            ("reaper", Reaper),
            ("Reaper", Reaper),
            ("scourge", Scourge),
            ("Scourge", Scourge),
            ("harbinger", Harbinger),
            ("Harbinger", Harbinger),
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
