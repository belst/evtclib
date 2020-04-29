# Changelog

All notable changes to this project will be documented in this file.

## Unreleased
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
