//! Boss fight analyzers for Wing 4 (Bastion of the Penitent).
use crate::{
    analyzers::{helpers, Analyzer},
    Log,
};

pub const CAIRN_CM_BUFF: u32 = 38_098;

/// Analyzer for the first fight of Wing 4, Cairn.
///
/// The CM is detected by the presence of the buff representing the countdown before which you have
/// to use your special action skill.
#[derive(Debug, Clone, Copy)]
pub struct Cairn<'log> {
    log: &'log Log,
}

impl<'log> Cairn<'log> {
    pub fn new(log: &'log Log) -> Self {
        Cairn { log }
    }
}

impl<'log> Analyzer for Cairn<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        helpers::buff_present(self.log, CAIRN_CM_BUFF)
    }
}

pub const MO_CM_HEALTH: u64 = 30_000_000;

/// Analyzer for the second fight of Wing 4, Mursaat Overseer.
///
/// The CM is detected by the boss's health, which is higher in the challenge mote.
#[derive(Debug, Clone, Copy)]
pub struct MursaatOverseer<'log> {
    log: &'log Log,
}

impl<'log> MursaatOverseer<'log> {
    pub fn new(log: &'log Log) -> Self {
        MursaatOverseer { log }
    }
}

impl<'log> Analyzer for MursaatOverseer<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        helpers::boss_health(self.log)
            .map(|h| h >= MO_CM_HEALTH)
            .unwrap_or(false)
    }
}

pub const SAMAROG_CM_HEALTH: u64 = 40_000_000;

/// Analyzer for the third fight of Wing 4, Samarog.
///
/// The CM is detected by the boss's health, which is higher in the challenge mote.
#[derive(Debug, Clone, Copy)]
pub struct Samarog<'log> {
    log: &'log Log,
}

impl<'log> Samarog<'log> {
    pub fn new(log: &'log Log) -> Self {
        Samarog { log }
    }
}

impl<'log> Analyzer for Samarog<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        helpers::boss_health(self.log)
            .map(|h| h >= SAMAROG_CM_HEALTH)
            .unwrap_or(false)
    }
}

pub const DEIMOS_CM_HEALTH: u64 = 42_000_000;

/// Analyzer for the fourth fight of Wing 4, Deimos.
///
/// The CM is detected by the boss's health, which is higher in the challenge mote.
#[derive(Debug, Clone, Copy)]
pub struct Deimos<'log> {
    log: &'log Log,
}

impl<'log> Deimos<'log> {
    pub fn new(log: &'log Log) -> Self {
        Deimos { log }
    }
}

impl<'log> Analyzer for Deimos<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        helpers::boss_health(self.log)
            .map(|h| h >= DEIMOS_CM_HEALTH)
            .unwrap_or(false)
    }
}
