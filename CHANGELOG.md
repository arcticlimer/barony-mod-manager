# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.2.0] - 2021-10-09

### Added
- Mod manager now fetches the available mods at startup.

### Changes
- Usage of Steam API key is no longer required for end users.

### Fixes
- Fix bug where mods containg invalid characters for filenames on Windows (such as ":")
  on its names used to crash when saving the mod to disk.
- Fix bug where users could click to download again while the mod was being
    downloaded.
- Mods sorting does not occur at every render anymore, thus improving performance.

## [0.1.0] - 2021-10-09

### Added
- Initial usable release.
