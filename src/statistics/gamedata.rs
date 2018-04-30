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
    // Queue size for duration based boons are wonky, more or less guess work.
    Boon(717, "Protection", 5, BoonType::Duration),
    Boon(718, "Regeneration", 5, BoonType::Duration),
    Boon(719, "Swiftness", 5, BoonType::Duration),
    Boon(725, "Fury", 5, BoonType::Duration),
    Boon(726, "Vigor", 5, BoonType::Duration),
    Boon(740, "Might", 25, BoonType::Intensity),
    Boon(743, "Aegis", 5, BoonType::Duration),
    Boon(1187, "Quickness", 5, BoonType::Duration),
    Boon(30328, "Alacrity", 9, BoonType::Duration),
];

pub fn get_boon(boon_id: u16) -> Option<&'static Boon> {
    BOONS.iter().find(|b| b.0 == boon_id)
}
