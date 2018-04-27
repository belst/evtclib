use std::cmp;

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
/// Now, you might ask *"How do I know how big the boon queues are?"* and sadly,
/// I do not have a satisfactory answer for this. For intensity-based boons, the
/// answer is "the number of maximum stacks". However, most boons are not
/// intensity-based. For all other boons, either check the source code of other
/// people who (claim to) know, or just make up a value. Your calculations might
/// be off by fractions of a second, but it should be good enough for most use
/// cases.
///
/// Interesting fun fact: Most (if not all) boons don't have a hardcoded limit
/// on how much you can have at a time, it rather depends on the boon duration
/// of the person who applies them. The only limitation is the size of the boon
/// queue.
#[derive(Clone, Debug)]
pub struct BoonQueue {
    capacity: u32,
    queue: Vec<u64>,
    boon_type: BoonType,
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
                self.queue = self.queue
                    .iter()
                    .cloned()
                    .filter(|v| *v > duration)
                    .map(|v| v - duration)
                    .collect();
            }
        }
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
