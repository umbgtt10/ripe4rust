# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- `xtask` takes its gates from serge4rust 0.3.2, the house's gates as a library, as beggar's, cartel's and
  faction's do, and keeps `src/main.rs` alone: which gates each stage runs, in what order, and this workspace's
  constants. Its own copy of the drill -- the runner, the gates, the stage, the failures, crap4rust's report and the
  commit scoring, 42 files of source and their 146 tests -- is gone; the technology-free core, the cargo-shear and
  the lock-sync gates went into serge4rust with their tests, and the rest were serge4rust's already. Both stages and
  the commit's score block say what they said, but for the counts of what is no longer here: stern scans 694 files,
  not 776, crap4rust measures 721 functions, not 833, and the xtask runs no test of its own.
- `xtask` inherits the workspace's `rust-version`, 1.85, again: the let-chain that needed 1.88 went with the
  boundary gate, and serge4rust builds on 1.85.
- The compiler the gates pass with, pinned in `rust-toolchain.toml`, moves from 1.99.1 to 1.100.1, the unit
  ripe4rust's own plan called ripe. `rust-version`, the minimum a user's toolchain needs, stays 1.85.

## [0.1.4] — 2026-11-28

What the first pins written in a round found: a pin with its review date ahead held nothing once its crate fell
far behind.

### Changed
- The rule against drift no longer asks a unit a pin holds where every pin holding it carries a review date: until
  that date it is ripening, however far behind its crate falls, and on that date rule 5 asks it, as before. Two pins
  a round wrote from the owner's "wait" and "later", each with its review date, were asked again on the next plan
  as overripe, keep-pinned or unpin -- a decision already made, recorded with its date, asked on every plan until
  that date. A pin released by a condition alone has no date that asks it again, and is still asked overripe; so is
  a young line (`ADR-AReviewDateHoldsAgainstDrift`).

## [0.1.3] — 2026-11-28

What the rounds after 0.1.2 found, applying plans in three workspaces: a plan made later the same day read the
morning's cached answers and missed a release published since, and `check` said "as planned" of units that
re-pointed dependencies between versions the lock already held, naming none of it.

### Added
- `cargo ripe4rust plan --refresh`: online, every answer the plan reads -- the crates' index files, the API's answers
  and the Rust distribution's list of channel manifests -- is fetched again and the day's cached copy overwritten.
  The day cache served a plan made later the same day the morning's answers, so it never saw a release published
  since: the first applied rounds missed a crate's new release and Rust 1.100.1, and saw them only from a cache
  directory of their own. `--refresh` with `--offline` is `usage-invalid` -- one reads the cache alone, the other
  none of it --, and `guide` names the option and says in the round when to take it
  (`cargo_ripe4rust::adapters::registry::lookup::Lookup`, which `DayCache` and the request take in place of
  `offline: bool`).
- `check` tells every dependency re-pointed between versions the lock already held. `cargo update -p proptest`
  took the `windows-sys` dependency of `errno`, `rustix` and `tempfile` from 0.61.2 to 0.52.0, both locked, no
  version added or removed, and the next unit took them back; `check` said "as planned" both times and named none of
  it. The plan now records the lock's versioned edges in the new `workspace.edges`, after `git` -- every dependency
  `Cargo.lock` writes with a version, of every package, members included, on a crates.io package, `errno 0.3.14 ->
  windows-sys 0.61.2`; a bare edge, to a crate the lock holds once, cannot re-point and is left out. `check` reports
  each edge whose dependent is in both locks at the same version and whose dependency moved between two versions both
  hold, in the new `repointed` after `shared`, "`errno` 0.3.14's `windows-sys` 0.61.2 -> 0.52.0, both already in the
  lock", in the table as `repointed: ...`; an edge a version's move explains is not reported. A re-point is
  information, never a difference: `as_planned`, the exit code and staleness are unchanged. It is cumulative against
  the plan's lock, and the round says to name in the unit's commit message those its check lists that the check
  before did not, and those it no longer lists; the gates judge them. A plan written by 0.1.2 or earlier, with no
  `edges`, still reads and checks as before, with the new notice `plan-without-edges`, once; `guide` lists it with
  its remedy, and names both fields (`ADR-ARepointIsToldNeverJudged`, `ripe4rust_core::inputs::lock_edge::LockEdge`,
  `ripe4rust_core::planning::round::edge_changes::EdgeChanges`, the core's `WorkspaceFiles::edges`, read by the cli's
  `LockReader::edges`).

### Changed
- The source scan's records -- `Scan`, `CrateUse`, `UnfollowedInclude` and `UnparsedFile` -- moved from
  `ripe4rust_core::inputs` into `ripe4rust_core::inputs::source_scan`, beside `inputs::git`: `inputs/` had reached
  the 20 files stern4rust lets a directory hold, and the lock's edges are one more.
- `cargo-ripe4rust` requires `ripe4rust-core` at exactly its own release, `=0.1.3`, where it took any `0.1` from
  its own on: the two are released together, and the core's API moves within `0.1` -- this release adds a method to
  `WorkspaceFiles` and moves the scan's records --, so a later core would break a release of the binary installed
  without `--locked`. An earlier binary installed that way takes this core and does not build: install it with
  `--locked`, or install this one.

### Fixed
- A cache entry was written to `<name>.partial` beside it, then renamed: two rounds writing one answer at once --
  two plans refreshing the same day, say -- shared that file, so one could rename the other's half-written answer
  into place for a third to read, and fail its own rename. Each write now has a temporary file of its own, named for
  the process and the write, and one whose rename fails is removed.

## [0.1.2] — 2026-11-27

What the first round that applied a plan found, in one real workspace: units sharing a crate made `check` say "not
as planned" with nothing wrong, and said so again at every later check, and a breaking unit a dependent's requirement
held back read ripe -- or urgent, credited with an advisory its move could not clear.

### Fixed
- Units sharing a crate made `check` say "not as planned" with nothing wrong. Each unit's moves are computed alone,
  on the lock the plan found, and applied one after another they move the lock under the units after them: a crate
  one unit took to a version the whole workspace removes was removed by the next, a crate a unit removed alone was
  kept, and the rest's crates only added or removed came only with units a round had reverted -- each a ripe unit
  turned into an ask. The plan now names the crates the units share, in the new `shared` after `gates`: those whose
  moves, each unit's computed alone, do not add up to the whole workspace's -- a unit leaving a version the whole
  workspace keeps or reaching one it does not, two units taking one version to different places, or the whole
  workspace moving a version no unit does --, a crate pinned with `hold = "all"` never one of them. `check` holds a
  unit to the moves it owns -- every move of a crate the units do not share, and of a shared one its own crate's,
  its split's and the rest's `--precise` steps --, and reports a shared crate's move it does not own, or a change of
  one no move explains, in the check document's new `shared`, never as a difference -- of the crates the unit moves
  alone, so a later check does not repeat an earlier unit's shared crates; its exit codes are unchanged,
  and it still fails a move of a crate no unit shares, its own crate not moved, a split or the rest's step not
  taken. `hold` counts such a change as no change outside the plan, and a unit is listed applied by the moves it
  owns. The table writes the shared crates on a line after it, and `guide` names both fields. A plan written before
  reads as sharing none (`ADR-AUnitIsCheckedOnWhatItOwns`,
  `ripe4rust_core::planning::units::shared_crates::SharedCrates`,
  `ripe4rust_core::planning::round::ownership::Ownership`).
- A later unit's move, and the whole workspace's, was left out of it where an earlier unit's move left the same
  version, wherever each took it: the whole workspace's removal of a crate a unit had moved went unnamed, and a later
  unit taking a version further lost its move. A move is now left out where the moves taken before it leave the
  version it leaves and reach the one it reaches, so a crate the lock holds in two versions is told apart however
  cargo prints it -- the rest no longer reads two of `rand`'s versions moving as two crates added.
- A rest of crates only added or removed applied nothing -- no `--precise` step takes them -- and its check could
  only fail. It is no unit now: those crates come with the units that need them, and are shared.
- A breaking unit whose crate another package of the tree requires on the line in use was judged as if it could
  land alone: `breaking:prost`, 0.13.5 to 0.14.6, read ripe by its footprint while `tonic` 0.12.3 requires `prost`
  `^0.13`, so the edit only added 0.14.6 beside 0.13.5. Every package of the lock -- the unit's own members aside --
  that depends on the version in use with a requirement the target does not meet now holds the crate back: it is
  named among the unit's reasons, "tonic 0.12.3 requires prost ^0.13: moving it adds 0.14.6 beside 0.13.5", and in
  its evidence as the new `held_by`, and a unit the rules would take or ask ripens instead, its rule kept, with the
  new suggestion `until-its-dependents-move` -- the dependent's own upgrade releases it. An urgent unit keeps its
  ask, and one already ripening what holds it; `guide` names the new fields, and says the rules are 1 to 13
  (`ADR-ARequirementHoldsACrateBack`,
  `ripe4rust_core::planning::judging::held_by_dependents::HeldByDependents`).
- Such a held-back move was credited with the advisories against the version it leaves, which stays in the lock for
  the dependent: the breaking unit read urgent, and no row said the advisory would remain. A move a dependent holds
  back now clears nothing. The advisory keeps its row, urgent, "replace or accept", which holds the move that would
  clear it in its evidence's `held`, names the dependents among its reasons and in `held_by`, and says in its
  question that the upgrade that would clear it ripens until its dependents move; the breaking unit, urgent no more,
  ripens with the rest (`ripe4rust_core::planning::units::clearing::Clearing` takes the dependents).

## [0.1.1] — 2026-11-26

What the first dry round found, with 0.1.0 across 32 workspaces: a git source moving stopped the plan, a
crate that stays read as moving to itself, `include!` went unread, a move back went unsaid, a missing lock was taken
for an outdated one, and the small ones.

### Fixed
- `cargo ripe4rust --version`, and `-V`, print `cargo-ripe4rust <version>` and exit 0, where they were refused as
  `usage-invalid`; `guide` lists it among the commands.
- The table's unit column counts a group's other members and the crates a compatible unit pulls in within one
  parenthesis, `tonic (+1 released with it, +16 transitive)`, where it wrote `tonic (+1) (+16 transitive)`; the JSON
  is unchanged.
- A workspace with no `Cargo.lock` -- a library that leaves it out -- is the new error `lock-missing`, which needs a
  person: whether to commit a lock is the owner's call, and with their word `cargo generate-lockfile` writes it, to
  commit before planning again. It was `lock-outdated`, which needs none and told the agent to write and commit one;
  `lock-outdated` is now only a lock that exists and no longer matches the manifests. `guide` lists the new code.
- A compatible unit whose own crate does not move, only the crates it pulls in, read `serde 1.0.229 -> 1.0.229`
  and asked "Move tokio to 1.48.0?" of a crate already there. Its `to` is now null, the table writes `stays`, and
  its question asks "Move the N crates <crate> pulls in?", as its summary already said; one whose own move a pin
  holds still writes `held`. Nothing else about the unit changes: it groups, and its evidence names the
  `rust-version`, by the version in use (`ripe4rust_core::planning::units::own_move::OwnMove`); `guide`'s
  description of `units[].to` says when it is null.
- A move to a lower version inside a unit was silent: a compatible unit of over a hundred moves took one crate back
  from 0.1.7 to 0.1.6, a release not yanked, and read as ripe. Every move back is now named among the unit's reasons,
  "moves <crate> back from <from> to <to>", and listed in its evidence as `downgrades`; one leaving a release that
  is not yanked makes a unit the rules call ripe ask, rule and urgency kept, with the new suggestion `moves-back`.
  One leaving a yanked release is the cure, named "back from the yanked ...", its outcome unchanged
  (`ADR-AMoveBackIsAsked`).
- What no unit clears is one advisory row per crate version, `advisory:<crate>@<version>`, listing every advisory
  against that version and its yanking, as a compatible unit lists all it clears; it was one row per advisory,
  `advisory:<id>`, so a crate with four advisories took four rows. `check` and `hold` refuse the new ids as they did
  the old, and `guide` names the form.
- The parse follows an `include!` of a literal path -- the generated code `prost-build` writes, pulled into a
  module with `include!("generated/x.rs")` -- to the file it names, from the including file's directory as the
  compiler does, its items in the module the `include!` is in; what that code names counts in the footprint, and in
  the exposure where the module is public. A breaking unit whose crate only generated code named read "footprint 0
  lines in 0 files". An included file not there is unparsed at the `include!`'s line.
- An `include!` the parse cannot follow -- its path no literal, `concat!(env!("OUT_DIR"), ...)`, or in an
  expression -- and a macro that includes generated code, `include_proto!`, no longer leave a footprint that reads
  as exact: each is named where it is, and the footprint of every crate the member holding it uses is a floor --
  never small under rule 11, the verdict "wait", the reason "footprint at least N lines in M files: code is included
  the parse cannot follow", or "named nowhere the parse could read" for none -- and the unit's evidence lists them as
  `unfollowed_includes`. The scan carries them (`ripe4rust_core::inputs::unfollowed_include::UnfollowedInclude`).
- A breaking unit whose crate a complete parse finds named nowhere read "footprint 0 lines in 0 files", and rule 11
  called it ripe. It is now possibly an unused dependency: rule 11 leaves it, its verdict is "migrate now", its
  reasons say "named nowhere in the sources: possibly an unused dependency" whatever rule decided it, and its
  question offers the new answer `remove` beside its others -- a person decides, as a crate can be needed for its
  features or side effects (`ripe4rust_core::planning::judging::named_nowhere::NamedNowhere`).
- A plan with a cold day cache fetched the sparse index one `curl` process per crate, one after the other. The index
  files the day lacks are now one batch, which the command runner runs at most eight at once, each on a thread of
  its own, answering in the batch's order, so the plan and the cache are the same as before: for a workspace of 461
  locked crates, 163 s became 71 s. The core's `Index` port is asked once for every crate, and the cli's
  `CommandRunner` and `Http` seams take a batch (`run_all`, `get_all`); the API keeps its one request a second.
- A git dependency moving stopped the plan: `cargo update --dry-run` prints a git source's move as `Updating
  <crate> v<version> (<source>#<rev>) -> #<rev>`, which the strict reader answered `cargo-update-unrecognised`, and a
  workspace depending on a git branch got no plan. The reader now reads git lines strictly as what they are --
  moved, moved back, added or removed, the source and the short revs, and no version for a moved crate, which cargo
  does not print -- and a git source moving is a unit of the new kind `git`, `git:<crate>`: every crate of the source
  at once, from rev to rev, with the registry's moves `cargo update --dry-run -p <crate>@<version>` says it pulls in,
  claimed in plan order. Its `apply` is one `cargo update -p <crate>`; it never splits. It is always asked, by the new
  rule 13 -- a git source makes no version promise --, urgent by rule 3 where it clears an advisory or a yanked
  release, and held by a pin on any crate of its source. `check` reads the lock's git sources by their revs, `hold`
  pins the unit's first crate with `hold = "all"`, and a pin on a crate of a git source is no absent crate. The plan
  document gains `workspace.git` and a unit's `source`, a git unit's `from` and `to` are its revs, and `guide` names
  them; older plans still read (`ADR-AGitSourceIsAsked`).

## [0.1.0] — 2026-11-26

The first release that decides: `plan`, `check`, `hold` and `guide`, every slice of `docs/DESIGN.md`'s
"Delivery" in place.

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
- What a plan does not know, said rather than guessed: the source scan's port answers `None` where the scan failed
  as a whole, and the toolchain's releases `None` while the channel manifests were not read. A footprint not known
  is never small -- rule 11 calls nothing ripe, the verdict waits -- and a breaking unit a published member uses
  asks under rule 8, its exposure not known; with no releases no toolchain unit is offered. The plan says so, in the
  warning `source-unscanned` and the notice `toolchain-releases-unknown`, retired with slice 5, which reads the
  releases.
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
  no releases until slice 5, which reads them; and a source scan that knows nothing until slice 4.
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
- The source scan's answer says more (`ripe4rust_core::inputs`): an `UnparsedFile` names where to look -- the line
  its parse stopped at, its first line, or the `mod` item declaring it --, and `source-unparsed` stands there; and a
  `Scan` names the members whose public API it could not read in full, a file of their library among those it could
  not read. A breaking unit a published one of them uses asks under rule 8, its exposure there not known, as for a
  scan that failed as a whole. The reasons and the question say "not known" where they said "not yet known".
- What a file's names stand for (`cargo_ripe4rust::adapters::workspace::sources::naming`), the first part of slice
  4's parse, read with `syn` 2 and `proc-macro2`'s span locations: a member's dependencies by the names its code
  calls them -- a `package = ...` rename, hyphens as underscores --; every name a file's `use` and `extern crate`
  items bind, through renames, groups and one another, and the dependencies it glob-imports; and `NameFinder`, every
  line naming a dependency -- a path rooted at it, a `use` item and every name it binds, a macro by path, by
  `#[macro_use]` or through a glob, the tokens a macro or an attribute is given, a derive, an attribute the crate
  owns. A name nothing declares that a glob may have brought -- starting a longer path, or in capitals -- is put down
  to the file's globs; the standard library's names, built-in attributes, derives and macros never are. A
  dependency's own name alone in code is a variable's, and an item of the file's own shadows it.
- A target's modules walked (`cargo_ripe4rust::adapters::workspace::sources::walking`), the parse's second part: from
  the target's root through every `mod` item it and its modules declare, at the top or in inline modules -- `name.rs`
  then `name/mod.rs`, beside a file owning its directory or under its own name, a `#[path]` resolved beside the
  declaring file --, each file read once and parsed with `syn`, and marked reachable where every module on the way is
  `pub`. A file that does not parse is unparsed at the line its parse stopped at, one that cannot be read at its first
  line, one a `mod` item names but is not there at that item.
- A library's public API read (`cargo_ripe4rust::adapters::workspace::sources::exposure`), the parse's third part:
  the dependencies its users see, by package -- in a `pub` item of a module reachable from its root through `pub mod`
  declarations, a function's signature, a struct's or a union's public fields, every variant of an enum, a trait's
  bounds and items, a type alias, a constant's or a static's type, a derive, a `pub use` or `pub extern crate` --,
  and wherever in the library they are written, a trait's implementation where neither the trait nor the type is
  the library's own and private, and a public type's `pub` methods and constants. Bodies, private fields and
  private items are not seen.
- The parse, slice 4 of `docs/DESIGN.md`'s "Delivery": `SourceScanner`
  (`cargo_ripe4rust::adapters::workspace::sources`) replaces the scan that knew nothing. Every member's targets as
  `cargo metadata --no-deps` lists them -- library, binaries, tests, benches, examples, build script -- are walked
  with the modules they declare, and give every crates.io dependency named its footprint, every `file:line` naming
  it, and the members whose public API it reaches, the library's alone. A file the parse cannot read is
  `source-unparsed` where it broke, and a library holding one has its API unread; where not one file of all it
  tried could be parsed, the scan failed as a whole and knows nothing. With footprint and exposure known, rules 8
  and 11, the verdict and a migration's work list read them: on the recorded workspace, `itoa` is asked as in
  `app-core`'s public API, its three locations the work list. `guide`'s entry for the footprint says what the parse
  cannot see. ripe4rust's own policy names its two crates on crates.io as `published`.
- The version choice moved to `ripe4rust_core::planning::units::choice`, so `planning/` keeps five folders with
  `round/` beside them.
- Exposure follows `pub use` re-exports (`cargo_ripe4rust::adapters::workspace::sources::exposure::re_export_reach`):
  the facade, `mod inner; pub use inner::Thing;`, puts `Thing` in the library's public API wherever it is defined.
  A reachable `pub use` reaches what it names, told by name as the library's types are -- `Thing` and `Thing as
  Other` every item named `Thing`, a `use` binding that name so a re-export re-exported is followed --, and a glob,
  or a module it names, every item of that module and of the `pub mod`s in it, followed until nothing more is
  reached. The walk names each file's module as its `mod` item does, `#[path]` or not. `docs/DESIGN.md` drops the
  limit, and it and `ADR-OneParseForFootprintAndExposure` count what `pub mod` declarations or `pub use` re-exports
  reach.
- The toolchain's stable releases, read (`cargo_ripe4rust::adapters::toolchain`), the first part of slice 5:
  `ManifestList` reads the list of every channel manifest the Rust distribution published,
  `https://static.rust-lang.org/manifests.txt`, a stable release a line with the day it was published -- dated by
  its last listing, a release listed earlier too having been promoted ahead of its day --, passing over the
  nightlies, the betas, a line's alias and `stable`. `ChannelManifests`, behind a new seam, `ReleaseSource`, fetches
  it through `curl` and keeps it in the day cache, `<date>/toolchain/manifests.txt`, once it reads. Any status but
  200, curl failing, or nothing cached `--offline`, is `toolchain-unreachable`; a list naming no stable release is
  not kept. Tested on the list as the server answered it on 2026-11-26.
- The toolchain unit, end to end, slice 5 of `docs/DESIGN.md`'s "Delivery" and the last: the toolchain port answers
  the stable releases, or why it could not (`ripe4rust_core::traits::toolchain`), read only where the channel pins
  a release; `RustcToolchain` takes the release source through its constructor, and `plan` reads the
  distribution's list through the run's `curl` and day cache. A channel pinning an older stable release is a
  toolchain unit to the newest, judged as the design says, its evidence naming the versions it would release; on
  the recorded workspace, 1.99.1 to 1.100.0, asked as it reaches the firmware target, and pinned at 1.62.1 it
  releases `itertools 0.15.0` and `itoa 1.0.19`. The releases unreachable with nothing cached stop the plan,
  `toolchain-unreachable`, as the registry's do. `toolchain-releases-unknown` names no case any more and is gone
  from the catalogue, `guide` and the design; its tests ask for the failure that takes its place, and
  `toolchain-unpinned`'s remedy. `ADR-TheToolchainIsAUnit` names the list, and why not a manifest. Every slice is in
  place; next is the first dry round across real workspaces.
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
- `.cargo/mutants.toml`, which onboards ripe4rust on sentinel4rust: `ripe4rust-core` is mutated, against its own
  tests only; the cli and the xtask are not.

### Changed
- `source-unscanned` is a warning, and means a scan that failed as a whole -- not one source file could be parsed --,
  its remedy to run `cargo check` and report syntax the parser does not read; no plan carries it any more for sources
  not yet parsed.
- `ripe4rust-core`'s root carries `#![forbid(unsafe_code)]` beside `#![no_std]`, as the technology-free gate asks;
  the workspace's lints add clippy's `all`, denied, and `pedantic`, warned.
- `docs/DESIGN.md`'s "Delivery" puts slice 1's table and JSON renderers in the cli, where "Architecture" has them.
- `xtask` states its own `rust-version`, 1.88, which its let-chain needs, instead of inheriting the workspace's 1.85.
- `CLAUDE.md`, the README and `docs/DESIGN.md`'s status say slice 1 is in place, and where its code lives.
- `curl-missing` says curl reads the toolchain's releases as well as the registry.
- ripe4rust's own `published` names `ripe4rust-core` alone: `cargo-ripe4rust` is installed as a subcommand, and its
  library is there for its own tests, so its plan no longer asks `breaking:syn` under rule 8. `docs/DESIGN.md`'s
  policy says so: `published` names the crates others depend on as libraries.

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
