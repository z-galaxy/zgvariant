# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## 1.0.0 - 2026-08-16

### Added
- ✨ Port the GVariant codec from zvariant.
- ✨ Port the Value system from zvariant.
- ✨ Port the foundation and type system from zvariant.

### Changed
- 🚚 Drop the Format parameter from Basic::alignment.
- 🏗️ Scaffold the zgvariant/zgvariant_derive workspace.

### Dependencies
- ⬆️ Switch zvariant_utils to the crates.io release.
- ⬆️ Retarget zvariant_utils git dependency to z-galaxy/zbus.

### Documentation
- 📝 Write the README, migration guide and derive docs.

### Fixed
- 🐛 Fix inherited soundness and robustness issues.
- 🐛 Fix dict-entry framing-offset width at size boundaries.
- 🩹 Address final-review findings.

### Other
- ✏️ Fix typos inherited from zvariant.

### Removed
- 🔥 Drop the born-deprecated vec_to_cstr helper.

### Testing
- ✅ Port gvariant fuzzing from zvariant.
- ✅ Port the gvariant benchmarks from zvariant.
- ✅ Port the gvariant test-suite from zvariant.
