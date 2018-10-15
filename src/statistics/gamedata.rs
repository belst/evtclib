//! This module contains some game data that is necessary to correctly calculate
//! some statistics.
use super::boon::{BoonQueue, BoonType};

/// Enum containing all bosses with their IDs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, FromPrimitive)]
pub enum Boss {
    ValeGuardian = 0x3C4E,
    Gorseval = 0x3C45,
    Sabetha = 0x3C0F,

    Slothasor = 0x3EFB,
    Matthias = 0x3EF3,

    KeepConstruct = 0x3F6B,
    /// Xera ID for phase 1.
    ///
    /// This is only half of Xera's ID, as there will be a second agent for the
    /// second phase. This agent will have another ID, see
    /// [`XERA_PHASE2_ID`](constant.XERA_PHASE2_ID.html).
    Xera = 0x3F76,

    Cairn = 0x432A,
    MursaatOverseer = 0x4314,
    Samarog = 0x4324,
    Deimos = 0x3202,

    SoullessHorror = 0x4D37,
    Dhuum = 0x4BFA,

    ConjuredAmalgamate = 0xABC6,
    LargosTwins = 0x5271,
    Qadim = 0x51C6,
}

/// ID for Xera in the second phase.
///
/// The original Xera will despawn, and after the tower phase, a separate spawn
/// will take over. This new Xera will have this ID. Care needs to be taken when
/// calculating boss damage on this encounter, as both Xeras have to be taken
/// into account.
pub const XERA_PHASE2_ID: u16 = 0x3F9E;

/// Contains a boon.
///
/// Fields:
/// * boon id
/// * name (english) (just for easier debugging)
/// * maximum number of stacks
/// * boon type (intensity or duration)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boon(pub u32, pub &'static str, pub u32, pub BoonType);

impl Boon {
    pub fn create_queue(&self) -> BoonQueue {
        BoonQueue::new(self.2, self.3)
    }
}

/// A list of all boons (and conditions)
pub static BOONS: &[Boon] = &[
    // Standard boons.
    // Boon queue sizes taken from the wiki:
    // https://wiki.guildwars2.com/wiki/Effect_stacking
    // IDs from wiki and skilldef.log:
    // https://www.deltaconnected.com/arcdps/evtc/

    // Duration based
    Boon(743, "Aegis", 5, BoonType::Duration),
    Boon(30328, "Alacrity", 9, BoonType::Duration),
    Boon(725, "Fury", 9, BoonType::Duration),
    Boon(717, "Protection", 5, BoonType::Duration),
    Boon(718, "Regeneration", 5, BoonType::Duration),
    Boon(26980, "Resistance", 5, BoonType::Duration),
    Boon(873, "Retaliation", 5, BoonType::Duration),
    Boon(719, "Swiftness", 9, BoonType::Duration),
    Boon(1187, "Quickness", 5, BoonType::Duration),
    Boon(726, "Vigor", 5, BoonType::Duration),
    // Intensity based
    Boon(740, "Might", 25, BoonType::Intensity),
    Boon(1122, "Stability", 25, BoonType::Intensity),
    // Standard conditions.
    // Duration based
    Boon(720, "Blinded", 5, BoonType::Duration),
    Boon(722, "Chilled", 5, BoonType::Duration),
    Boon(721, "Crippled", 5, BoonType::Duration),
    Boon(791, "Fear", 5, BoonType::Duration),
    Boon(727, "Immobile", 3, BoonType::Duration),
    Boon(26766, "Slow", 3, BoonType::Duration),
    Boon(742, "Weakness", 3, BoonType::Duration),
    // Intensity based
    Boon(736, "Bleeding", 1500, BoonType::Intensity),
    Boon(737, "Burning", 1500, BoonType::Intensity),
    Boon(861, "Confusion", 1500, BoonType::Intensity),
    Boon(723, "Poison", 1500, BoonType::Intensity),
    Boon(19426, "Torment", 1500, BoonType::Intensity),
    Boon(738, "Vulnerability", 25, BoonType::Intensity),
];

pub fn get_boon(boon_id: u32) -> Option<&'static Boon> {
    BOONS.iter().find(|b| b.0 == boon_id)
}

/// Contains pre-defined triggers for boss mechanics.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Trigger {
    /// Triggers when the given boon is applied to the player.
    BoonPlayer(u32),
    /// Triggers when the given boon is applied to the boss.
    BoonBoss(u32),
    /// Triggers when the given skill is used by a player.
    SkillByPlayer(u32),
    /// Triggers when the given skill is used on a player.
    SkillOnPlayer(u32),
    /// Triggers when the given boon is stripped from an enemy.
    BoonStripped(u32),
    /// Triggers when the given entity spawned.
    Spawn(u16),
    /// Triggers when the boss finishes channeling the given skill.
    ChannelComplete(u32),
}

/// Struct describing a boss mechanic.
///
/// Fields:
/// * Boss id that this mechanic belongs to.
/// * How the mechanic is triggered.
/// * Technical term for the mechanic (for debugging purposes).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mechanic(pub u16, pub Trigger, pub &'static str);

impl Mechanic {
    #[inline]
    pub fn boss_id(&self) -> u16 {
        self.0
    }

    #[inline]
    pub fn trigger(&self) -> &Trigger {
        &self.1
    }

    #[inline]
    pub fn name(&self) -> &'static str {
        self.2
    }
}

macro_rules! mechanics {
    ( $( $boss_id:expr => [ $($name:expr => $trigger:expr,)* ], )* ) => {
        &[
            $( $(Mechanic($boss_id as u16, $trigger, $name)),* ),*
         ]
    }
}

/// A slice of all mechanics that we know about.
pub static MECHANICS: &[Mechanic] = mechanics! {
    // Wing 1
    Boss::ValeGuardian => [
        // Teleport:
        "Unstable Magic Spike" => Trigger::SkillOnPlayer(31392),
    ],
    Boss::Gorseval => [
        // Slam
        "Spectral Impact" => Trigger::SkillOnPlayer(31875),
        // Egg
        "Ghastly Prison" => Trigger::BoonPlayer(31623),
    ],
    Boss::Sabetha => [
        // Took the launch pad
        "Shell-Shocked" => Trigger::BoonPlayer(34108),
    ],

    // Wing 4
    Boss::Samarog => [
        "Prisoner Sweep" => Trigger::SkillOnPlayer(38168),
        "Shockwave" => Trigger::SkillOnPlayer(37996),
    ],
};

/// Get all mechanics that belong to the given boss.
pub fn get_mechanics(boss_id: u16) -> Vec<&'static Mechanic> {
    MECHANICS.iter().filter(|m| m.0 == boss_id).collect()
}
