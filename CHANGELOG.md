# Changelog

All notable changes to this project will be documented in this file.

## Unreleased

## 0.7.3 - 2022-05-11
### Added
- Support for CM detection in the Xunlai Jade Junkyard strike (Ankka)

## 0.7.2 - 2022-04-20
### Added
- Support for Aetherblade Hideout logs with the Challenge Mote active:
  - Success detection for CM logs
  - CM detection

### Fixed
- Success detection for Mai Trin logs that ended before the Echo of Scarlet
  Briar spawned.

## 0.7.1 - 2022-04-01
### Added
- Various analyzers for the End of Dragons strike missions:
  - `analyzers::strikes::CaptainMaiTrin`
  - `analyzers::strikes::Ankka`
  - `analyzers::strikes::MinisterLi`
  - `analyzers::strikes::Dragonvoid`

### Fixed
- Success/failure detection for the End of Dragons strike missions.
- Some Dragonvoid logs not being recognized as such.

## 0.7.0 - 2022-03-10
### Added
- `EliteSpec`s for the new End of Dragons specializations
- `Boss` and `Encounter` for the new strikes
  - `{Boss, Encounter}::CaptainMaiTrin`
  - `{Boss, Encounter}::Ankka`
  - `{Boss, Encounter}::MinisterLi`
  - `{Boss, Encounter}::Dragonvoid`

## 0.6.1 - 2021-11-25
### Added
- `Encounter::TwistedCastle` to identify twisted castle logs.

## 0.6.0 - 2021-11-19
### Added
- Boss and encounter definitions for the training golems (`StandardKittyGolem`,
  `MediumKittyGolem`, `LargeKittyGolem`).
- Missing encounters and bosses:
  - `Encounter::BanditTrio` (`Boss::Berg`, `Boss::Zane`, `Boss::Narella`)
  - `Encounter::RiverOfSouls` (no bosses)
  - `Encounter::BrokenKing` (`Boss::BrokenKing`)
  - `Encounter::EaterOfSouls` (`Boss::EaterOfSouls`)
  - `Encounter::StatueOfDarkness` (`Boss::EyeOfJudgment` and `Boss::EyeOfFate`)
- `Log::is_generic` to check whether a log is generic (WvW).
- `gamedata::GameMode` and the `Encounter::game_mode` and `Log::game_mode`
  methods.
- `FromRawEventError::UnknownLanguage` has been added to deal with an invalid
  language byte.
- The `CbtStateChange::BarrierUpdate` and `CbtResult::Breakbar` low-level
  variants.
- The `EventKind::{BuffInitial, StackActive, StackReset}` events.

### Changes
- Internal changes that lead to some small speedups, especially in the
  *Soulless Horror* outcome analyzer (~50x speedup) as well as most other
  analyzers (up to a 2.77x speedup).
- An internal parsing change to speed up log processing (by around 50%).
- `EventKind` has been marked non-exhaustive.

### Fixed
- `evtclib` will no longer choke on WvW logs where player names might not contain the expected
  information.

## 0.5.0 - 2020-10-07
### Added
- `Boss::Ai` to represent Ai, Keeper of the Peak in the Sunqua Peak fractal.
- `analyzers::fractal::Ai` with logic to determine CM and outcome of the
  Sunqua Peak CM fight.
- `Log::gadgets` to retrieve all gadget agents.
- `Log::build_id` to retrieve the game's build id.
- The `serde` optional feature to enable (de)serialization of API types.
- `Encounter::from_header_id` to convert a header ID from arcdps to the correct
  encounter.

### Changed
- `gamedata::Boss` has been split in `gamedata::Boss` and `gamedata::Encounter`
  - `Encounter::VoiceOfTheFallen` is now `Encounter::SuperKodanBrothers`
  - `Encounter::LargosTwins` is now `Encounter::TwinLargos`
  - `Boss::Xera2`, `Boss::Nikare`, `Boss::Kenut`, `Boss::ClawOfTheFallen` and
    `Boss::VoiceOfTheFallen` have been introduced
- `gamedata::Boss` is no longer re-exported as `evtclib::Boss`, instead
  `evtclib::Encounter` is exported.
- Renamed `Log::npcs` to `Log::characters` to have consistent naming.

### Fixed
- Some edge cases where raid success was not detected (as long as the fight was
  rewarded).

### Removed
- Various `*_ID` constants from `gamedata`: `XERA_PHASE2_ID`, `NIKARE_ID`,
  `KENUT_ID`, `VOICE_OF_THE_FALLEN_ID` and `CLAW_OF_THE_FALLEN_ID`.

## 0.4.3 - 2020-09-21
### Added
- `gamedata::VOICE_OF_THE_FALLEN_ID` and `gamedata::CLAW_OF_THE_FALLEN_ID`.

### Fixed
- Handling of log files with "Claw of the Fallen" as the encounter id.
- Both bosses are now returned for the "Voice & Claw of the Fallen" strike
  mission.
- Fixed CM detection for Skorvald logs done after the 2020-09-15 patch
  (introduction of Sunqua Peak).

## 0.4.2 - 2020-08-28
### Fixed
- Removed leftover debug output ("First aware: ...") from the Deimos analyzer.

## 0.4.1 - 2020-08-17
### Added
- `Log::errors` as a convenience function.

### Fixed
- Fixed the conversion from `CBTS_ERROR` to `EventKind::Error` not having the
  correct text.

## 0.4.0 - 2020-07-24
### Added
- A variant for `CBTS_TAG`.
- The function `Log::span` to get the duration of a log.
- Analyzers to detect fight outcomes and challenge motes in a fight-dependent
  way.
- `gamedata::KENUT_ID` and `gamedata::NIKARE_ID` for the Largos Twins' IDs.

### Fixed
- `Log::is_boss` and `Log::boss_agents` now properly work with both Largos in
  the Twin Largos fight.

### Removed
- `CmTrigger` and `Boss::cm_trigger`, as that is now handled by analyzers.

## 0.3.3 - 2020-05-25
### Added
- Variants for `CBTS_BREAKBARSTATE`, `CBTS_BREAKBARPERCENT` and `CBTS_ERROR`.
- `EventKind::Error` as the higher-level part for `CBTS_ERROR`.
- Equivalents for the  `e_attribute` and `e_buffcategory` enums.

### Changed
- Invalid state changes no longer cause the parser to choke, instead they are
  ignored in `parse_events`.

## 0.3.2 - 2020-05-12
### Added
- Support for determining Challenge Motes.
  - `evtclib::gamedata::CmTrigger` along with
    `evtclib::gamedata::Boss::cm_trigger`.
  - `evtclib::Log::is_cm`.
- Convenience methods `evtclib::process_stream` and `evtclib::process_file`.
- `Display` implementation for `Boss`, `Profession` and `EliteSpec`.

## 0.3.1 - 2020-05-04
### Added
- Implement `FromStr` for `Profession` and `EliteSpec`.

### Changed
- Removed dependency on `fnv`.

## 0.3.0 - 2020-05-02
### Added
- Implement standard traits `Debug`, `Default`, `PartialEq`, `Eq` and `Hash`
  for raw types in `evtclib::raw`.
- Implement `From<Evtc>` for `PartialEvtc`.

### Changed
- Parsing functions now take their input by-value.
- `evtclib::Event` now provides getters instead of public fields.

## 0.2.0 - 2020-04-29
### Added
- `Hash`, `PartialEq` and `Eq` implementations for `Agent`.
- `Hash` implementation for `WeaponSet` and `Activation`.
- `evtclib::gamedata::{Profession, EliteSpec}` to make dealing with
  profession/elite specialization ids easier.
- `evtclib::Log::encounter` to automatically convert the ID to the right
  `Boss`.
- `evtclib::raw::cstr_up_to_nul` to make dealing with the embedded strings
  easier.
- Handling for `CBTS_VELOCITY`, `CBTS_POSITION`, `CBTS_FACING`, `CBTS_MAPID`,
  `CBTS_ATTACKTARGET` and `CBTS_TARGETABLE` events.
- Convenience methods to `Log`: `local_start_timestamp`, `local_end_timestamp`,
  `was_rewarded`.

### Changed
- `evtclib::statistics::gamedata` is now called `evtclib::gamedata`, and the
  list of boons has been removed.
- `evtclib::Agent` now takes a `Kind` parameter which make some methods more
  ergonomical to use.
- `evtclib::Agent{Name, Kind}` have been reworked:
  - They have been consolidated into a single `AgentKind`, which also contains
    the name.
  - Three new structs `Player`, `Gadget` and `Character` have been added
    instead of embedding the fields directly into the enum.
  - `Player::profession()` and `Player::elite()` now use the new `Profession`
    and `EliteSpec` enums.
- `evtclib::Log::boss_id()` has been renamed `encounter_id`.
- `evtclib::EvtcError::Utf8Error` has changed the inner type from
  `FromUtf8Error` to `Utf8Error`.
- The submodule `evtclib::event` is now publicly accessible.
- Structs are now converted using `TryFrom` instead of our custom `from_raw`
  method.

### Fixed
- Fixes for parsing `evtclib::gamedata::Boss`:
  - "soulless horror" will now be parsed correctly as `Boss::SoullessHorror`.
  - "largos twins" will now be parsed correctly as `Boss::LargosTwins`.
  - "ensolyss of the endless torment" will now be parsed correctly as
    `Boss::Ensolyss`.
  - "kodans" will now be parsed as `Boss::VoiceOfTheFallen`.
  - "conjured amalgamate" will now be parsed correctly as
    `Boss::ConjuredAmalgamate`. The typo in "conjured almagamate" has been
    fixed.

### Removed
- Removed `evtclib::statistics` submodule, see `08465ea` for the rationale.
- Removed all feature flags, so the crate can now be used on stable.
- Removed `Eq` from `evtclib::Event` & `evtclib::EventKind`.
- `main.rs` is gone.

### Unsafe
- An unsafe one-liner has been added in `Agent::transmute`. Rationale and a
  comment about safety can be found in the source.
