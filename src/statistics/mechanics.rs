use super::gamedata::Mechanic;
use super::math::{Monoid, RecordFunc, Semigroup};

use std::fmt;

/// A simple wrapper for integers.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Counter(u32);

impl Semigroup for Counter {
    #[inline]
    fn combine(&self, other: &Counter) -> Counter {
        Counter(self.0 + other.0)
    }
}

impl Monoid for Counter {
    #[inline]
    fn mempty() -> Counter {
        Counter(0)
    }
}

/// Provides access to the mechanic log.
#[derive(Clone, Default)]
pub struct MechanicLog {
    inner: RecordFunc<u64, (&'static Mechanic, u64), Counter>,
}

impl MechanicLog {
    /// Increase the mechanic counter for the given mechanic and agent by one.
    pub fn increase(&mut self, time: u64, mechanic: &'static Mechanic, agent: u64) {
        self.inner.insert(time, (mechanic, agent), Counter(1));
    }

    /// Return the count of mechanics.
    ///
    /// A function can be provided to filter entries by mechanic type and agent.
    pub fn count<F: FnMut(&'static Mechanic, u64) -> bool>(&self, mut filter: F) -> u32 {
        self.inner.tally_only(|(a, b)| filter(a, *b)).0
    }

    /// Return the count of mechanics between the two given times.
    ///
    /// A function can be provided to filter entries by mechanic type and agent.
    pub fn count_between<F: FnMut(&'static Mechanic, u64) -> bool>(
        &self,
        start: u64,
        stop: u64,
        mut filter: F,
    ) -> u32 {
        self.inner
            .between_only(&start, &stop, |(a, b)| filter(a, *b))
            .0
    }
}

impl fmt::Debug for MechanicLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MechanicLog {{ ... }}")
    }
}
