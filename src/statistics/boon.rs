//! Module providing functions and structs to deal with boon related statistics.
use std::cmp;

use std::collections::HashMap;
use std::fmt;
use std::ops::Mul;

use super::math::{Monoid, RecordFunc, Semigroup};

use fnv::FnvHashMap;

/// The type of a boon.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BoonType {
    /// Boon stacks duration, e.g. Regeneration.
    Duration,
    /// Boon stacks intensity, e.g. Might.
    Intensity,
}

/// A struct that helps with simulating boon changes over time.
///
/// This basically simulates a single boon-queue (for a single boon).
///
/// # A quick word about how boon queues work
///
/// For each boon, you have an internal *boon queue*, limited to a specific
/// capacity. When the current stack expires, the next one is taken from the
/// queue.
///
/// The queue is sorted by boon strength. This means that "weak" boons are
/// always at the end (and as such, are the first ones to be deleted when the
/// queue is full). This prevents "bad" boons (e.g. the Quickness from Lightning
/// Hammer #2) to override the "good" boons (e.g. the Quickness from your
/// friendly neighborhood Chrono with 100% boon duration).
///
/// This also means that boons can be "lost". If the queue is full, the boon
/// might not get applied, or it might replace another boon, thus wasting some
/// of the boon duration.
///
/// Intensity-stacked boons (such as Might) work a bit differently: as time
/// passes, all stacks are decreased simultaneously! As soon as a stack reaches
/// 0, it is dropped.
///
/// You can find more information and the size of some of the queues on the wiki:
/// https://wiki.guildwars2.com/wiki/Effect_stacking
#[derive(Clone, Debug)]
pub struct BoonQueue {
    capacity: u32,
    queue: Vec<u64>,
    boon_type: BoonType,
    next_update: u64,
}

impl BoonQueue {
    /// Create a new boon queue.
    ///
    /// * `capacity` - The capacity of the queue.
    /// * `boon_type` - How the boons stack.
    pub fn new(capacity: u32, boon_type: BoonType) -> BoonQueue {
        BoonQueue {
            capacity,
            queue: Vec::new(),
            boon_type,
            next_update: 0,
        }
    }

    fn fix_queue(&mut self) {
        // Sort reversed, so that the longest stack is at the front.
        self.queue.sort_unstable_by(|a, b| b.cmp(a));
        // Truncate queue by cutting of the shortest stacks
        if self.queue.len() > self.capacity as usize {
            self.queue.drain(self.capacity as usize..);
        }
    }

    /// Get the type of this boon.
    pub fn boon_type(&self) -> BoonType {
        self.boon_type
    }

    /// Add a boon stack to this queue.
    ///
    /// * `duration` - Duration (in milliseconds) of the added stack.
    pub fn add_stack(&mut self, duration: u64) {
        self.queue.push(duration);
        self.fix_queue();
        self.next_update = self.next_change();
    }

    /// Return the amount of current stacks.
    ///
    /// If the boon type is a duration boon, this will always return 0 or 1.
    ///
    /// If the boon type is an intensity boon, it will return the number of
    /// stacks.
    pub fn current_stacks(&self) -> u32 {
        let result = match self.boon_type {
            BoonType::Intensity => self.queue.len(),
            BoonType::Duration => cmp::min(1, self.queue.len()),
        };
        result as u32
    }

    /// Simulate time passing.
    ///
    /// This will decrease the remaining duration of the stacks accordingly.
    ///
    /// * `duration` - The amount of time (in milliseconds) to simulate.
    pub fn simulate(&mut self, duration: u64) {
        if duration == 0 {
            return;
        }
        if duration < self.next_update {
            self.next_update -= duration;
            return;
        }
        let mut remaining = duration;
        match self.boon_type {
            BoonType::Duration => {
                while remaining > 0 && !self.queue.is_empty() {
                    let next = self.queue.remove(0);
                    if next > remaining {
                        self.queue.push(next - remaining);
                        break;
                    } else {
                        remaining -= next;
                    }
                }
                self.fix_queue();
            }

            BoonType::Intensity => {
                self.queue = self
                    .queue
                    .iter()
                    .cloned()
                    .filter(|v| *v > duration)
                    .map(|v| v - duration)
                    .collect();
            }
        }
        self.next_update = self.next_change();
    }

    /// Remove all stacks.
    pub fn clear(&mut self) {
        self.queue.clear();
    }

    /// Checks if any stacks are left.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Calculate when the stacks will have the next visible change.
    ///
    /// This assumes that the stacks will not be modified during this time.
    ///
    /// The return value is the duration in milliseconds. If the boon queue is
    /// currently empty, 0 is returned.
    pub fn next_change(&self) -> u64 {
        match self.boon_type {
            BoonType::Duration => self.queue.iter().sum(),
            BoonType::Intensity => self.queue.last().cloned().unwrap_or(0),
        }
    }

    /// Calculate when the boon queue should be updated next.
    ///
    /// The next update always means that a stack runs out, even if it has no
    /// visible effect.
    ///
    /// For each queue: `next_update() <= next_change()`.
    ///
    /// A return value of 0 means that there's no update awaiting.
    pub fn next_update(&self) -> u64 {
        self.queue.last().cloned().unwrap_or(0)
    }
}

/// Amount of stacks of a boon.
// Since this is also used to represent changes in stacks, we need access to
// negative numbers too, as stacks can drop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stacks(i32);

impl Semigroup for Stacks {
    #[inline]
    fn combine(&self, other: &Self) -> Self {
        Stacks(self.0 + other.0)
    }
}

impl Monoid for Stacks {
    #[inline]
    fn mempty() -> Self {
        Stacks(0)
    }
}

// This shouldn't be negative, as total stacks are always greater than 0, thus
// the area below the curve will always be positive.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[doc(hidden)]
pub struct BoonArea(u64);

impl Semigroup for BoonArea {
    #[inline]
    fn combine(&self, other: &Self) -> Self {
        BoonArea(self.0 + other.0)
    }
}

impl Monoid for BoonArea {
    #[inline]
    fn mempty() -> Self {
        BoonArea(0)
    }
}

impl Mul<u64> for Stacks {
    type Output = BoonArea;

    #[inline]
    fn mul(self, rhs: u64) -> BoonArea {
        BoonArea(self.0 as u64 * rhs)
    }
}

/// A boon log for a specific player.
///
/// This logs the amount of stacks of each boon a player had at any given time.
#[derive(Clone, Default)]
pub struct BoonLog {
    // Keep a separate RecordFunc for each boon.
    inner: FnvHashMap<u16, RecordFunc<u64, (), Stacks>>,
}

impl BoonLog {
    /// Create a new, empty boon log.
    pub fn new() -> Self {
        Default::default()
    }

    /// Add an event to the boon log.
    pub fn log(&mut self, time: u64, boon_id: u16, stacks: u32) {
        let func = self.inner.entry(boon_id).or_insert_with(Default::default);
        let current = func.tally();
        if current.0 == stacks as i32 {
            return;
        }
        let diff = stacks as i32 - current.0;
        func.insert(time, (), Stacks(diff));
    }

    /// Get the average amount of stacks between the two given time points.
    ///
    /// * `a` - Start time point.
    /// * `b` - End time point.
    /// * `boon_id` - ID of the boon that you want to get the average for.
    pub fn average_stacks(&self, a: u64, b: u64, boon_id: u16) -> f32 {
        assert!(b > a);
        let func = if let Some(f) = self.inner.get(&boon_id) {
            f
        } else {
            return 0.;
        };
        let area = func.integral(&a, &b);
        area.0 as f32 / (b - a) as f32
    }

    /// Get the amount of stacks at the given time point.
    ///
    /// * `x` - Time point.
    /// * `boon_id` - ID of the boon that you want to get.
    pub fn stacks_at(&self, x: u64, boon_id: u16) -> u32 {
        self.inner.get(&boon_id).map(|f| f.get(&x)).unwrap_or(0)
    }
}

impl fmt::Debug for BoonLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BoonLog {{ .. }}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_queue_capacity() {
        let mut queue = BoonQueue::new(5, BoonType::Intensity);
        assert_eq!(queue.current_stacks(), 0);
        for _ in 0..10 {
            queue.add_stack(10);
        }
        assert_eq!(queue.current_stacks(), 5);
    }

    #[test]
    fn test_simulate_duration() {
        let mut queue = BoonQueue::new(10, BoonType::Duration);
        queue.add_stack(10);
        queue.add_stack(20);
        assert_eq!(queue.current_stacks(), 1);
        queue.simulate(30);
        assert_eq!(queue.current_stacks(), 0);

        queue.add_stack(50);
        queue.simulate(30);
        assert_eq!(queue.current_stacks(), 1);
        queue.simulate(10);
        assert_eq!(queue.current_stacks(), 1);
        queue.simulate(15);
        assert_eq!(queue.current_stacks(), 0);
    }

    #[test]
    fn test_simulate_intensity() {
        let mut queue = BoonQueue::new(5, BoonType::Intensity);

        queue.add_stack(10);
        queue.add_stack(20);
        assert_eq!(queue.current_stacks(), 2);

        queue.simulate(5);
        assert_eq!(queue.current_stacks(), 2);

        queue.simulate(5);
        assert_eq!(queue.current_stacks(), 1);
        queue.simulate(15);
        assert_eq!(queue.current_stacks(), 0);
    }
}
