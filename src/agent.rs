use std::convert::TryFrom;
use std::marker::PhantomData;

use getset::{CopyGetters, Getters, Setters};
use num_traits::FromPrimitive;

use super::{
    gamedata::{EliteSpec, Profession},
    raw, EvtcError,
};

/// Player-specific agent data.
///
/// Player agents are characters controlled by a player and as such, they contain data about the
/// account and character used (name, profession), as well as the squad composition.
///
/// Note that a `Player` is only the player character itself. Any additional entities that are
/// spawned by the player (clones, illusions, banners, ...) are either a [`Character`][Character]
/// or a [`Gadget`][Gadget].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq, CopyGetters)]
pub struct Player {
    /// The player's profession.
    #[get_copy = "pub"]
    profession: Profession,

    /// The player's elite specialization, if any is equipped.
    #[get_copy = "pub"]
    elite: Option<EliteSpec>,

    character_name: String,

    account_name: String,

    /// The subgroup the player was in.
    #[get_copy = "pub"]
    subgroup: u8,
}

impl Player {
    /// The player's character name.
    pub fn character_name(&self) -> &str {
        &self.character_name
    }

    /// The player's account name.
    ///
    /// This includes the leading colon and the 4-digit denominator.
    pub fn account_name(&self) -> &str {
        &self.account_name
    }
}

/// Gadget-specific agent data.
///
/// Gadgets are entities that are spawned by certain skills. They are mostly inanimate objects that
/// only exist to achieve a certain skill effect.
///
/// Examples of this include the [banners](https://wiki.guildwars2.com/wiki/Banner) spawned by
/// Warriors, but also skill effects like the roots created by
/// [Entangle](https://wiki.guildwars2.com/wiki/Entangle) or the other objects in the arena.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq, CopyGetters)]
pub struct Gadget {
    /// The id of the gadget.
    ///
    /// Note that gadgets do not have true ids and the id is generated "through a combination of
    /// gadget parameters".
    #[get_copy = "pub"]
    id: u16,
    name: String,
}

impl Gadget {
    /// The name of the gadget.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Character-specific agent data.
///
/// Characters are NPCs such as the bosses themselves, additional mobs that they spawn, but also
/// friendly characters like Mesmer's clones and illusions, Necromancer minions, and so on.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq, CopyGetters)]
pub struct Character {
    /// The id of the character.
    #[get_copy = "pub"]
    id: u16,
    name: String,
}

impl Character {
    /// The name of the character.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// The type of an agent.
///
/// arcdps differentiates between three types of agents: [`Player`][Player],
/// [`Character`][Character] and [`Gadget`][Gadget]. This enum unifies handling between them by
/// allowing you to pattern match or use one of the accessor methods.
///
/// The main way to obtain a `AgentKind` is by using the [`.kind()`][Agent::kind] method on an
/// [`Agent`][Agent]. In cases where you already have a [`raw::Agent`][raw::Agent] available, you
/// can also use the [`TryFrom`][TryFrom]/[`TryInto`][std::convert::TryInto] traits to convert a
/// `raw::Agent` or `&raw::Agent` to a `AgentKind`:
///
/// ```no_run
/// # use evtclib::{AgentKind, raw};
/// use std::convert::TryInto;
/// // Get a raw::Agent from somewhere
/// let raw_agent: raw::Agent = panic!();
/// // Convert it
/// let agent: AgentKind = raw_agent.try_into().unwrap();
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AgentKind {
    /// The agent is a player.
    ///
    /// The player-specific data is in the included [`Player`][Player] struct.
    Player(Player),
    /// The agent is a gadget.
    ///
    /// The gadget-specific data is in the included [`Gadget`][Gadget] struct.
    Gadget(Gadget),
    /// The agent is a character.
    ///
    /// The character-specific data is in the included [`Character`][Character] struct.
    Character(Character),
}

impl AgentKind {
    fn from_raw_character(raw_agent: &raw::Agent) -> Result<Character, EvtcError> {
        assert!(raw_agent.is_character());
        let name = raw::cstr_up_to_nul(&raw_agent.name).ok_or(EvtcError::InvalidData)?;
        Ok(Character {
            id: raw_agent.prof as u16,
            name: name.to_str()?.to_owned(),
        })
    }

    fn from_raw_gadget(raw_agent: &raw::Agent) -> Result<Gadget, EvtcError> {
        assert!(raw_agent.is_gadget());
        let name = raw::cstr_up_to_nul(&raw_agent.name).ok_or(EvtcError::InvalidData)?;
        Ok(Gadget {
            id: raw_agent.prof as u16,
            name: name.to_str()?.to_owned(),
        })
    }

    fn from_raw_player(raw_agent: &raw::Agent) -> Result<Player, EvtcError> {
        assert!(raw_agent.is_player());
        let character_name = raw::cstr_up_to_nul(&raw_agent.name)
            .ok_or(EvtcError::InvalidData)?
            .to_str()?;
        let account_name = raw::cstr_up_to_nul(&raw_agent.name[character_name.len() + 1..])
            .ok_or(EvtcError::InvalidData)?
            .to_str()?;
        let subgroup = raw_agent.name[character_name.len() + account_name.len() + 2] - b'0';
        let elite = if raw_agent.is_elite == 0 {
            None
        } else {
            Some(
                EliteSpec::from_u32(raw_agent.is_elite)
                    .ok_or(EvtcError::InvalidEliteSpec(raw_agent.is_elite))?,
            )
        };
        Ok(Player {
            profession: Profession::from_u32(raw_agent.prof)
                .ok_or(EvtcError::InvalidProfession(raw_agent.prof))?,
            elite,
            character_name: character_name.to_owned(),
            account_name: account_name.to_owned(),
            subgroup,
        })
    }

    /// Accesses the inner [`Player`][Player] struct, if available.
    pub fn as_player(&self) -> Option<&Player> {
        if let AgentKind::Player(ref player) = *self {
            Some(player)
        } else {
            None
        }
    }

    /// Determines whether this `AgentKind` contains a player.
    pub fn is_player(&self) -> bool {
        self.as_player().is_some()
    }

    /// Accesses the inner [`Gadget`][Gadget] struct, if available.
    pub fn as_gadget(&self) -> Option<&Gadget> {
        if let AgentKind::Gadget(ref gadget) = *self {
            Some(gadget)
        } else {
            None
        }
    }

    /// Determines whether this `AgentKind` contains a gadget.
    pub fn is_gadget(&self) -> bool {
        self.as_gadget().is_some()
    }

    /// Accesses the inner [`Character`][Character] struct, if available.
    pub fn as_character(&self) -> Option<&Character> {
        if let AgentKind::Character(ref character) = *self {
            Some(character)
        } else {
            None
        }
    }

    /// Determines whether this `AgentKind` contains a character.
    pub fn is_character(&self) -> bool {
        self.as_character().is_some()
    }
}

impl TryFrom<raw::Agent> for AgentKind {
    type Error = EvtcError;
    /// Convenience method to avoid manual borrowing.
    ///
    /// Note that this conversion will consume the agent, so if you plan on re-using it, use the
    /// `TryFrom<&raw::Agent>` implementation that works with a reference.
    fn try_from(raw_agent: raw::Agent) -> Result<Self, Self::Error> {
        Self::try_from(&raw_agent)
    }
}

impl TryFrom<&raw::Agent> for AgentKind {
    type Error = EvtcError;

    /// Extract the correct `AgentKind` from the given [raw agent][raw::Agent].
    ///
    /// This automatically discerns between player, gadget and characters.
    ///
    /// Note that in most cases, you probably want to use `Agent::try_from` or even
    /// [`process`][super::process] instead of this function.
    fn try_from(raw_agent: &raw::Agent) -> Result<Self, Self::Error> {
        if raw_agent.is_character() {
            Ok(AgentKind::Character(AgentKind::from_raw_character(
                raw_agent,
            )?))
        } else if raw_agent.is_gadget() {
            Ok(AgentKind::Gadget(AgentKind::from_raw_gadget(raw_agent)?))
        } else if raw_agent.is_player() {
            Ok(AgentKind::Player(AgentKind::from_raw_player(raw_agent)?))
        } else {
            Err(EvtcError::InvalidData)
        }
    }
}

/// An agent.
///
/// Agents in arcdps are very versatile, as a lot of things end up being an "agent". This includes:
/// * Players
/// * Bosses
/// * Any additional mobs that spawn
/// * Mesmer illusions
/// * Ranger spirits, pets
/// * Guardian spirit weapons
/// * ...
///
/// Generally, you can divide them into three kinds ([`AgentKind`][AgentKind]):
/// * [`Player`][Player]: All players themselves.
/// * [`Character`][Character]: Non-player mobs, including most bosses, "adds" and player-generated
///   characters.
/// * [`Gadget`][Gadget]: Some additional gadgets, such as ley rifts, continuum split, ...
///
/// All of these agents share some common fields, which are the ones accessible in `Agent<Kind>`.
/// The kind can be retrieved using [`.kind()`][Agent::kind], which can be matched on.
///
/// # Obtaining an agent
///
/// The normal way to obtain the agents is to use the [`.agents()`](super::Log::agents) method on a
/// [`Log`][super::Log], or one of the other accessor methods (like
/// [`.players()`][super::Log::players] or [`.agent_by_addr()`][super::Log::agent_by_addr]).
///
/// In the cases where you already have a [`raw::Agent`][raw::Agent] available, you can also
/// convert it to an [`Agent`][Agent] by using the standard
/// [`TryFrom`][TryFrom]/[`TryInto`][std::convert::TryInto] traits:
///
/// ```no_run
/// # use evtclib::{Agent, raw};
/// use std::convert::TryInto;
/// let raw_agent: raw::Agent = panic!();
/// let agent: Agent = raw_agent.try_into().unwrap();
/// ```
///
/// Note that you can convert references as well, so if you plan on re-using the raw agent
/// afterwards, you should opt for `Agent::try_from(&raw_agent)` instead.
///
/// # The `Kind` parameter
///
/// The type parameter is not actually used and only exists at the type level. It can be used to
/// tag `Agent`s containing a known kind. For example, `Agent<Player>` implements
/// [`.player()`][Agent::player], which returns a `&Player` directly (instead of a
/// `Option<&Player>`). This works because such tagged `Agent`s can only be constructed (safely)
/// using [`.as_player()`][Agent::as_player], [`.as_gadget()`][Agent::as_gadget] or
/// [`.as_character()`][Agent::as_character]. This is useful since functions like
/// [`Log::players`][super::Log::players], which already filter only players, don't require the
/// consumer to do another check/pattern match for the right agent kind.
///
/// The unit type `()` is used to tag `Agent`s which contain an undetermined type, and it is the
/// default if you write `Agent` without any parameters.
///
/// The downside is that methods which work on `Agent`s theoretically should be generic over
/// `Kind`. An escape hatch is the method [`.erase()`][Agent::erase], which erases the kind
/// information and produces the default `Agent<()>`. Functions/methods that only take `Agent<()>`
/// can therefore be used by any other agent as well.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Getters, CopyGetters, Setters)]
// For the reasoning of #[repr(C)] see Agent::transmute.
#[repr(C)]
pub struct Agent<Kind = ()> {
    /// The address of this agent.
    ///
    /// This is not actually the address of the in-memory Rust object, but rather a serialization
    /// detail of arcdps. You should consider this as an opaque number and only compare it to other
    /// agent addresses.
    #[getset(get_copy = "pub", set = "pub(crate)")]
    addr: u64,

    /// The kind of this agent.
    #[getset(get = "pub", set = "pub(crate)")]
    kind: AgentKind,

    /// The toughness of this agent.
    ///
    /// This is not an absolute number, but a relative indicator that indicates this agent's
    /// toughness relative to the other people in the squad.
    ///
    /// 0 means lowest toughness, 10 means highest toughness.
    #[get_copy = "pub"]
    toughness: i16,

    /// The concentration of this agent.
    ///
    /// This is not an absolute number, but a relative indicator that indicates this agent's
    /// concentration relative to the other people in the squad.
    ///
    /// 0 means lowest concentration, 10 means highest concentration.
    #[getset(get_copy = "pub", set = "pub(crate)")]
    concentration: i16,

    /// The healing power of this agent.
    ///
    /// This is not an absolute number, but a relative indicator that indicates this agent's
    /// healing power relative to the other people in the squad.
    ///
    /// 0 means lowest healing power, 10 means highest healing power.
    #[getset(get_copy = "pub", set = "pub(crate)")]
    healing: i16,

    /// The condition damage of this agent.
    ///
    /// This is not an absolute number, but a relative indicator that indicates this agent's
    /// condition damage relative to the other people in the squad.
    ///
    /// 0 means lowest condition damage, 10 means highest condition damage.
    #[getset(get_copy = "pub", set = "pub(crate)")]
    condition: i16,

    /// The instance ID of this agent.
    #[getset(get_copy = "pub", set = "pub(crate)")]
    instance_id: u16,

    /// The timestamp of the first event entry with this agent.
    #[getset(get_copy = "pub", set = "pub(crate)")]
    first_aware: u64,

    /// The timestamp of the last event entry with this agent.
    #[getset(get_copy = "pub", set = "pub(crate)")]
    last_aware: u64,

    /// The master agent's address.
    #[getset(get_copy = "pub", set = "pub(crate)")]
    master_agent: Option<u64>,

    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    phantom_data: PhantomData<Kind>,
}

// We could derive this, however that would derive Deserialize generically for Agent<T>, where T is
// deserializable. In particular, this would mean that you could deserialize Agent<Character>,
// Agent<Player> and Agent<Gadget> directly, which would not be too bad - the problem is that serde
// has no way of knowing if the serialized agent actually is a character/player/gadget agent, as
// that information only exists on the type level. This meant that you could "coerce" agents into
// the wrong type, safely, using a serialization followed by a deserialization.
// Now this doesn't actually lead to memory unsafety or other bad behaviour, but it could mean that
// the program would panic if you tried to access a non-existing field, e.g. by calling
// Agent<Character>::id() on a non-character agent.
// In order to prevent this, we manually implement Deserialize only for Agent<()>, so that the
// usual conversion functions with the proper checks have to be used.
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Agent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Addr,
            Kind,
            Toughness,
            Concentration,
            Healing,
            Condition,
            InstanceId,
            FirstAware,
            LastAware,
            MasterAgent,
        };

        struct AgentVisitor;

        impl<'de> Visitor<'de> for AgentVisitor {
            type Value = Agent;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Agent")
            }

            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Agent, V::Error> {
                let mut addr = None;
                let mut kind = None;
                let mut toughness = None;
                let mut concentration = None;
                let mut healing = None;
                let mut condition = None;
                let mut instance_id = None;
                let mut first_aware = None;
                let mut last_aware = None;
                let mut master_agent = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Addr => {
                            if addr.is_some() {
                                return Err(de::Error::duplicate_field("addr"));
                            }
                            addr = Some(map.next_value()?);
                        }
                        Field::Kind => {
                            if kind.is_some() {
                                return Err(de::Error::duplicate_field("kind"));
                            }
                            kind = Some(map.next_value()?);
                        }
                        Field::Toughness => {
                            if toughness.is_some() {
                                return Err(de::Error::duplicate_field("toughness"));
                            }
                            toughness = Some(map.next_value()?);
                        }
                        Field::Concentration => {
                            if concentration.is_some() {
                                return Err(de::Error::duplicate_field("concentration"));
                            }
                            concentration = Some(map.next_value()?);
                        }
                        Field::Healing => {
                            if healing.is_some() {
                                return Err(de::Error::duplicate_field("healing"));
                            }
                            healing = Some(map.next_value()?);
                        }
                        Field::Condition => {
                            if condition.is_some() {
                                return Err(de::Error::duplicate_field("condition"));
                            }
                            condition = Some(map.next_value()?);
                        }
                        Field::InstanceId => {
                            if instance_id.is_some() {
                                return Err(de::Error::duplicate_field("instance_id"));
                            }
                            instance_id = Some(map.next_value()?);
                        }
                        Field::FirstAware => {
                            if first_aware.is_some() {
                                return Err(de::Error::duplicate_field("first_aware"));
                            }
                            first_aware = Some(map.next_value()?);
                        }
                        Field::LastAware => {
                            if last_aware.is_some() {
                                return Err(de::Error::duplicate_field("last_aware"));
                            }
                            last_aware = Some(map.next_value()?);
                        }
                        Field::MasterAgent => {
                            if master_agent.is_some() {
                                return Err(de::Error::duplicate_field("master_agent"));
                            }
                            master_agent = Some(map.next_value()?);
                        }
                    }
                }

                Ok(Agent {
                    addr: addr.ok_or_else(|| de::Error::missing_field("addr"))?,
                    kind: kind.ok_or_else(|| de::Error::missing_field("kind"))?,
                    toughness: toughness.ok_or_else(|| de::Error::missing_field("toughness"))?,
                    concentration: concentration
                        .ok_or_else(|| de::Error::missing_field("concentration"))?,
                    healing: healing.ok_or_else(|| de::Error::missing_field("healing"))?,
                    condition: condition.ok_or_else(|| de::Error::missing_field("condition"))?,
                    instance_id: instance_id
                        .ok_or_else(|| de::Error::missing_field("instance_id"))?,
                    first_aware: first_aware
                        .ok_or_else(|| de::Error::missing_field("first_aware"))?,
                    last_aware: last_aware.ok_or_else(|| de::Error::missing_field("last_aware"))?,
                    master_agent: master_agent
                        .ok_or_else(|| de::Error::missing_field("master_agent"))?,
                    phantom_data: PhantomData,
                })
            }
        };

        const FIELDS: &[&'static str] = &[
            "addr",
            "kind",
            "toughness",
            "concentration",
            "healing",
            "condition",
            "instance_id",
            "first_aware",
            "last_aware",
            "master_agent",
        ];
        deserializer.deserialize_struct("Agent", FIELDS, AgentVisitor)
    }
}

impl TryFrom<&raw::Agent> for Agent {
    type Error = EvtcError;

    /// Parse a raw agent.
    fn try_from(raw_agent: &raw::Agent) -> Result<Self, Self::Error> {
        let kind = AgentKind::try_from(raw_agent)?;
        Ok(Agent {
            addr: raw_agent.addr,
            kind,
            toughness: raw_agent.toughness,
            concentration: raw_agent.concentration,
            healing: raw_agent.healing,
            condition: raw_agent.condition,
            instance_id: 0,
            first_aware: 0,
            last_aware: u64::max_value(),
            master_agent: None,
            phantom_data: PhantomData,
        })
    }
}

impl TryFrom<raw::Agent> for Agent {
    type Error = EvtcError;

    /// Convenience method to avoid manual borrowing.
    ///
    /// Note that this conversion will consume the agent, so if you plan on re-using it, use the
    /// `TryFrom<&raw::Agent>` implementation that works with a reference.
    fn try_from(raw_agent: raw::Agent) -> Result<Self, Self::Error> {
        Agent::try_from(&raw_agent)
    }
}

impl<Kind> Agent<Kind> {
    /// Unconditionally change the tagged type.
    #[inline]
    fn transmute<T>(&self) -> &Agent<T> {
        // Beware, unsafe code ahead!
        //
        // What are we doing here?
        // In Agent<T>, T is a marker type that only exists at the type level. There is no actual
        // value of type T being held, instead, we use PhantomData under the hood. This is so we
        // can implement special methods on Agent<Player>, Agent<Gadget> and Agent<Character>,
        // which allows us in some cases to avoid the "second check" (e.g. Log::players() can
        // return Agent<Player>, as the function already makes sure all returned agents are
        // players). This makes the interface more ergonomical, as we can prove to the type checker
        // at compile time that a given Agent has a certain AgentKind.
        //
        // Why is this safe?
        // PhantomData<T> (which is what Agent<T> boils down to) is a zero-sized type, which means
        // it does not actually change the layout of the struct. There is some discussion in [1],
        // which suggests that this is true for #[repr(C)] structs (which Agent is). We can
        // therefore safely transmute from Agent<U> to Agent<T>, for any U and T.
        //
        // Can this lead to unsafety?
        // No, the actual data access is still done through safe rust and a if-let. In the worst
        // case it can lead to an unexpected panic, but the "guarantee" made by T is rather weak in
        // that regard.
        //
        // What are the alternatives?
        // None, as far as I'm aware. Going from Agent<U> to Agent<T> is possible in safe Rust by
        // destructuring the struct, or alternatively by [2] (if it would be implemented). However,
        // when dealing with references, there seems to be no way to safely go from Agent<U> to
        // Agent<T>, even if they share the same layout.
        //
        // [1]: https://www.reddit.com/r/rust/comments/avrbvc/is_it_safe_to_transmute_foox_to_fooy_if_the/
        // [2]: https://github.com/rust-lang/rfcs/pull/2528
        unsafe { &*(self as *const Agent<Kind> as *const Agent<T>) }
    }

    /// Erase any extra information about the contained agent kind.
    #[inline]
    pub fn erase(&self) -> &Agent {
        self.transmute()
    }

    /// Try to convert this `Agent` to an `Agent` representing a `Player`.
    #[inline]
    pub fn as_player(&self) -> Option<&Agent<Player>> {
        if self.kind.is_player() {
            Some(self.transmute())
        } else {
            None
        }
    }

    /// Try to convert this `Agent` to an `Agent` representing a `Gadget`.
    #[inline]
    pub fn as_gadget(&self) -> Option<&Agent<Gadget>> {
        if self.kind.is_gadget() {
            Some(self.transmute())
        } else {
            None
        }
    }

    /// Try to convert this `Agent` to an `Agent` representing a `Character`.
    #[inline]
    pub fn as_character(&self) -> Option<&Agent<Character>> {
        if self.kind.is_character() {
            Some(self.transmute())
        } else {
            None
        }
    }
}

impl Agent<Player> {
    /// Directly access the underlying player data.
    #[inline]
    pub fn player(&self) -> &Player {
        self.kind.as_player().expect("Agent<Player> had no player!")
    }

    /// Shorthand to get the player's account name.
    #[inline]
    pub fn account_name(&self) -> &str {
        self.player().account_name()
    }

    /// Shorthand to get the player's character name.
    #[inline]
    pub fn character_name(&self) -> &str {
        self.player().character_name()
    }

    /// Shorthand to get the player's elite specialization.
    #[inline]
    pub fn elite(&self) -> Option<EliteSpec> {
        self.player().elite()
    }

    /// Shorthand to get the player's profession.
    #[inline]
    pub fn profession(&self) -> Profession {
        self.player().profession()
    }

    /// Shorthand to get the player's subgroup.
    #[inline]
    pub fn subgroup(&self) -> u8 {
        self.player().subgroup()
    }
}

impl Agent<Gadget> {
    /// Directly access the underlying gadget data.
    #[inline]
    pub fn gadget(&self) -> &Gadget {
        self.kind.as_gadget().expect("Agent<Gadget> had no gadget!")
    }

    /// Shorthand to get the gadget's id.
    #[inline]
    pub fn id(&self) -> u16 {
        self.gadget().id()
    }

    /// Shorthand to get the gadget's name.
    #[inline]
    pub fn name(&self) -> &str {
        self.gadget().name()
    }
}

impl Agent<Character> {
    /// Directly access the underlying character data.
    #[inline]
    pub fn character(&self) -> &Character {
        self.kind
            .as_character()
            .expect("Agent<Character> had no character!")
    }

    /// Shorthand to get the character's id.
    #[inline]
    pub fn id(&self) -> u16 {
        self.character().id()
    }

    /// Shorthand to get the character's name.
    #[inline]
    pub fn name(&self) -> &str {
        self.character().name()
    }
}

#[cfg(all(feature = "serde", test))]
mod tests {
    use super::*;

    fn agent() -> Agent {
        Agent {
            addr: 0xdeadbeef,
            kind: AgentKind::Character(Character {
                id: 0xf00,
                name: "Foo Bar".into(),
            }),
            toughness: -13,
            concentration: -14,
            healing: -15,
            condition: -16,
            instance_id: 1337,
            first_aware: 0,
            last_aware: 0xffffff,
            master_agent: None,
            phantom_data: PhantomData,
        }
    }

    #[test]
    fn serialization() {
        let agent = agent();
        let json = serde_json::to_string(&agent).unwrap();
        let expected = r#"{"addr":3735928559,"kind":{"Character":{"id":3840,"name":"Foo Bar"}},"toughness":-13,"concentration":-14,"healing":-15,"condition":-16,"instance_id":1337,"first_aware":0,"last_aware":16777215,"master_agent":null}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn deserialization() {
        let json = r#"{"addr":3735928559,"kind":{"Character":{"id":3840,"name":"Foo Bar"}},"toughness":-13,"concentration":-14,"healing":-15,"condition":-16,"instance_id":1337,"first_aware":0,"last_aware":16777215,"master_agent":null}"#;
        let deserialized: Agent = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, agent());
    }

    #[test]
    #[should_panic(expected = "missing field `master_agent`")]
    fn deserialization_missing_field() {
        let json = r#"{"addr":3735928559,"kind":{"Character":{"id":3840,"name":"Foo Bar"}},"toughness":-13,"concentration":-14,"healing":-15,"condition":-16,"instance_id":1337,"first_aware":0,"last_aware":16777215}"#;
        serde_json::from_str::<Agent>(json).unwrap();
    }

    #[test]
    #[should_panic(expected = "duplicate field `master_agent`")]
    fn deserialization_duplicated_field() {
        let json = r#"{"addr":3735928559,"kind":{"Character":{"id":3840,"name":"Foo Bar"}},"toughness":-13,"concentration":-14,"healing":-15,"condition":-16,"instance_id":1337,"first_aware":0,"last_aware":16777215,"master_agent":null,"master_agent":null}"#;
        serde_json::from_str::<Agent>(json).unwrap();
    }
}
