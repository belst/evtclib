# Changelog

All notable changes to this project will be documented in this file.

## Unreleased
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
