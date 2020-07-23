//! Analyzers for Strike Mission logs.
use crate::{
    analyzers::{helpers, Analyzer, Outcome},
    Log,
};

#[derive(Debug, Clone, Copy)]
pub struct GenericStrike<'log> {
    log: &'log Log,
}

impl<'log> GenericStrike<'log> {
    pub fn new(log: &'log Log) -> Self {
        GenericStrike { log }
    }
}

impl<'log> Analyzer for GenericStrike<'log> {
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
