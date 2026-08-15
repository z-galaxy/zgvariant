# AGENTS.md

This file provides guidance to AI coding agents when working with code in this repository.

## Project Overview

zgvariant is a serde-based implementation of the [GVariant](https://developer.gnome.org/documentation/specifications/gvariant-specification-1.0.html)
binary serialization format, extracted from the zvariant crate (zbus project). It's a two-crate
workspace: `zgvariant` (the format implementation) and `zgvariant_derive` (thin proc-macro
shells over the codegen shared with zvariant via `zvariant_utils`).

## Conventions

- **Commit style**: gimoji emoji-prefixed commit messages, enforced by commitlint via
  `@gimoji/commitlint-config-gimoji`.
- **Formatting**: `cargo +nightly fmt` (the repo's `.rustfmt.toml` uses nightly-only options).
- **MSRV**: 1.87.
- **Line length**: 100 chars, in code and docs alike.
- **Changelog**: `CHANGELOG.md` is managed by [release-plz] — do not hand-edit it. Write a good
  commit message and release-plz will generate the entry at release time.
- **Changelog-skip trailer**: end a commit message with a `Changelog: skip` git trailer to
  keep it out of the user-facing changelog (use for AI-workflow artifacts such as design docs
  and implementation plans).

[release-plz]: https://release-plz.ieni.dev/
