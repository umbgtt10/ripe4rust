# ripe4rust -- design

Status: settled 2026-11-26. Nothing here is implemented. The decisions that
were not obvious each have an ADR under `docs/ADRs/`.

## The question

Is this upgrade ripe?

Never upgrading is wrong: dependencies drift, maintained lines stop, fixes
arrive only in versions nobody moved to. Upgrading everything the moment it is
published is not right either: a new major can be a migration, a young line can
still be settling, a change can reach code that needs more than the test suite
to validate. Between the two sits a judgement that is made over and over, the
same way, from the same facts.

ripe4rust makes that judgement explicit. For every upgrade a workspace could
take, it answers one of three things:

| outcome | meaning | what the caller does |
|---|---|---|
| **ripe** (AUTO) | safe to take without asking | apply it, run the gates, commit if green, revert if red |
| **ask** (ASK) | worth taking, but a person decides | present it with a suggestion and the evidence |
| **ripening** (HOLD) | not yet | list it, with its reason and what will release it |

Any outcome can be **urgent**: an advisory or a yanked release the unit would
clear. An urgent unit is taken, or asked, ahead of the rest.

It decides; it never applies. The plan it writes is the input to whoever does
the upgrading -- an agent, a script, a person.

## What it is not

- Not an updater. It edits no manifest, moves no lock file, makes no commit.
  It asks cargo what a move would do, with a `cargo update` that only reports
  and writes nothing.
- Not a gate. A workspace being behind is information, not a defect in the
  change being gated, so no plan fails a stage, urgent or not
  (ADR-StalenessNeverFailsAGate).
- Not a vulnerability scanner. It reads advisories, through cargo-deny, as one
  input among several; cargo-deny stays the gate on them.

## Vocabulary

- **Unit** -- the smallest upgrade worth one commit. Three kinds:
  - a **breaking** unit: one direct dependency moved to a new major (for
    `0.x`, a new minor), with whatever the lock file moves along with it;
  - a **compatible** unit: the semver-compatible lock-file moves a direct
    dependency pulls in, grouped by that dependency. One commit per transitive
    crate would be hundreds; one for everything would not bisect;
  - the **toolchain** unit: `rust-toolchain.toml`'s channel moved to a newer
    stable release (ADR-TheToolchainIsAUnit).
- **Line** -- every release of one major (`2.x`), or one minor for `0.x`
  (`0.13.x`). For the toolchain, one minor (`1.100.x`).
- **Group** -- crates released together, moved as one unit
  (ADR-CratesReleasedTogetherMoveTogether).
- **Pin** -- a dependency held back on purpose, with its reason and what
  releases it.
- **Policy** -- the workspace's facts and thresholds the rules read.
- **Plan** -- the output: one row per unit, with its outcome and the reasons.

## Scope

- **In:** dependencies from the crates.io registry, normal, build and dev
  alike, and the toolchain.
- **Out:** path and git dependencies. Their moves are the workspace's own
  business (a lock sync, a submodule), and the registry has nothing to say
  about them.
- **Not yet:** the cargo subcommands a workspace's gates install. They are
  upgrades of the same shape, and moving them can turn a gate red with no code
  changed, but they live outside the workspace's manifest. An open point.

## Inputs

| signal | from |
|---|---|
| available versions, yanked flags, `rust-version` | the crates.io sparse index, for every crate in the lock |
| release dates | the crates.io API, for direct dependencies only |
| what a compatible unit moves | `cargo update --recursive -p <crate>`, reporting only |
| the dependency graph, which targets reach a crate | `cargo metadata` |
| advisories against the versions in use | `cargo deny check advisories --format json` |
| the toolchain in use | `rust-toolchain.toml` |
| the toolchain's releases and their dates | the Rust distribution's stable channel manifests |
| footprint and public exposure | the workspace's sources, parsed with `syn` |
| pins and policy | `[workspace.metadata.ripe4rust]` in `Cargo.toml` |
| today's date | injected, so a plan is reproducible |

The registry is read politely: the sparse index for what every crate needs,
the API only where a date is needed, at most one request a second, with a user
agent that names the tool, its version and its repository and nothing
personal. Every answer is cached for the day it was read; a plan names that
date, and with the same cache and the same date it is the same plan
(ADR-TheIndexForVersionsTheApiForDates).

## Which version a unit moves to

A version is never a candidate if it is yanked, a pre-release, or needs a
newer Rust than the toolchain in use. The toolchain is a unit of its own, so
"needs a newer Rust" is not a reason to hold an upgrade: it is a reason the
version is not offered yet, recorded as evidence on the toolchain unit
(ADR-AnUnbuildableVersionIsNoCandidate).

- **Compatible:** whatever cargo's resolver picks. Under resolver 3 it already
  holds back a version whose `rust-version` the workspace does not meet.
- **Breaking:** the newest release of the newest line that has settled. A
  younger line beyond it is in the evidence. If every newer line is young, the
  unit moves to the newest, and rule 9 holds it.
- **Toolchain:** the newest stable release.

A version within the toolchain but needing more than the workspace's declared
`rust-version` is a candidate. Its unit raises `rust-version` too, and for a
published crate that is rule 8's question.

## What each unit moves

- **Compatible:** what `cargo update --recursive -p <crate>` reports, asked
  to write nothing (ADR-CargoComputesTheLockMoves). Units are taken in plan order, by crate
  name. A transitive move several units share belongs to the first, because
  applying it moves it for the rest. The moves a reporting `cargo update` makes
  for the whole workspace and no unit reaches form one last unit, **the
  rest**.
- **Breaking:** the direct crate and its target version. What the lock moves
  with it is known only once the manifest is edited, which ripe4rust never
  does; the caller sees it when applying.
- **Toolchain:** `rust-toolchain.toml`'s channel, nothing else. `rust-version`
  moves only with a crate that needs it.
- **Group:** the members' moves together.

An advisory or a yanked release in use is attached to the unit whose moves
clear it, wherever it sits in the tree -- an advisory on a transitive crate
lands on the unit of the direct dependency that pulls it. One that no unit
clears gets a row of its own: **ask**, urgent, "replace the crate, or accept it
in `deny.toml`'s `[advisories] ignore` with its reason".

## The rules

Applied in order to every unit; the first that matches decides.

| # | when | outcome | suggestion |
|---|---|---|---|
| 1 | the unit clears an advisory or a yanked release, and reaches a target the policy marks as needing more than the gates | **ask**, urgent | apply, then validate there -- even past a pin or a freeze |
| 2 | the unit clears an advisory or a yanked release, and is compatible | **ripe**, urgent | -- even past a pin or a freeze |
| 3 | the unit clears an advisory or a yanked release, and is breaking | **ask**, urgent | migrate now -- even past a pin or a freeze |
| 4 | pinned, release condition not met, review date not reached | **ripening** | -- |
| 5 | pinned, release condition met or review date reached | **ask** | revisit the pin |
| 6 | a freeze window is open | **ripening** | until the window closes |
| 7 | the unit reaches a target the policy marks as needing more than the gates (firmware, say) | **ask** | apply, then validate there |
| 8 | breaking, and the crate's types are in a published crate's public API, or the unit raises a published crate's `rust-version` | **ask** | the upgrade is a breaking change of ours too |
| 9 | breaking or toolchain, and the new line is young | **ripening** | until it settles |
| 10 | compatible or toolchain | **ripe** | -- (a red gate turns it into ask) |
| 11 | breaking, footprint under `small-footprint` | **ripe** | -- (a red gate turns it into ask) |
| 12 | breaking | **ask** | migrate now / plan it / wait, by the verdict below |

Rules 1 to 3: an upgrade that removes an advisory or a yanked release and stays
inside the semver range is the safest upgrade there is, and the most pressing,
so it is ripe; it is asked only where a person must decide anyway
(ADR-AFixInRangeIsRipe). The toolchain unit reaches every target, so rule 7
asks it in every workspace that names one.

### The verdict for a breaking unit that needs asking

- **Migrate now**: the line in use has had no release for `stale-months`
  while the new one is active, or migrating removes a second copy of the crate
  from the tree.
- **Plan it**: the new line has settled and the footprint is moderate.
- **Wait**: anything else, with the reason.

### Against drift

Rule 4 must not become "never". A pin carries a review date or a release
condition, and two escalations override a unit that rule 4 or rule 9 would
hold:

- the workspace is two majors or more behind;
- the line in use has had no release for `stale-months` while a newer line is
  active.

Either turns the unit into **ask** with the suggestion "overripe". A freeze
still holds: it ends on its own date. Rules 1 to 3 already override
everything.

## Footprint and exposure

One `syn` parse of the workspace's sources answers both
(ADR-OneParseForFootprintAndExposure).

- **Footprint** -- the files and lines that name the crate: paths, `use`
  items, macros, derives and attributes, through renames and globs.
- **Exposure** -- whether the crate's items reach a published crate's public
  API: public signatures, public fields, `pub use` re-exports, and the crate's
  traits derived or implemented on public types.

A parse can be wrong about exposure (a type alias, a macro that expands to a
public item). The policy can override it, crate by crate.

## Groups

Some crates only move together: a framework and its macros, a family released
in lockstep. They form one unit when

- they share a repository and released the same version on the same day, or
- one requires an exact version (`=x.y.z`) of the other.

A group is judged as one unit; its footprint and exposure are the members'
together, and its youth is its youngest member's.

## Pins and policy

Both live in the workspace's `Cargo.toml`, where `cargo metadata` hands them
over and every other tool in the family keeps its configuration:

```toml
[workspace.metadata.ripe4rust]
# Crates published to crates.io: a breaking dependency in their public API
# makes the upgrade a breaking change of theirs (rule 8).
published = ["mytool-core"]
# Targets whose code the gates alone cannot validate (rules 1 and 7).
validate-beyond-gates = ["thumbv7em-none-eabihf"]
# A freeze: everything not urgent ripening until the date (rule 6).
freeze-until = "2027-01-15"
# Where the parse is wrong about a crate's exposure (rule 8).
exposure = { serde = "public", anyhow = "private" }
# Thresholds, with these defaults.
young-days = 30
young-releases = 3
toolchain-young-days = 0   # the family takes a stable release on its day
stale-months = 6
small-footprint = 10       # lines naming the crate

[workspace.metadata.ripe4rust.pins]
# Every pin has a reason, and a review date or a release condition, or both.
syn = { reason = "the parser every measure is written against; 3 is a migration of the visitors", review = "2027-01-31" }
probe-rs = { reason = "the rig's CLI is 0.31; the library moves with it", until = { crate = "probe-rs-tools", version = ">=0.32" } }
tokio = { reason = "the runtime the service ships on; a release is load-tested first", until = "the load test passes on the new release", review = "2027-02-28" }
```

A pin without a reason, or without a review date or a release condition, is
an error, reported before any rule runs. A release condition is either
checkable -- a crate reaching a version, read from the index -- or in words.
One in words is honest but cannot be checked, so it must come with a review
date; a pin with neither is refused (ADR-APinConditionCanBeChecked). A pin
named `rust` holds the toolchain unit.

## The plan

A table for a person, JSON for a caller -- one document either way:

```
date 2026-10-30, registry read 2026-10-30
unit                  kind        from      to        outcome          reasons
anyhow (+1)           compatible  1.0.102   1.0.104   ripe, urgent     clears RUSTSEC-2026-0190
clap (+3 transitive)  compatible  4.6.6     4.6.7     ripe             compatible
rust                  toolchain   1.99.1    1.100.0   ripe             toolchain; releases 2 versions held back
smoltcp               breaking    0.13.1    0.14.0    ask              reaches thumbv7em-none-eabihf; plan it: footprint 42 lines in 7 files
syn                   breaking    2.0.119   3.0.9     ripening         pinned: the parser ...; review 2027-01-31
the rest              compatible  --        --        ripe             2 moves no unit reaches
```

```json
{
  "schema": 1,
  "date": "2026-10-30",
  "registry_read": "2026-10-30",
  "units": [
    {
      "crate": "syn",
      "kind": "breaking",
      "from": "2.0.119",
      "to": "3.0.9",
      "moves": ["syn"],
      "outcome": "ripening",
      "urgent": false,
      "suggestion": null,
      "rule": 4,
      "evidence": {
        "line_in_use": { "last_release": "2026-07-15", "releases": 90 },
        "new_line": { "first_release": "2026-07-18", "releases": 10 },
        "rust_version": "1.71",
        "footprint": { "files": 11, "lines": 20 },
        "public": false,
        "duplicate_in_tree": true,
        "advisories": [],
        "yanked_in_use": false
      }
    }
  ]
}
```

The rule that decided and the evidence it read are always included: a caller
asking a person must be able to show why. `schema` moves when the document's
shape does.

## How a caller uses it

The plan is the input to an upgrade round, done the same way every time:

1. Run `cargo ripe4rust --format json`.
2. Take the urgent units first, then the rest, in plan order.
3. For each **ripe** unit, one at a time: apply it (`cargo update --recursive
   -p <crate>` for a compatible unit, the channel for the toolchain, the
   manifest for a breaking one), run every mandatory gate, commit if green --
   one commit per unit, so each is revertible -- and if red, revert and treat
   the unit as **ask**, with the failure attached. A compatible unit that fails
   is bisected: split, find the crate that broke it, hold that one, commit the
   rest.
4. Gather every **ask** unit into one question, each with its suggestion and
   the evidence that led to it.
5. Report every **ripening** unit, with what will release it.

The applying stays in the workspace, which knows its own gates. ripe4rust
only decides, so its answer for a given set of inputs is the same every time,
and it can be tested like the rest of the family.

## Architecture

The family's shape: a technology-free `ripe4rust-core` and a `cargo-ripe4rust`
CLI.

- **core** -- the units, the version choice, the rules, the verdicts, the plan.
  `no_std`-friendly and free of I/O: every input arrives through a trait -- the
  index, the release dates, the lock moves, the workspace's metadata, the
  advisories, the toolchain's releases, the source scan, the clock -- so every
  rule is tested against fakes, the date included.
- **cli** -- the real implementations: the sparse index and the API with their
  day cache, the reporting `cargo update`, `cargo metadata`, `cargo deny`, the
  channel manifests, the `syn` parse, the system clock, and the table and JSON
  renderers.

Exit codes: `0` when a plan was produced, whatever it says; `2` when it could
not be (a pin without a reason, an unreadable manifest, the registry
unreachable with nothing cached for the day).

## Delivery

In slices, each releasable and each held to the family's stages 1 and 2:

1. **The core.** Units, version choice, rules, verdicts and the plan, against
   fakes; the table and JSON renderers.
2. **The adapters.** `cargo metadata`, the reporting `cargo update`, the index and
   the API with the cache, `cargo deny`, the clock.
3. **The parse.** Footprint and exposure, and the policy's override.
4. **The toolchain unit.** The channel manifests, `rust-toolchain.toml`, the
   held-back versions as evidence.

Then a first dry round across real workspaces: plans only, nothing applied,
to tune the thresholds and the grouping before any caller acts on a plan.

## Open points

1. **The gates' own tools.** The cargo subcommands a workspace's gates
   install move like dependencies, and a new one can turn a gate red with no
   code changed (iceberg4rust 0.5.0 did, in three workspaces, on 2026-11-25).
   They are named in a workspace's docs or CI, not its manifest. A fourth kind
   of unit, read from a list the policy keeps?
2. **The thresholds.** `young-days`, `young-releases`, `toolchain-young-days`,
   `stale-months` and `small-footprint` are guesses, and so is the grouping's
   "same day". The first dry round tunes them.
