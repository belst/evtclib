use crate::{
    analyzers::{helpers, Analyzer, Outcome},
    Log,
};

/// Analyzer for the final fight of Wing 3, Xera.
#[derive(Debug, Clone, Copy)]
pub struct Xera<'log> {
    log: &'log Log,
}

impl<'log> Xera<'log> {
    pub fn new(log: &'log Log) -> Self {
        Xera { log }
    }
}

impl<'log> Analyzer for Xera<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        false
    }

    fn outcome(&self) -> Option<Outcome> {
        Outcome::from_bool(helpers::players_exit_after_boss(self.log))
    }
}
