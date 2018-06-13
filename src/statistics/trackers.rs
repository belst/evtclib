//! evtclib tracker definitions.
//!
//! The idea behind a "tracker" is to have one object taking care of one
//! specific thing. This makes it easier to organize the whole "statistic
//! gathering loop", and it keeps each tracker somewhat small.
//!
//! It's also easy to define your own trackers if there are any statistics that
//! you want to track. Just implement [`Tracker`](trait.Tracker.html). It
//! doesn't matter what you track, it doesn't matter how many trackers you
//! define.
//!
//! If you want to track stats separated by player or phases, consider writing
//! your tracker in a way that it only tracks statistics for a single player,
//! and then use a [`Multiplexer`](struct.Multiplexer.html) to automatically
//! track it for every player/agent.
//!
//! You can use [`run_trackers`](../fn.run_trackers.html) to run multiple
//! trackers on the same log.
use std::collections::HashMap;
use std::error::Error;

use super::super::{Event, EventKind, Log};
use super::boon::BoonQueue;
use super::damage::{DamageLog, DamageType};
use super::gamedata::{self, Mechanic, Trigger};

/// A tracker.
///
/// A tracker should be responsible for tracking a single statistic. Each
/// tracker is fed each event. If an error is returned while feeding, the whole
/// calculation will be aborted.
pub trait Tracker {
    /// The resulting statistic that this tracker will return.
    type Stat;
    /// The error that this tracker might return.
    type Error: Error;

    /// Feed a single event into this tracker.
    ///
    /// The tracker will update its internal state.
    fn feed(&mut self, event: &Event) -> Result<(), Self::Error>;

    /// Finalize this tracker and get the statistics out.
    fn finalize(self) -> Result<Self::Stat, Self::Error>;
}

/// A helper trait that erases the types from a tracker.
///
/// This makes it able to use references like `&mut RunnableTracker` without
/// having to specify the generic types, allowing you to e.g. store a bunch of
/// them in a vector, regardless of their output type. Unless you want to do
/// that, you probably don't want to use this trait directly.
///
/// Note that you do not need to implement this yourself. It is automatically
/// implemented for all types that implement `Tracker`.
///
/// RunnableTrackers provide no way to extract their resources, and they wrap
/// all errors in `Box<_>`, so you should always keep a "real" reference around
/// if you plan on getting any data.
pub trait RunnableTracker {
    /// See `Tracker.feed()`. Renamed to avoid conflicts.
    fn run_feed(&mut self, event: &Event) -> Result<(), Box<Error>>;
}

impl<S, E: Error + 'static, T: Tracker<Stat = S, Error = E>> RunnableTracker for T {
    fn run_feed(&mut self, event: &Event) -> Result<(), Box<Error>> {
        self.feed(event).map_err(|e| Box::new(e) as Box<Error>)
    }
}

/// A tracker that tracks per-target damage of all agents.
pub struct DamageTracker<'l> {
    log: &'l Log,
    damage_log: DamageLog,
}

impl<'t> DamageTracker<'t> {
    /// Create a new damage tracker for the given log.
    pub fn new(log: &Log) -> DamageTracker {
        DamageTracker {
            log,
            damage_log: DamageLog::new(),
        }
    }
}

impl<'t> Tracker for DamageTracker<'t> {
    type Stat = DamageLog;
    type Error = !;

    fn feed(&mut self, event: &Event) -> Result<(), Self::Error> {
        match event.kind {
            EventKind::Physical {
                source_agent_addr,
                destination_agent_addr,
                damage,
                skill_id,
                ..
            } => {
                let source = if let Some(master) = self.log.master_agent(source_agent_addr) {
                    master.addr
                } else {
                    source_agent_addr
                };
                self.damage_log.log(
                    event.time,
                    source,
                    destination_agent_addr,
                    DamageType::Physical,
                    skill_id,
                    damage as u64,
                );
            }

            EventKind::ConditionTick {
                source_agent_addr,
                destination_agent_addr,
                damage,
                condition_id,
                ..
            } => {
                let source = if let Some(master) = self.log.master_agent(source_agent_addr) {
                    master.addr
                } else {
                    source_agent_addr
                };
                self.damage_log.log(
                    event.time,
                    source,
                    destination_agent_addr,
                    DamageType::Condition,
                    condition_id,
                    damage as u64,
                );
            }

            _ => (),
        }
        Ok(())
    }

    fn finalize(self) -> Result<Self::Stat, Self::Error> {
        Ok(self.damage_log)
    }
}

/// Tracks when the log has been started.
#[derive(Default)]
pub struct LogStartTracker {
    event_time: u64,
}

impl LogStartTracker {
    /// Create a new log start tracker.
    pub fn new() -> LogStartTracker {
        LogStartTracker { event_time: 0 }
    }
}

impl Tracker for LogStartTracker {
    type Stat = u64;
    type Error = !;

    fn feed(&mut self, event: &Event) -> Result<(), Self::Error> {
        if let EventKind::LogStart { .. } = event.kind {
            self.event_time = event.time;
        }
        Ok(())
    }

    fn finalize(self) -> Result<Self::Stat, Self::Error> {
        Ok(self.event_time)
    }
}

/// A tracker that tracks the combat entry and exit times for each agent.
#[derive(Default)]
pub struct CombatTimeTracker {
    times: HashMap<u64, (u64, u64)>,
}

impl CombatTimeTracker {
    /// Create a new combat time tracker.
    pub fn new() -> CombatTimeTracker {
        Default::default()
    }
}

impl Tracker for CombatTimeTracker {
    // Maps from agent addr to (entry time, exit time)
    type Stat = HashMap<u64, (u64, u64)>;
    type Error = !;

    fn feed(&mut self, event: &Event) -> Result<(), Self::Error> {
        match event.kind {
            EventKind::EnterCombat { agent_addr, .. } => {
                self.times.entry(agent_addr).or_insert((0, 0)).0 = event.time;
            }

            EventKind::ExitCombat { agent_addr } => {
                self.times.entry(agent_addr).or_insert((0, 0)).1 = event.time;
            }

            _ => (),
        }
        Ok(())
    }

    fn finalize(self) -> Result<Self::Stat, Self::Error> {
        Ok(self.times)
    }
}

/// A tracker that tracks the total "boon area" per agent.
///
/// The boon area is defined as the amount of stacks multiplied by the time. So
/// 1 stack of Might for 1000 milliseconds equals 1000 "stackmilliseconds" of
/// Might. You can use this boon area to calculate the average amount of stacks
/// by taking the boon area and dividing it by the combat time.
///
/// Note that this also tracks conditions, because internally, they're handled
/// the same way.
///
/// This tracker only tracks the boons that are known to evtclib, that is the
/// boons defined in `evtclib::statistics::gamedata::BOONS`.
pub struct BoonTracker {
    agent_addr: u64,
    boon_areas: HashMap<u16, u64>,
    boon_queues: HashMap<u16, BoonQueue>,
    last_time: u64,
    next_update: u64,
}

impl BoonTracker {
    /// Creates a new boon tracker for the given agent.
    pub fn new(agent_addr: u64) -> BoonTracker {
        BoonTracker {
            agent_addr,
            boon_areas: Default::default(),
            boon_queues: Default::default(),
            last_time: 0,
            next_update: 0,
        }
    }

    /// Updates the internal boon queues by the given amount of milliseconds.
    ///
    /// * `delta_t` - Amount of milliseconds to update.
    fn update_queues(&mut self, delta_t: u64) {
        self.boon_queues
            .values_mut()
            .for_each(|queue| queue.simulate(delta_t));

        // Throw away empty boon queues or to improve performance
        self.boon_queues.retain(|_, q| !q.is_empty());
    }

    /// Update the internal tracking areas.
    ///
    /// Does not update the boon queues.
    ///
    /// * `delta_t` - Amount of milliseconds that passed.
    fn update_areas(&mut self, delta_t: u64) {
        for (buff_id, queue) in &self.boon_queues {
            let current_stacks = queue.current_stacks();
            let area = self.boon_areas.entry(*buff_id).or_insert(0);
            *area += current_stacks as u64 * delta_t;
        }
    }

    fn update_next_update(&mut self) {
        let next_update = self
            .boon_queues
            .values()
            .map(BoonQueue::next_update)
            .filter(|v| *v != 0)
            .min()
            .unwrap_or(0);
        self.next_update = next_update;
    }

    /// Get the boon queue for the given buff_id.
    ///
    /// If the queue does not yet exist, create it.
    ///
    /// * `buff_id` - The buff (or condition) id.
    fn get_queue(&mut self, buff_id: u16) -> Option<&mut BoonQueue> {
        use std::collections::hash_map::Entry;
        let entry = self.boon_queues.entry(buff_id);
        match entry {
            // Queue already exists
            Entry::Occupied(e) => Some(e.into_mut()),
            // Queue needs to be created, but only if we know about that boon.
            Entry::Vacant(e) => {
                let boon_queue = gamedata::get_boon(buff_id).map(gamedata::Boon::create_queue);
                if let Some(queue) = boon_queue {
                    Some(e.insert(queue))
                } else {
                    None
                }
            }
        }
    }
}

impl Tracker for BoonTracker {
    type Stat = HashMap<u16, u64>;
    type Error = !;

    fn feed(&mut self, event: &Event) -> Result<(), Self::Error> {

        let delta_t = event.time - self.last_time;
        if self.next_update != 0 && delta_t > self.next_update {
            self.update_queues(delta_t);
            self.update_areas(delta_t);
            self.update_next_update();
            self.last_time = event.time;
        }

        match event.kind {
            EventKind::BuffApplication {
                buff_id, duration, ..
            } => {
                if let Some(queue) = self.get_queue(buff_id) {
                    queue.add_stack(duration as u64);
                }
                self.update_next_update();
            }

            // XXX: Properly handle SINGLE and MANUAL removal types
            EventKind::BuffRemove { buff_id, .. } => {
                if let Some(queue) = self.get_queue(buff_id) {
                    queue.clear();
                }
                self.update_next_update();
            }

            _ => (),
        }

        Ok(())
    }

    fn finalize(self) -> Result<Self::Stat, Self::Error> {
        Ok(self.boon_areas)
    }
}

/// A tracker that tracks boss mechanics for each player.
pub struct MechanicTracker {
    mechanics: HashMap<&'static Mechanic, u32>,
    available_mechanics: Vec<&'static Mechanic>,
    boss_address: u64,
    agent_address: u64,
}

impl MechanicTracker {
    /// Create a new mechanic tracker that watches over the given mechanics.
    pub fn new(
        agent_address: u64,
        boss_address: u64,
        mechanics: Vec<&'static Mechanic>,
    ) -> MechanicTracker {
        MechanicTracker {
            mechanics: HashMap::new(),
            available_mechanics: mechanics,
            boss_address,
            agent_address,
        }
    }
}

impl Tracker for MechanicTracker {
    type Stat = HashMap<&'static Mechanic, u32>;
    type Error = !;

    fn feed(&mut self, event: &Event) -> Result<(), Self::Error> {
        fn increase(map: &mut HashMap<&'static Mechanic, u32>, entry: &'static Mechanic) {
            *map.entry(entry).or_insert(0) += 1;
        }

        for mechanic in &self.available_mechanics {
            match (&event.kind, &mechanic.1) {
                (
                    EventKind::Physical {
                        source_agent_addr,
                        destination_agent_addr,
                        skill_id,
                        ..
                    },
                    Trigger::SkillOnPlayer(trigger_id),
                ) if skill_id == trigger_id
                    && *source_agent_addr == self.boss_address
                    && *destination_agent_addr == self.agent_address =>
                {
                    increase(&mut self.mechanics, mechanic);
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn finalize(self) -> Result<Self::Stat, Self::Error> {
        Ok(self.mechanics)
    }
}
