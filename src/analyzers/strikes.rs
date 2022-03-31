//! Analyzers for Strike Mission logs.
use crate::{
    analyzers::{helpers, Analyzer, Outcome},
    gamedata::Boss,
    EventKind, Log,
};

/// Analyzer for strikes.
///
/// Since there are currently no strikes requiring special logic, this analyzer is used for all
/// strike missions.
#[derive(Debug, Clone, Copy)]
pub struct GenericStrike<'log> {
    log: &'log Log,
}

impl<'log> GenericStrike<'log> {
    /// Create a new [`GenericStrike`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
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

/// Analyzer for the Captain Mai Trin/Aetherblade Hideout strike.
#[derive(Debug, Clone, Copy)]
pub struct CaptainMaiTrin<'log> {
    log: &'log Log,
}

impl<'log> CaptainMaiTrin<'log> {
    pub const ECHO_OF_SCARLET_BRIAR: u16 = 24_768;
    pub const WINNING_BUFF: u32 = 895;

    /// Create a new [`CaptainMaiTrin`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
    pub fn new(log: &'log Log) -> Self {
        CaptainMaiTrin { log }
    }
}

impl<'log> Analyzer for CaptainMaiTrin<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        // EoD strike CMs are not implemented yet as of 2022-03-31
        false
    }

    fn outcome(&self) -> Option<Outcome> {
        check_reward!(self.log);

        let scarlet = self
            .log
            .characters()
            .find(|npc| npc.id() == Self::ECHO_OF_SCARLET_BRIAR)?;
        let mai = self
            .log
            .characters()
            .find(|npc| npc.id() == Boss::CaptainMaiTrin as u16)?;

        for event in self.log.events() {
            if let EventKind::BuffApplication {
                destination_agent_addr,
                buff_id,
                ..
            } = event.kind()
            {
                if *buff_id == Self::WINNING_BUFF
                    && *destination_agent_addr == mai.addr()
                    && event.time() > scarlet.first_aware()
                {
                    return Some(Outcome::Success);
                }
            }
        }

        Some(Outcome::Failure)
    }
}
