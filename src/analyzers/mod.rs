//! Traits and structures to analyze fights.
//!
//! Fights need different logic to determine some data, for example each fight has a different way
//! to determine whether or not the Challenge Mote was activated, whether or not the fight was
//! successful, ...
//!
//! This module aims to unify that logic by providing a trait [`Analyzer`][Analyzer], which
//! provides a unified interface to query this information. You can use
//! [`Log::analyzer`][Log::analyzer] or [`for_log`][for_log] to obtain an analyzer fitting for the
//! encounter that is represented by the log.
//!
//! The implementation of the different analyzers is split off in different submodules:
//! * [`raids`][raids] for the raid-related encounters.
//!
//! Note that you should not create concrete analyzers on your own, as the behaviour is not
//! specified when you use a wrong analyzer for the given log. Rely only on
//! [`Log::analyzer`][Log::analyzer] (or [`for_log`][for_log]) and the methods defined in
//! [`Analyzer`][Analyzer].

use crate::{Boss, Log};

pub mod fractals;
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
        Boss::ValeGuardian | Boss::Gorseval | Boss::Sabetha => {
            Some(Box::new(raids::GenericRaid::new(log)))
        }

        Boss::Slothasor | Boss::Matthias => Some(Box::new(raids::GenericRaid::new(log))),

        Boss::KeepConstruct => Some(Box::new(raids::GenericRaid::new(log))),
        Boss::Xera => Some(Box::new(raids::Xera::new(log))),

        Boss::Cairn => Some(Box::new(raids::Cairn::new(log))),
        Boss::MursaatOverseer => Some(Box::new(raids::MursaatOverseer::new(log))),
        Boss::Samarog => Some(Box::new(raids::Samarog::new(log))),
        Boss::Deimos => Some(Box::new(raids::Deimos::new(log))),

        Boss::SoullessHorror => Some(Box::new(raids::SoullessHorror::new(log))),
        Boss::Dhuum => Some(Box::new(raids::Dhuum::new(log))),

        Boss::ConjuredAmalgamate => Some(Box::new(raids::ConjuredAmalgamate::new(log))),
        Boss::LargosTwins => Some(Box::new(raids::LargosTwins::new(log))),
        Boss::Qadim => Some(Box::new(raids::Qadim::new(log))),

        Boss::CardinalAdina => Some(Box::new(raids::CardinalAdina::new(log))),
        Boss::CardinalSabir => Some(Box::new(raids::CardinalSabir::new(log))),
        Boss::QadimThePeerless => Some(Box::new(raids::QadimThePeerless::new(log))),

        Boss::Skorvald => Some(Box::new(fractals::Skorvald::new(log))),
        Boss::Artsariiv | Boss::Arkk | Boss::MAMA | Boss::Siax | Boss::Ensolyss => {
            Some(Box::new(fractals::GenericFractal::new(log)))
        }

        Boss::IcebroodConstruct
        | Boss::VoiceOfTheFallen
        | Boss::FraenirOfJormag
        | Boss::Boneskinner
        | Boss::WhisperOfJormag => Some(Box::new(strikes::GenericStrike::new(log))),
    }
}
