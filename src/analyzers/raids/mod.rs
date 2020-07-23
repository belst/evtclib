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
pub use w6::{ConjuredAmalgamate, LargosTwins, Qadim};

mod w7;
pub use w7::{CardinalAdina, CardinalSabir, QadimThePeerless};

/// A generic raid analyzer that works for bosses without special interactions.
#[derive(Debug, Clone, Copy)]
pub struct GenericRaid<'log> {
    log: &'log Log,
}

impl<'log> GenericRaid<'log> {
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
