//! Traits and structures to analyze fights.
//!
//! Fights need different logic in order to determine specific data, for example each fight has a
//! different way to determine whether or not the Challenge Mote was activated, whether or not the
//! fight was successful, ...
//!
//! This module aims to unify that logic by providing the [`Analyzer`][Analyzer] trait, which
//! provides a unified interface to query this information. You can use
//! [`Log::analyzer`][Log::analyzer] or [`for_log`][for_log] to obtain an analyzer fitting for the
//! encounter that is represented by the log.
//!
//! Most of the time, you will be dealing with a dynamically dispatched version of
//! [`Analyzer`][Analyzer], that is either `&dyn Analyzer` or `Box<dyn Analyzer>`. Also keep in
//! mind that an analyzer keeps a reference to the log that it is analyzing, which can be accessed
//! through [`Analyzer::log`][Analyzer::log].
//!
//! The implementation of the different analyzers is split off in different submodules:
//! * [`raids`][raids] for the raid-related encounters.
//! * [`fractals`][fractals] for the fractal-specific encounters.
//! * [`strikes`][strikes] for the strike-mission specific encounters.
//!
//! Note that you should not create concrete analyzers on your own, as the behaviour is not
//! specified when you use a wrong analyzer for the given log. Rely only on
//! [`Log::analyzer`][Log::analyzer] (or [`for_log`][for_log]) and the methods defined in
//! [`Analyzer`][Analyzer].

use crate::{Encounter, Log};

pub mod fractals;
#[macro_use]
pub mod helpers;
pub mod raids;
pub mod strikes;

/// The outcome of a fight.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Outcome {
    /// The fight succeeded.
    Success,
    /// The fight failed, i.e. the group wiped.
    Failure,
}

impl Outcome {
    /// A function that turns a boolean into an [`Outcome`][Outcome].
    ///
    /// This is a convenience function that can help implementing
    /// [`Analyzer::outcome`][Analyzer::outcome], which is also why this function returns an Option
    /// instead of the outcome directly.
    ///
    /// This turns `true` into [`Outcome::Success`][Outcome::Success] and `false` into
    /// [`Outcome::Failure`][Outcome::Failure].
    pub fn from_bool(b: bool) -> Option<Outcome> {
        if b {
            Some(Outcome::Success)
        } else {
            Some(Outcome::Failure)
        }
    }
}

/// An [`Analyzer`][Analyzer] is something that implements fight-dependent analyzing of the log.
///
/// For more information and explanations, see the [module level documentation][self].
pub trait Analyzer {
    /// Returns a reference to the log being analyzed.
    fn log(&self) -> &Log;

    /// Checks whether the fight was done with the challenge mote activated.
    fn is_cm(&self) -> bool;

    /// Returns the outcome of the fight.
    ///
    /// Note that not all logs need to have an outcome, e.g. WvW or Golem logs may return `None`
    /// here.
    fn outcome(&self) -> Option<Outcome>;
}

/// Returns the correct [`Analyzer`][Analyzer] for the given log file.
///
/// See also [`Log::analyzer`][Log::analyzer].
pub fn for_log<'l>(log: &'l Log) -> Option<Box<dyn Analyzer + 'l>> {
    let boss = log.encounter()?;

    match boss {
        Encounter::ValeGuardian | Encounter::Gorseval | Encounter::Sabetha => {
            Some(Box::new(raids::GenericRaid::new(log)))
        }

        Encounter::Slothasor | Encounter::BanditTrio | Encounter::Matthias => {
            Some(Box::new(raids::GenericRaid::new(log)))
        }

        Encounter::KeepConstruct => Some(Box::new(raids::GenericRaid::new(log))),
        Encounter::TwistedCastle => Some(Box::new(raids::TwistedCastle::new(log))),
        Encounter::Xera => Some(Box::new(raids::Xera::new(log))),

        Encounter::Cairn => Some(Box::new(raids::Cairn::new(log))),
        Encounter::MursaatOverseer => Some(Box::new(raids::MursaatOverseer::new(log))),
        Encounter::Samarog => Some(Box::new(raids::Samarog::new(log))),
        Encounter::Deimos => Some(Box::new(raids::Deimos::new(log))),

        Encounter::SoullessHorror => Some(Box::new(raids::SoullessHorror::new(log))),
        Encounter::RiverOfSouls => Some(Box::new(raids::RiverOfSouls::new(log))),
        Encounter::BrokenKing | Encounter::EaterOfSouls | Encounter::StatueOfDarkness => {
            Some(Box::new(raids::GenericRaid::new(log)))
        }
        Encounter::VoiceInTheVoid => Some(Box::new(raids::Dhuum::new(log))),

        Encounter::ConjuredAmalgamate => Some(Box::new(raids::ConjuredAmalgamate::new(log))),
        Encounter::TwinLargos => Some(Box::new(raids::TwinLargos::new(log))),
        Encounter::Qadim => Some(Box::new(raids::Qadim::new(log))),

        Encounter::CardinalAdina => Some(Box::new(raids::CardinalAdina::new(log))),
        Encounter::CardinalSabir => Some(Box::new(raids::CardinalSabir::new(log))),
        Encounter::QadimThePeerless => Some(Box::new(raids::QadimThePeerless::new(log))),

        Encounter::StandardKittyGolem
        | Encounter::MediumKittyGolem
        | Encounter::LargeKittyGolem => Some(Box::new(raids::GenericRaid::new(log))),

        Encounter::Ai => Some(Box::new(fractals::Ai::new(log))),
        Encounter::Skorvald => Some(Box::new(fractals::Skorvald::new(log))),
        Encounter::Artsariiv
        | Encounter::Arkk
        | Encounter::MAMA
        | Encounter::Siax
        | Encounter::Ensolyss => Some(Box::new(fractals::GenericFractal::new(log))),

        Encounter::IcebroodConstruct
        | Encounter::SuperKodanBrothers
        | Encounter::FraenirOfJormag
        | Encounter::Boneskinner
        | Encounter::WhisperOfJormag
        | Encounter::Dragonvoid => Some(Box::new(strikes::GenericStrike::new(log))),

        Encounter::CaptainMaiTrin => Some(Box::new(strikes::CaptainMaiTrin::new(log))),
        Encounter::Ankka => Some(Box::new(strikes::Ankka::new(log))),
        Encounter::MinisterLi => Some(Box::new(strikes::MinisterLi::new(log))),
    }
}
