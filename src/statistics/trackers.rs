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
//! You can use [`run_trackers`](../fn.run_trackers.html) to run multiple
//! trackers on the same log.
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;

use super::super::{Event, EventKind, Log};
use super::boon::BoonQueue;
use super::gamedata::{self, Mechanic, Trigger};
use super::DamageStats;

// A support macro to introduce a new block.
//
// Doesn't really require a macro, but it's nicer to look at
//   with! { foo = bar }
// rather than
//   { let foo = bar; ... }
macro_rules! with {
    ($name:ident = $expr:expr => $bl:block) => {{
        let $name = $expr;
        $bl
    }};
}

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

/// A trait that allows a tracker to be multiplexed.
///
/// Basically, this is a factory that allows new trackers to be created. Each
/// tracker can be given a key that it should listen on, which is expressed by
/// the `K` type parameter and the `key` parameter to `create`.
///
/// A blanket implementation for closures is provided, so you can use any
/// `FnMut(K) -> T`, where `K` is the key and `T` is the tracker.
pub trait Multiplexable<K> {
    /// The type of tracker that this multiplexable/factory creates.
    type T: Tracker;

    /// Create a new tracker, listening for the given key.
    fn create(&mut self, key: K) -> Self::T;
}

// This implementation allows a closure to be used as a multiplexable/tracker
// factory.
impl<T, K, O> Multiplexable<K> for T
where
    T: FnMut(K) -> O,
    O: Tracker,
{
    type T = O;

    fn create(&mut self, key: K) -> Self::T {
        self(key)
    }
}

/// A helper that wraps (decorates) another tracker and separates the results
/// based on the given criteria.
///
/// Instead of outputting a single statistic, it outputs a `HashMap`, mapping
/// the criteria to its own tracker.
///
/// This can be used for example to count damage per player: The damage tracker
/// itself only counts damage for a single player, and together with a
/// multiplexer, it will count the damage per player.
///
/// Type parameters:
/// * `K` Key that is used to distinguish criteria. For example, `u64` for a
///   multiplexer that separates based on agents.
/// * `F` Factory that creates new trackers for each key.
/// * `T` Inner tracker type. Usually determined by the factory.
/// * `S` Selection function type. Takes an event and outputs a key.
///
/// # Example
///
/// ```
/// # use evtclib::statistics::trackers::*;
/// let boons = Multiplexer::multiplex_on_destination(|agent| BoonTracker::new(agent));
/// ```
pub struct Multiplexer<K, F, T, S> {
    factory: F,
    trackers: HashMap<K, T>,
    selector: S,
}

impl Multiplexer<(), (), (), ()> {
    /// Create a new multiplexer that multiplexes on the source agent.
    pub fn multiplex_on_source<Factory: Multiplexable<u64>>(
        factory: Factory,
    ) -> Multiplexer<u64, Factory, Factory::T, impl Fn(&Event) -> Option<u64>> {
        Multiplexer {
            factory,
            trackers: HashMap::new(),
            selector: |event: &Event| event.kind.source_agent_addr(),
        }
    }

    /// Create a new multiplexer that multiplexes on the destination agent.
    pub fn multiplex_on_destination<Factory: Multiplexable<u64>>(
        factory: Factory,
    ) -> Multiplexer<u64, Factory, Factory::T, impl Fn(&Event) -> Option<u64>> {
        Multiplexer {
            factory,
            trackers: HashMap::new(),
            selector: |event: &Event| event.kind.destination_agent_addr(),
        }
    }
}

impl<K: Hash + Eq + Clone, F: Multiplexable<K>, S: FnMut(&Event) -> Option<K>> Tracker
    for Multiplexer<K, F, F::T, S>
{
    type Stat = HashMap<K, <F::T as Tracker>::Stat>;
    type Error = <F::T as Tracker>::Error;

    fn feed(&mut self, event: &Event) -> Result<(), Self::Error> {
        if let Some(key) = (self.selector)(event) {
            let factory = &mut self.factory;
            let entry = self
                .trackers
                .entry(key.clone())
                .or_insert_with(|| factory.create(key));
            entry.feed(event)?;
        }
        Ok(())
    }

    fn finalize(self) -> Result<Self::Stat, Self::Error> {
        self.trackers
            .into_iter()
            .map(|(k, v)| v.finalize().map(|inner| (k, inner)))
            .collect()
    }
}

/// A tracker that tracks per-target damage of all agents.
pub struct DamageTracker<'l> {
    log: &'l Log,
    // Source -> Target -> Damage
    damages: HashMap<u64, HashMap<u64, DamageStats>>,
}

impl<'t> DamageTracker<'t> {
    /// Create a new damage tracker for the given log.
    pub fn new(log: &Log) -> DamageTracker {
        DamageTracker {
            log,
            damages: HashMap::new(),
        }
    }

    fn get_stats(&mut self, source: u64, target: u64) -> &mut DamageStats {
        self.damages
            .entry(source)
            .or_insert_with(Default::default)
            .entry(target)
            .or_insert_with(Default::default)
    }
}

impl<'t> Tracker for DamageTracker<'t> {
    type Stat = HashMap<u64, HashMap<u64, DamageStats>>;
    type Error = !;

    fn feed(&mut self, event: &Event) -> Result<(), Self::Error> {
        match event.kind {
            EventKind::Physical {
                source_agent_addr,
                destination_agent_addr,
                damage,
                ..
            } => {
                with! { stats = self.get_stats(source_agent_addr, destination_agent_addr) => {
                    stats.total_damage += damage as u64;
                    stats.power_damage += damage as u64;
                }}

                if let Some(master) = self.log.master_agent(source_agent_addr) {
                    let master_stats = self.get_stats(master.addr, destination_agent_addr);
                    master_stats.total_damage += damage as u64;
                    master_stats.add_damage += damage as u64;
                }
            }

            EventKind::ConditionTick {
                source_agent_addr,
                destination_agent_addr,
                damage,
                ..
            } => {
                with! { stats = self.get_stats(source_agent_addr, destination_agent_addr) => {
                    stats.total_damage += damage as u64;
                    stats.condition_damage += damage as u64;
                }}

                if let Some(master) = self.log.master_agent(source_agent_addr) {
                    let master_stats = self.get_stats(master.addr, destination_agent_addr);
                    master_stats.total_damage += damage as u64;
                    master_stats.add_damage += damage as u64;
                }
            }

            _ => (),
        }
        Ok(())
    }

    fn finalize(self) -> Result<Self::Stat, Self::Error> {
        Ok(self.damages)
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
        if event.kind.destination_agent_addr() != Some(self.agent_addr) {
            return Ok(());
        }

        let delta_t = event.time - self.last_time;
        if self.next_update != 0 && delta_t > self.next_update {
            self.update_queues(delta_t);
            self.update_areas(delta_t);
            self.update_next_update();
            self.last_time = event.time;
        }

        match event.kind {
            EventKind::BuffApplication {
                buff_id,
                duration,
                ..
            } => {
                if let Some(queue) = self.get_queue(buff_id) {
                    queue.add_stack(duration as u64);
                }
                self.update_next_update();
            }

            // XXX: Properly handle SINGLE and MANUAL removal types
            EventKind::BuffRemove {
                buff_id,
                ..
            } => {
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
    mechanics: HashMap<u64, HashMap<&'static Mechanic, u32>>,
    available_mechanics: Vec<&'static Mechanic>,
    boss_address: u64,
}

impl MechanicTracker {
    /// Create a new mechanic tracker that watches over the given mechanics.
    pub fn new(boss_address: u64, mechanics: Vec<&'static Mechanic>) -> MechanicTracker {
        MechanicTracker {
            mechanics: HashMap::new(),
            available_mechanics: mechanics,
            boss_address: boss_address,
        }
    }
}

impl Tracker for MechanicTracker {
    type Stat = HashMap<u64, HashMap<&'static Mechanic, u32>>;
    type Error = !;

    fn feed(&mut self, event: &Event) -> Result<(), Self::Error> {
        for mechanic in &self.available_mechanics {
            match (&event.kind, &mechanic.1) {
                (EventKind::SkillUse { .. }, Trigger::SkillOnPlayer { .. }) => (),
                _ => (),
            }
        }
        Ok(())
    }

    fn finalize(self) -> Result<Self::Stat, Self::Error> {
        Ok(self.mechanics)
    }
}
