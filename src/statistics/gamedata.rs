//! This module contains some game data that is necessary to correctly calculate
//! some statistics.
use super::boon::{BoonQueue, BoonType};

/// Contains a boon.
///
/// Fields:
/// * boon id
/// * name (english) (just for easier debugging)
/// * maximum number of stacks
/// * boon type (intensity or duration)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boon(pub u16, pub &'static str, pub u32, pub BoonType);

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

pub fn get_boon(boon_id: u16) -> Option<&'static Boon> {
    BOONS.iter().find(|b| b.0 == boon_id)
}
