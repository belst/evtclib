use super::math::{Monoid, RecordFunc, Semigroup};
use std::fmt;

/// Type of the damage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DamageType {
    Physical,
    Condition,
}

/// Meta information about a damage log entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Meta {
    pub source: u64,
    pub target: u64,
    pub kind: DamageType,
    pub skill: u32,
}

/// A small wrapper that wraps a damage number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Damage(pub u64);

impl Semigroup for Damage {
    #[inline]
    fn combine(&self, other: &Self) -> Self {
        Damage(self.0 + other.0)
    }
}

impl Monoid for Damage {
    #[inline]
    fn mempty() -> Self {
        Damage(0)
    }
}

/// Provides access to the damage log.
#[derive(Clone, Default)]
pub struct DamageLog {
    inner: RecordFunc<u64, Meta, Damage>,
}

impl DamageLog {
    pub fn new() -> Self {
        DamageLog {
            inner: RecordFunc::new(),
        }
    }

    pub fn log(
        &mut self,
        time: u64,
        source: u64,
        target: u64,
        kind: DamageType,
        skill: u32,
        value: u64,
    ) {
        self.inner.insert(
            time,
            Meta {
                source,
                target,
                kind,
                skill,
            },
            Damage(value),
        )
    }

    pub fn damage_between<F: FnMut(&Meta) -> bool>(
        &self,
        start: u64,
        stop: u64,
        filter: F,
    ) -> Damage {
        self.inner.between_only(&start, &stop, filter)
    }

    pub fn damage<F: FnMut(&Meta) -> bool>(&self, filter: F) -> Damage {
        self.inner.tally_only(filter)
    }
}

impl fmt::Debug for DamageLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DamageLog {{ {} events logged, {:?} total damage }}",
            self.inner.len(),
            self.inner.tally()
        )
    }
}
