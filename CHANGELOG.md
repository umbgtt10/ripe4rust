# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- `ripe4rust_core::values`, the arithmetic the rules are written in, by hand since the core takes no crate: `Version`
  with semver's order and its line (the major, or the minor for `0.x` and the toolchain); `PartialVersion`, a version
  as a requirement or a `rust-version` writes it; `Requirement`, cargo's comparators joined by commas, which reads a
  pin's `until` (`>=0.32`), grouping's exact `=x.y.z` and an advisory's patched ranges, with cargo's rule that a
  pre-release meets a requirement only where it names one; `Date`, civil dates with days added, days between and
  months back, clamped to the shorter month; `Location`, a `file:line`.
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
- The version choice (`ripe4rust_core::planning::choice`), over a crate's or the toolchain's releases as the index
  will hand them over (`ripe4rust_core::inputs::release`): candidates -- neither yanked nor pre-releases, and
  buildable by the toolchain in use --, the versions it holds back that the next toolchain would release, a line's
  releases and their first and last dates, young or settled and active or stale as of the injected date, and a
  breaking unit's target, the newest release of the newest settled line with the younger lines beyond it named.
- The units (`ripe4rust_core::planning::units`), from what cargo and the index will hand over -- the lock moves, the
  workspace's members and lock, the advisories (`ripe4rust_core::inputs`): compatible units, a shared move the
  first's, the rest from what no unit reaches, a move pinned with `hold = "all"` held out unless it clears an
  advisory or a yanked release; breaking units on the version choice; the toolchain unit; groups, by a shared
  repository and the same release on the same day, or an exact requirement; every advisory and yanked release on
  the first unit that clears it, the rest as `advisory:<id>` rows; and the plan's order. Unit ids as the
  vocabulary gives them, `UnitKind` gaining `Toolchain` and `Advisory` and a label.
- The rules (`ripe4rust_core::planning::judging`): all twelve, each a `Rule` (`ripe4rust_core::traits::rule`) of
  its own over a unit's facts, applied in order with the first match deciding; the verdict for a breaking unit that
  asks -- migrate now, plan it, wait --, also carried by rules 7 and 8; against drift, a hold of rule 4 or 9 asked
  overripe two lines behind or on a stale line, a freeze still holding; the no-gates downgrade; the advisory row,
  asked and urgent by no rule. Every decision carries its rule, its suggestion, short reasons for the table and,
  for a unit that asks, why.
- The steps and the questions (`ripe4rust_core::planning::writing`): every unit's `apply`, `check`, `gates`, `revert`
  and `split` -- one recursive update, or one `--precise` update per move where a move is held out and for the rest; a
  breaking unit's requirement lines and raised `rust-version`s, then its update; the toolchain's channel; a crate the
  lock holds twice named at its version, each member of a breaking group at its own --, and the question of every unit
  that asks, its options by the way it asks, each with what it means and the step that carries it out.
- The plan made from a survey of the workspace (`ripe4rust_core::planning`): `Surveyed`, everything the ports will
  answer, read once; `PlanWriter`, which builds, groups and orders the units, lands every advisory and yanked
  release, and writes every row; the facts each unit is judged on (`planning::judging::facts`) -- its lines, its
  pins and whether they are released, the targets it reaches, its exposure and the `rust-version`s it raises, its
  footprint, a second copy migrating removes, what the toolchain would release --; each row's evidence, summary and
  edits; and `Plan`, `PlannedUnit` and `PlanFailure`, the model the renderers print.
- `plan` in the core, against its ports (`ripe4rust_core::traits`): the index, the repositories, the lock moves,
  the workspace's metadata and files, the advisories, the toolchain, the source scan and the clock, each a trait
  the cli will implement in slice 2, faked in the tests. `Survey` reads every port once -- the policy first, its
  errors stopping the plan before any other port is read -- and a port that cannot answer stops it as the
  diagnostic of its own code (`PortFailure`), with every diagnostic found so far; `Planner` makes the plan from the
  survey. The notices and warnings the survey meets: `toolchain-unpinned`, `target-unknown`,
  `pin-for-absent-crate`, `registry-cache-old` and `source-unparsed`.
- The renderers, in the cli (`cargo_ripe4rust::render`), a library target beside the command:
  the JSON document -- `schema`, `command`, `date`, `registry_read`, `workspace`, `gates`, `diagnostics` and
  `units`, each unit with its moves, outcome, rule, suggestion, reasons, summary, steps, question and evidence; on
  failure, `schema`, `command` and `diagnostics` -- and the table for a person, the diagnostics after it in text.
- What a plan does not know yet, said rather than guessed: the source scan's port answers `None` while the sources
  are not parsed, and the toolchain's releases `None` while the channel manifests are not read. A footprint not
  known is never small -- rule 11 calls nothing ripe, the verdict waits -- and a breaking unit a published member
  uses asks under rule 8, its exposure not yet known; with no releases no toolchain unit is offered. The plan says
  so in two notices, `source-unscanned` and `toolchain-releases-unknown`.
- Every object of the JSON document written in the design's order, at every depth -- a record of its own each
  (`cargo_ripe4rust::render::json::documents`), serialized as declared -- where the nested objects were written
  with their keys sorted. serde_json's `preserve_order` stays off: it would bring a crate under a licence
  `deny.toml` refuses.
- The seams every adapter reaches the system through, in the cli (`cargo_ripe4rust::system::traits`): `CommandRunner`, a
  program run to its end with what it printed, `FileSystem`, a file's text, a directory's entries and a file written
  whole, and `Pause`, a wait between requests; their real implementations (`cargo_ripe4rust::system::real`) run a child
  process without colour, write a file beside its place and rename it, and sleep the thread. A program not
  installed is `LaunchFailure::Missing`, a file not there `FileFailure::Missing`.
- `cargo metadata`, read (`cargo_ripe4rust::adapters::cargo::cargo_metadata`): the members, each with its crates.io
  dependencies and the requirement that asked for each, and the lock's crates.io packages, from
  `cargo metadata --format-version 1 --locked`; the crates a `validate-beyond-gates` target reaches, from the same
  with `--filter-platform`. `--locked` leaves the lock file alone; `--offline` follows an offline plan. A failure is
  `cargo-metadata-failed` with cargo's error on one line, output that does not read `internal`.
- `cargo update --dry-run`, read strictly (`cargo_ripe4rust::adapters::cargo::cargo_update`): a direct dependency's moves
  with `--recursive -p <crate>@<version>`, the workspace's without. Every line is a move -- updated, downgraded,
  added, removed, with no note or one of cargo's own -- or a status that moves nothing; any other line is
  `cargo-update-unrecognised`, quoting it, and a failure `cargo-update-failed`. Tested on cargo 1.99.1's own
  output, recorded from scratch workspaces.
- The advisories, from cargo-deny (`cargo_ripe4rust::adapters::cargo::deny`): those in force from its JSON log,
  without what `deny.toml` ignores, each with its `patched` and `unaffected` ranges from its audit-compatible
  report. `cargo-deny-missing` where cargo has no `deny` command, `cargo-deny-failed` with the error of its log on
  one line where it could not check. The cli's modules regrouped, at most five folders to a directory:
  `adapters/` for the ports' implementations, `system/` for the seams -- `traits/`, `model/`, `real/`.
- The registry (`cargo_ripe4rust::adapters::registry`): the sparse index, every release with its yanked flag,
  `rust_version` and `pubtime`'s day, and the API, a direct dependency's repository, at most a request a second --
  both through `curl`, as `docs/ADRs/ADR-TheRegistryThroughCurl.md` decides, with a user agent naming the tool,
  its version and its repository. Every answer is kept in a day cache, `ripe4rust/<date>/` under
  `$XDG_CACHE_HOME`, `~/.cache` or `%LOCALAPPDATA%`; `--offline` reads the latest day on or before the plan's.
  Two codes for what it meets: `curl-missing`, and `registry-cache-unusable` where the cache cannot be read or
  written.
- The workspace's files (`cargo_ripe4rust::adapters::workspace`), found where `cargo metadata --no-deps` says
  they are, asked once behind a `LayoutSource` seam: `[workspace.metadata.ripe4rust]` as the raw tree with every key's `file:line`, through `toml`'s
  span-keeping parser; a crate's requirement lines in every dependency table, every target's and
  `[workspace.dependencies]`; a member's `rust-version` line, its own or the workspace's it inherits;
  `rust-toolchain.toml`'s channel; and the fingerprint of the lock, the manifests and the toolchain file, FNV-1a
  by hand. A file that cannot be read or parsed is `workspace-unreadable`, at the line where it breaks.
- The clock, the toolchain and the scan, for now (`cargo_ripe4rust::adapters::{clock, toolchain}`): today in UTC
  from the system's clock through the core's new `Date::from_unix_days`, or the day `--date` names; `rustc
  --version` and `--print target-list` in the workspace's directory, `toolchain-unreachable` where they fail, and
  no releases until slice 5; and a source scan that knows nothing until slice 4.
- `plan`, end to end (`cargo_ripe4rust::command`): `cargo ripe4rust [plan] [--format table|json] [--date
  YYYY-MM-DD] [--offline]`, every port's adapter set up over the seams `main` passes in -- `main` a thin shim over
  `Ripe4rust`, the library's entry point -- and the one document on stdout: exit 0 where a plan was produced, 2 where
  none could be. `--date` stands for today, `--offline` keeps every adapter off the network. A command or an
  option it does not know is `usage-invalid`; `check`, `hold` and `guide`, until slice 3, `command-unavailable`,
  retired with slice 3.
  Tested through the entry point on a workspace recorded whole -- cargo, cargo-deny, rustc and the registry --
  against the JSON document and the table, an offline plan from its cache, and the plan's failures; the binary's
  own tests now run a command line that needs no network.
- `.gitattributes`: the recorded fixtures kept byte for byte on every system, so a Windows checkout reads what
  Linux recorded.
- The plan records every line a unit's `apply` can edit, with what it says (`workspace.lines`): each crates.io
  requirement in every manifest, each member's `rust-version` -- its own or the one it inherits -- and
  `rust-toolchain.toml`'s `channel`, read by the workspace's files adapter beside the requirement lines it already
  found. The fingerprint tells `check` that the workspace changed; the lines tell it where, so a requirement edited
  by hand is not taken for one a unit wrote.
- `check`, slice 3's first command: `cargo ripe4rust check <unit> --plan <file> [--format table|json]`, a unit's id
  or one move of its split, `<unit>/<crate>`. The core (`ripe4rust_core::planning::round`) reads the plan back
  through a new port, `PlanSource`, the policy again, and the workspace through `WorkspaceFiles`, now also answering
  `Cargo.lock`'s crates.io packages; then sorts every difference into the units that explain it. A lock move no unit
  makes is the unit's -- not as planned, exit 1, every difference with its `do` --; a requirement, `rust-version` or
  channel no unit's edit explains is the plan stale, exit 3, `plan-stale` where it is; as planned is 0, with the
  units applied and those still to go; a breaking unit applied reports its tail as `transitive`, and a tail move
  past a pin is a difference. The cli reads the plan document back -- schema 1, `plan`'s, holding units -- and
  `Cargo.lock` itself rather than through cargo, so a lock a hand-edited manifest no longer matches still reads.
  Three codes: `plan-unreadable`, `plan-schema-unknown`, `unit-unknown`. The command line reads each command's own
  options, and the renderers print `check`'s document in JSON and text.
- `hold`: `cargo ripe4rust hold <unit> --plan <file> --because <text> [--review YYYY-MM-DD] [--format table|json]`
  prints the pin that holds a unit and where it goes; the agent adds it. The core
  (`ripe4rust_core::planning::round::holding`) reads the plan, refuses an urgent unit as `unit-urgent` -- a new
  code, for the person: it becomes an urgent ask --, sorts the workspace as `check` does, any change no unit
  explains being the plan stale (3), and writes a line of `[workspace.metadata.ripe4rust.pins]` for every crate the
  unit's pins hold: a compatible unit's or a split move's whole, `hold = "all"`; a breaking unit's on its line; the
  toolchain's as `rust`. The reason is a TOML basic string, escaped; the review is `--review` or `hold-days` after
  the plan's date; a crate pinned already keeps its reason, its condition and the sooner review. The line goes under
  the pins table's line, or in that table added at the end of `Cargo.toml`. A test reads a printed pin back through
  the policy's own reader.
- `guide`: `cargo ripe4rust guide [--format table|json]` prints the round, every command with what it takes and the
  codes it exits with, every diagnostic code with its severity, whether it needs a person, what it means and its
  remedy, and every field of the documents of schema 1, versioned with the binary. It is generated in the core
  (`ripe4rust_core::planning::round::guiding`) from the catalogue the diagnostics come from: `DiagnosticCode` gains
  `ALL` and `entry`, a `CodeEntry` of what it means and its remedy, a test holds `ALL` to the enum as its source
  declares it, and another holds every key each document prints to a field of the guide. The round gains what to do
  when `check` exits 2.
- ripe4rust plans itself: the workspace's `Cargo.toml` declares its gates, `just stage1` and `just stage2`, in
  `[workspace.metadata.ripe4rust]`, so its own plan no longer warns `gates-undeclared`.
- The workspace's layout carries each member's sources, read from the same `cargo metadata --no-deps`: every target
  with its kinds and its root file, and every dependency its manifest declares, with its `package` rename and its
  source, which the parse of slice 4 reads. Naming a file from the workspace's root is `RootNames`, shared.
- The version choice moved to `ripe4rust_core::planning::units::choice`, so `planning/` keeps five folders with
  `round/` beside them.
- The scaffolding, slice 0 of `docs/DESIGN.md`'s "Delivery": every later slice is held to the family's gates from
  its first commit. `just stage1` -- the lock file synced, `cargo fmt`, clippy pedantic with `-D warnings` on every
  target, the tests -- and `just stage2`, run by a third workspace member, `xtask`, with tests of its own: the
  technology-free core, stern4rust, cargo-shear, dry4rust (floor 25, an empty baseline), crap4rust (threshold 15),
  twin4rust, iceberg4rust (ceiling 7) and cargo-deny, in that order. The compiler pinned at 1.99.1 in
  `rust-toolchain.toml`; `stern4rust.toml` naming `docs/header.txt`; `deny.toml`, the family's licences and sources.
- CI: `.github/workflows/ci.yml` runs both stages on Linux, Windows and macOS, every gate tool installed fresh.
- Every commit message carries core's grip and braintax: `.githooks/commit-msg` runs `cargo xtask grip-braintax`,
  enabled once a clone by `just hooks`.
- Tests for what 0.0.1 holds: `Outcome::label`'s three labels.
- `CLAUDE.md`: what ripe4rust is -- the design settled, the implementation in slices --, its boundary, its layout,
  the gates and the score hook, the rules every slice keeps, and the owner's strict rules and coding standards.
- `CODE_OF_CONDUCT.md` and `DONATE.md`, with the two QR images, as the family carries them.
- This changelog.

### Changed
- `ripe4rust-core`'s root carries `#![forbid(unsafe_code)]` beside `#![no_std]`, as the technology-free gate asks;
  the workspace's lints add clippy's `all`, denied, and `pedantic`, warned.
- `docs/DESIGN.md`'s "Delivery" puts slice 1's table and JSON renderers in the cli, where "Architecture" has them.
- `xtask` states its own `rust-version`, 1.88, which its let-chain needs, instead of inheriting the workspace's 1.85.
- `CLAUDE.md`, the README and `docs/DESIGN.md`'s status say slice 1 is in place, and where its code lives.

### Fixed
- A compatible unit whose own move a pin holds (`hold = "all"`) no longer reads as a move to the version in use,
  `memchr 2.5.0 -> 2.5.0`: its `to` is null in the JSON, `held` in the table, and `evidence.held` names the move;
  its summary counts the crates it pulls in, and its question asks to move the crate, naming no version.
- A diagnostic, a `check` difference or a guide entry whose text ends in a full stop -- a cargo error quoted at its
  end, often -- no longer prints `.. Do:`: the full stop closes it only where it has no ending of its own.
- Planning a workspace whose `Cargo.lock` no longer matches its manifests, or that has none, is no longer
  `cargo-metadata-failed` quoting cargo's advice to drop `--locked`: it is a code of its own, `lock-outdated`, in the
  catalogue `guide` lists, whose remedy says to bring the lock in step with `cargo update --workspace`, commit it,
  and plan again.

## [0.0.1] — 2026-10-30

### Added
- `ripe4rust-core` and `cargo-ripe4rust` on crates.io, at the design stage: the core holds the design's vocabulary,
  `Outcome` and `UnitKind`, and nothing that decides; the command says so and exits 2.
