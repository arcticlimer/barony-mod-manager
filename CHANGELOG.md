# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.3.3] - 2021-10-19

### Fixed
- Fix bug where mods creation date were always the same as their last update date.

### Changes
- Improve user error messages when downloading/removing mods fail.

## [0.3.2] - 2021-10-11

### Fixed
- Fix bug where only Linux releases where shipped into the GitHub releases.

## [0.3.1] - 2021-10-11

### Added
- New versions now are automatically distributed via GitHub and can be found [here](https://github.com/arcticlimer/barony-mod-manager/releases).

## [0.3.0] - 2021-10-11

### Added
- The app window and executable now have icons.
- The download buttons are now colored according to their statuses.
- An user can now filter mods that are still being downloaded.

### Fixes
- Fix bug where sort order was not changing when user selected a different sort strategy.
- The app no longer opens an empty console when starting on Windows.

### Changes
- The app version is now displayed in the window title and the header, along with its name.

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
