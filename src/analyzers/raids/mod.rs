//! Analyzers for raid logs.
//!
//! Most of the fights can use the [`GenericRaid`][GenericRaid] analyzer. The exception to this are
//! fights which have a Challenge Mote (Wing 4, Wing 5, Wing 6, Wing 7), and fights which need to
//! use a different method to determine their outcome (Xera, Deimos, Soulless Horror, Conjured
//! Amalgamate, Qadim).
use crate::{
    analyzers::{helpers, Analyzer, Outcome},
    Log,
};

mod w3;
pub use w3::Xera;

mod w4;
pub use w4::{Cairn, Deimos, MursaatOverseer, Samarog};

mod w5;
pub use w5::{Dhuum, SoullessHorror};

mod w6;
pub use w6::{ConjuredAmalgamate, TwinLargos, Qadim};

mod w7;
pub use w7::{CardinalAdina, CardinalSabir, QadimThePeerless};

/// A generic raid analyzer that works for bosses without special interactions.
///
/// This analyzer always returns `false` for the Challenge Mote calculation.
///
/// The outcome of the fight is determined by whether the boss agent has a death event - which
/// works for a lot of fights, but not all of them.
#[derive(Debug, Clone, Copy)]
pub struct GenericRaid<'log> {
    log: &'log Log,
}

impl<'log> GenericRaid<'log> {
    /// Create a new [`GenericRaid`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
    pub fn new(log: &'log Log) -> Self {
        GenericRaid { log }
    }
}

impl<'log> Analyzer for GenericRaid<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        false
    }

    fn outcome(&self) -> Option<Outcome> {
        Outcome::from_bool(helpers::boss_is_dead(self.log))
    }
}
