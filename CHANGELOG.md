# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- The scaffolding, slice 0 of `docs/DESIGN.md`'s "Delivery": every later slice is held to the family's gates from
  its first commit. `just stage1` -- the lock file synced, `cargo fmt`, clippy pedantic with `-D warnings` on every
  target, the tests -- and `just stage2`, run by a third workspace member, `xtask`, with tests of its own: the
  technology-free core, stern4rust, cargo-shear, dry4rust (floor 25, an empty baseline), crap4rust (threshold 15),
  twin4rust, iceberg4rust (ceiling 7) and cargo-deny, in that order. The compiler pinned at 1.99.1 in
  `rust-toolchain.toml`; `stern4rust.toml` naming `docs/header.txt`; `deny.toml`, the family's licences and sources.
- CI: `.github/workflows/ci.yml` runs both stages on Linux, Windows and macOS, every gate tool installed fresh.
- Every commit message carries core's grip and braintax: `.githooks/commit-msg` runs `cargo xtask grip-braintax`,
  enabled once a clone by `just hooks`.
- Tests for what 0.0.1 holds: `Outcome::label`'s three labels, and the command's exit code 2, its one line on stderr
  and nothing on stdout.
- This changelog.

### Changed
- `ripe4rust-core`'s root carries `#![forbid(unsafe_code)]` beside `#![no_std]`, as the technology-free gate asks;
  the workspace's lints add clippy's `all`, denied, and `pedantic`, warned.

## [0.0.1] — 2026-10-30

### Added
- `ripe4rust-core` and `cargo-ripe4rust` on crates.io, at the design stage: the core holds the design's vocabulary,
  `Outcome` and `UnitKind`, and nothing that decides; the command says so and exits 2.
