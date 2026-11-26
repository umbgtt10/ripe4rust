# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- The units (`ripe4rust_core::planning::units`), from what cargo and the index will hand over -- the lock moves, the
  workspace's members and lock, the advisories (`ripe4rust_core::inputs`): compatible units, a shared move the
  first's, the rest from what no unit reaches, a move pinned with `hold = "all"` held out unless it clears an
  advisory or a yanked release; breaking units on the version choice; the toolchain unit; groups, by a shared
  repository and the same release on the same day, or an exact requirement; every advisory and yanked release on
  the first unit that clears it, the rest as `advisory:<id>` rows; and the plan's order. Unit ids as the
  vocabulary gives them, `UnitKind` gaining `Toolchain` and `Advisory` and a label.
- The version choice (`ripe4rust_core::planning::choice`), over a crate's or the toolchain's releases as the index
  will hand them over (`ripe4rust_core::inputs::release`): candidates -- neither yanked nor pre-releases, and
  buildable by the toolchain in use --, the versions it holds back that the next toolchain would release, a line's
  releases and their first and last dates, young or settled and active or stale as of the injected date, and a
  breaking unit's target, the newest release of the newest settled line with the younger lines beyond it named.
- Diagnostics (`ripe4rust_core::diagnostics`): `Diagnostic` -- severity, code, where, what, do, needs a person --
  and `DiagnosticCode`, every code `docs/DESIGN.md` names with its stable label, its severity and whether only the
  owner can fix it, five added to the table for failures it left without one: `policy-invalid-value`,
  `cargo-update-failed`, `cargo-metadata-failed`, `workspace-unreadable` and `toolchain-unreachable`.
- The policy read and validated before any rule runs (`ripe4rust_core::planning::pins_and_policy`), from the raw
  tree with locations a port will hand over (`ripe4rust_core::inputs::policy_tree`): `gates`, `published`,
  `validate-beyond-gates`, `freeze-until`, `exposure`, the seven thresholds with their defaults, and the pins with
  `reason`, `review`, `until` -- a crate and a version requirement, or words -- and `hold`. An unknown key is
  `policy-unknown-key`, a value of the wrong kind `policy-invalid-value`, a pin without a reason
  `pin-without-reason`, one with neither a review date nor a checkable condition `pin-without-release`, and no gates
  `gates-undeclared`, each at its `file:line`.
- `ripe4rust_core::values`, the arithmetic the rules are written in, by hand since the core takes no crate: `Version`
  with semver's order and its line (the major, or the minor for `0.x` and the toolchain); `PartialVersion`, a version
  as a requirement or a `rust-version` writes it; `Requirement`, cargo's comparators joined by commas, which reads a
  pin's `until` (`>=0.32`), grouping's exact `=x.y.z` and an advisory's patched ranges, with cargo's rule that a
  pre-release meets a requirement only where it names one; `Date`, civil dates with days added, days between and
  months back, clamped to the shorter month; `Location`, a `file:line`.
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
- `CLAUDE.md`: what ripe4rust is -- the design settled, the implementation in slices --, its boundary, its layout,
  the gates and the score hook, the rules every slice keeps, and the owner's strict rules and coding standards.
- `CODE_OF_CONDUCT.md` and `DONATE.md`, with the two QR images, as the family carries them.
- This changelog.

### Changed
- `ripe4rust-core`'s root carries `#![forbid(unsafe_code)]` beside `#![no_std]`, as the technology-free gate asks;
  the workspace's lints add clippy's `all`, denied, and `pedantic`, warned.
- `docs/DESIGN.md`'s "Delivery" puts slice 1's table and JSON renderers in the cli, where "Architecture" has them.
- `xtask` states its own `rust-version`, 1.88, which its let-chain needs, instead of inheriting the workspace's 1.85.

## [0.0.1] — 2026-10-30

### Added
- `ripe4rust-core` and `cargo-ripe4rust` on crates.io, at the design stage: the core holds the design's vocabulary,
  `Outcome` and `UnitKind`, and nothing that decides; the command says so and exits 2.
