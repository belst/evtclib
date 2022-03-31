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
    /// Determined buff that is used in Mai Trin's Strike.
    ///
    /// Thanks to ArenaNet's consistency, there are multiple versions of the Determined buff in
    /// use.
    ///
    /// The chat link for this buff is `[&Bn8DAAA=]`.
    pub const DETERMINED_ID: u32 = 895;

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
                if *buff_id == Self::DETERMINED_ID
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

/// Analyzer for the Ankka/Xunlai Jade Junkyard strike.
#[derive(Debug, Clone, Copy)]
pub struct Ankka<'log> {
    log: &'log Log,
}

impl<'log> Ankka<'log> {
    /// Determined buff that is used in Ankka's Strike.
    ///
    /// Thanks to ArenaNet's consistency, there are multiple versions of the Determined buff in
    /// use.
    ///
    /// The chat link for this buff is `[&Bn8DAAA=]`.
    pub const DETERMINED_ID: u32 = CaptainMaiTrin::DETERMINED_ID;
    /// The minimum duration of [`DETERMINED_ID`] buff applications.
    pub const DURATION_CUTOFF: i32 = i32::MAX;
    /// The expected number of times that Ankka needs to phase before we consider it a success.
    pub const EXPECTED_PHASE_COUNT: usize = 3;

    /// Create a new [`Ankka`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
    pub fn new(log: &'log Log) -> Self {
        Ankka { log }
    }
}

impl<'log> Analyzer for Ankka<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        // EoD strike CMs are not implemented yet as of 2022-03-31
        false
    }

    fn outcome(&self) -> Option<Outcome> {
        check_reward!(self.log);

        let ankka = self
            .log
            .characters()
            .find(|npc| npc.id() == Boss::Ankka as u16)?;

        let phase_change_count = self
            .log
            .events()
            .iter()
            .filter(|event| {
                if let EventKind::BuffApplication {
                    destination_agent_addr,
                    buff_id,
                    duration,
                    ..
                } = event.kind()
                {
                    *buff_id == Self::DETERMINED_ID
                        && *destination_agent_addr == ankka.addr()
                        && *duration == Self::DURATION_CUTOFF
                } else {
                    false
                }
            })
            .count();

        Outcome::from_bool(phase_change_count == Self::EXPECTED_PHASE_COUNT)
    }
}

/// Analyzer for the Minister Li/Kaineng Overlook strike.
#[derive(Debug, Clone, Copy)]
pub struct MinisterLi<'log> {
    log: &'log Log,
}

impl<'log> MinisterLi<'log> {
    /// Determined buff that is used in Minister Li's Strike.
    ///
    /// Thanks to ArenaNet's consistency, there are multiple versions of the Determined buff in
    /// use.
    ///
    /// The chat link for this buff is `[&BvoCAAA=]`.
    pub const DETERMINED_ID: u32 = 762;
    /// The minimum number of times that Minister Li needs to phase before we consider it a success.
    pub const MINIMUM_PHASE_COUNT: usize = 3;

    /// Create a new [`MinisterLi`] analyzer for the given log.
    ///
    /// **Do not** use this method unless you know what you are doing. Instead, rely on
    /// [`Log::analyzer`]!
    pub fn new(log: &'log Log) -> Self {
        MinisterLi { log }
    }
}

impl<'log> Analyzer for MinisterLi<'log> {
    fn log(&self) -> &Log {
        self.log
    }

    fn is_cm(&self) -> bool {
        // EoD strike CMs are not implemented yet as of 2022-03-31
        false
    }

    fn outcome(&self) -> Option<Outcome> {
        check_reward!(self.log);

        let li = self
            .log
            .characters()
            .find(|npc| npc.id() == Boss::MinisterLi as u16)?;

        let phase_change_count = self
            .log
            .events()
            .iter()
            .filter(|event| {
                if let EventKind::BuffApplication {
                    destination_agent_addr,
                    buff_id,
                    ..
                } = event.kind()
                {
                    *buff_id == Self::DETERMINED_ID && *destination_agent_addr == li.addr()
                } else {
                    false
                }
            })
            .count();

        Outcome::from_bool(phase_change_count >= Self::MINIMUM_PHASE_COUNT)
    }
}
