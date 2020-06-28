//! Boss fight analyzers for Wing 6 (Mythwright Gambit)
use crate::{
    analyzers::{helpers, Analyzer},
    Log,
};

pub const CA_CM_BUFF: u32 = 53_075;

/// Analyzer for the first fight of Wing 6, Conjured Amalgamate.
///
/// The CM is detected by the presence of the buff that the player targeted by the laser has.
#[derive(Debug, Clone, Copy)]
pub struct ConjuredAmalgamate<'log> {
    log: &'log Log,
}

impl<'log> ConjuredAmalgamate<'log> {
    pub fn new(log: &'log Log) -> Self {
        ConjuredAmalgamate { log }
    }
}

impl<'log> Analyzer for ConjuredAmalgamate<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        helpers::buff_present(self.log, CA_CM_BUFF)
    }
}

pub const LARGOS_CM_HEALTH: u64 = 19_200_000;

/// Analyzer for the second fight of Wing 6, Largos Twins.
///
/// The CM is detected by the boss's health, which is higher in the challenge mote.
#[derive(Debug, Clone, Copy)]
pub struct LargosTwins<'log> {
    log: &'log Log,
}

impl<'log> LargosTwins<'log> {
    pub fn new(log: &'log Log) -> Self {
        LargosTwins { log }
    }
}

impl<'log> Analyzer for LargosTwins<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        helpers::boss_health(self.log)
            .map(|h| h >= LARGOS_CM_HEALTH)
            .unwrap_or(false)
    }
}

pub const QADIM_CM_HEALTH: u64 = 21_100_000;

/// Analyzer for the third fight of Wing 6, Qadim.
///
/// The CM is detected by the boss's health, which is higher in the challenge mote.
#[derive(Debug, Clone, Copy)]
pub struct Qadim<'log> {
    log: &'log Log,
}

impl<'log> Qadim<'log> {
    pub fn new(log: &'log Log) -> Self {
        Qadim { log }
    }
}

impl<'log> Analyzer for Qadim<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        helpers::boss_health(self.log)
            .map(|h| h >= QADIM_CM_HEALTH)
            .unwrap_or(false)
    }
}
