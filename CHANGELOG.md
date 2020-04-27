# Changelog

All notable changes to this project will be documented in this file.

## Unreleased
### Changed
- `evtclib::statistics::gamedata` is now called `evtclib::gamedata`, and the
  list of boons has been removed.

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
