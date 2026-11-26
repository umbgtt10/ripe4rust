# ripe4rust -- design

Status: settled 2026-11-26; the agent's contract added the same day. Nothing
here is implemented. The decisions that were not obvious each have an ADR
under `docs/ADRs/`.

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
| **ripe** (AUTO) | safe to take without asking | apply it, check it, run the gates, commit if green, revert and hold if red |
| **ask** (ASK) | worth taking, but a person decides | present the unit's question and carry out the answer |
| **ripening** (HOLD) | not yet | list it, with its reason and what will release it |

Any outcome can be **urgent**: an advisory or a yanked release the unit would
clear. An urgent unit is taken, or asked, ahead of the rest.

It decides; it never applies. The plan it writes is the input to whoever does
the upgrading.

## Who the caller is

The caller is an agent that knows nothing of the workspace, of ripe4rust or of
earlier rounds. Everything it needs comes from what ripe4rust prints
(ADR-TheAgentNeedsNoContext):

- **what to do**: every unit carries the exact steps that apply it, the gates
  to run and how to revert it;
- **what to expect**: every unit says what the workspace must show once
  applied, and `cargo ripe4rust check` compares the two;
- **what went wrong and what to do then**: every problem, fatal or not, is
  reported with where it is and what to do about it; nothing unexpected is
  silent;
- **how the procedure goes**: `cargo ripe4rust guide` prints the round, every
  problem's code and its remedy, versioned with the binary.

The agent stops for a person in one case only: a unit whose outcome is
**ask**, or a problem marked as needing a person -- a decision or a fact only
the workspace's owner has. Everything else it carries out on its own.

## What it is not

- Not an updater. It edits no manifest, moves no lock file, makes no commit.
  It asks cargo what a move would do with `cargo update --dry-run`, which
  writes nothing.
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
- **Unit id** -- `<kind>:<crate>`: `compatible:clap`, `breaking:syn`,
  `toolchain:rust`; `rest` for the last compatible unit; `advisory:<id>` for
  an advisory no unit clears, and `advisory:<crate>@<version>` for a yanked
  release none clears; such a row's kind is `advisory`. A group goes by its
  first member's name. A compatible unit's single moves, for bisecting it, are
  `<unit>/<crate>`.
- **Line** -- every release of one major (`2.x`), or one minor for `0.x`
  (`0.13.x`). For the toolchain, one minor (`1.100.x`).
- **Group** -- crates released together, moved as one unit
  (ADR-CratesReleasedTogetherMoveTogether).
- **Pin** -- a dependency held back on purpose, with its reason and what
  releases it.
- **Policy** -- the workspace's facts and thresholds the rules read.
- **Plan** -- the output: one row per unit, with its outcome, the reasons, the
  steps and the workspace it was made from.
- **Diagnostic** -- a problem found on the way, with where it is and what to
  do about it.

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
| available versions, yanked flags, `rust-version`, release dates | the crates.io sparse index (`pubtime`), for every crate in the lock and every crate a pin's condition names |
| a direct dependency's repository, for grouping | the crates.io API, for direct dependencies only |
| what a compatible unit moves | `cargo update --dry-run --recursive -p <crate>` |
| the dependency graph, which targets reach a crate | `cargo metadata` |
| advisories against the versions in use | `cargo deny check advisories --format json` |
| the toolchain in use | `rust-toolchain.toml`'s channel; `rustc --version` where it pins none |
| the toolchain's releases and their dates | the Rust distribution's stable channel manifests |
| footprint and public exposure | the workspace's sources, parsed with `syn` |
| pins, policy and the gates | `[workspace.metadata.ripe4rust]` in `Cargo.toml` |
| today's date | injected, so a plan is reproducible |

The registry is read politely: the sparse index for everything it carries,
the API only for a direct dependency's repository, at most one request a
second, with a user agent that names the tool, its version and its repository
and nothing personal. Every answer is cached for the day it was read, under
`~/.cache/ripe4rust/<date>/`; a plan names that date, and with the same cache
and the same date it is the same plan (ADR-TheIndexForVersionsAndDates).

cargo-deny is required. Without it, or when it fails, no plan is produced: a
plan blind to advisories would call ripe what it should call urgent.

`cargo update --dry-run` reports in text, not JSON. ripe4rust reads it
strictly: a line it does not recognise stops the plan with a diagnostic,
rather than leave a move out of it (ADR-CargoComputesTheLockMoves).

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
published crate that is rule 8's question. It raises the `rust-version` of
every member that uses the crate and declares a lower one; a member that
declares none promises nothing, and is not raised.

The toolchain's youth is judged by `toolchain-young-days` alone: a minor
rarely has three releases. Its evidence names, for each direct dependency, the
newest release the toolchain in use cannot build and the new one can.

A line is **young** while either holds: its first release is under
`young-days` old, or it has fewer than `young-releases` releases. It has
**settled** otherwise. A line is **active** while it has had a release within
`stale-months`, and **stale** otherwise. A line's releases are those neither
yanked nor pre-releases, whether or not the toolchain can build them. A
release the index gives no date for is older than any it does: it makes no
line young and no line active.

## What each unit moves

- **Compatible:** what `cargo update --dry-run --recursive -p <crate>`
  reports (ADR-CargoComputesTheLockMoves). Units are taken in plan order, by
  crate name. A transitive move several units share belongs to the first,
  because applying it moves it for the rest. The moves `cargo update
  --dry-run` makes for the whole workspace and no unit reaches form one last
  unit, **the rest**. A move of a crate pinned with `hold = "all"` is left
  out of every unit, which then applies its other moves one by one -- unless
  it clears an advisory or a yanked release, which no pin holds. A unit whose
  moves all belong to earlier units is no unit. A crate the members use at
  several versions is judged from the oldest.
- **Breaking:** the direct crate and its target version. What the lock moves
  with it is known only once the manifest is edited, which ripe4rust never
  does; `check` reports it after the agent has applied it.
- **Toolchain:** `rust-toolchain.toml`'s channel, nothing else. `rust-version`
  moves only with a crate that needs it.
- **Group:** the members' moves together.

An advisory or a yanked release in use is attached to the unit whose moves
clear it, wherever it sits in the tree -- an advisory on a transitive crate
lands on the unit of the direct dependency that pulls it. Where several units
clear it, it lands on the first in plan order: units by crate name, a crate's
compatible unit before its breaking one, the toolchain as `rust`, then the
rest, then the advisory rows. A move clears an advisory when it takes the
crate off the affected version to one the advisory names patched or
unaffected; it clears a yanked release when it takes the crate off it. One
that no unit clears gets a row of its own: **ask**, urgent, "replace the
crate, or accept it in `deny.toml`'s `[advisories] ignore` with its reason".

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
| 10 | compatible or toolchain | **ripe** | -- (a red gate turns it into a hold) |
| 11 | breaking, footprint under `small-footprint` | **ripe** | -- (a red gate turns it into ask) |
| 12 | breaking | **ask** | migrate now / plan it / wait, by the verdict below |

Rules 1 to 3: an upgrade that removes an advisory or a yanked release and stays
inside the semver range is the safest upgrade there is, and the most pressing,
so it is ripe; it is asked only where a person must decide anyway
(ADR-AFixInRangeIsRipe). The toolchain unit reaches every target, so rule 7
asks it in every workspace that names one.

A pin holds what its `hold` names (ADR-APinHoldsALine): by default the move to
a new line, so a crate pinned on its major still takes its patch releases;
with `hold = "all"`, every move. Rules 4 and 5 apply to a unit only where its
pin holds what the unit would move. Where several pins hold it, rule 4 needs
them all unreleased and holding all of it, and rule 5 takes the first that is
released.

With no gates declared, nothing can validate a unit, so every unit rules 2,
10 and 11 would call ripe is reported **ask**, "declare the gates", and the
plan carries the diagnostic that says so. The unit keeps the rule that called
it ripe, and its urgency. An advisory row is decided by no rule: its `rule` is
null.

### The verdict for a breaking unit that needs asking

- **Migrate now**: the line in use is stale while the new one is active, or
  migrating removes a second copy of the crate from the tree -- the lock
  already holds it on the new line, and nothing outside the workspace's
  members depends on the copy in use.
- **Plan it**: the new line has settled and the footprint is under
  `moderate-footprint`.
- **Wait**: anything else, with the reason.

A breaking unit that rule 7 or rule 8 asks carries its verdict too, among its
reasons and as its question's recommendation.

### Against drift

Rule 4 must not become "never". A pin carries a review date or a release
condition, and two escalations override a unit that rule 4 or rule 9 would
hold:

- the workspace is two majors or more behind;
- the line in use is stale while a newer line is active.

Either turns the unit into **ask** with the suggestion "overripe". A freeze
still holds: it ends on its own date. Rules 1 to 3 already override
everything. "Two majors behind" counts lines -- majors, or minors for `0.x`
and the toolchain -- among the candidates newer than the line in use, and "a
newer line" is any of them; the unit keeps the number of the rule whose hold
it overrides, 4 or 9.

## Footprint and exposure

One `syn` parse of the workspace's sources answers both
(ADR-OneParseForFootprintAndExposure).

- **Footprint** -- the files and lines that name the crate: paths, `use`
  items, macros, derives and attributes, through renames and globs. Every
  location is kept, as `file:line`: it is the work list of a migration.
- **Exposure** -- whether the crate's items reach a published crate's public
  API: public signatures, public fields, `pub use` re-exports, and the crate's
  traits derived or implemented on public types, counted where the item is
  reachable from the crate's `lib.rs` through `pub mod` declarations.

A parse can be wrong about exposure (a type alias, a macro that expands to a
public item). The policy can override it, crate by crate: `public` exposes the
crate in every published member that uses it, `private` in none. A source file
the parse cannot read is a warning: the footprint may be low.

## Groups

Some crates only move together: a framework and its macros, a family released
in lockstep. They form one unit when

- they share a repository and released the same version on the same day, or
- one requires an exact version (`=x.y.z`) of the other.

Only units of one kind group: the version is the one each moves to, and the
exact requirement is read from the lock.

A group is judged as one unit; its footprint and exposure are the members'
together, and its youth is its youngest member's.

## Pins and policy

Both live in the workspace's `Cargo.toml`, where `cargo metadata` hands them
over and every other tool in the family keeps its configuration:

```toml
[workspace.metadata.ripe4rust]
# The commands that validate a change, in order; each must exit 0. Without
# them nothing is ripe: the plan asks every unit.
gates = ["just stage1", "just stage2"]
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
small-footprint = 10       # lines naming the crate: ripe below
moderate-footprint = 100   # lines naming the crate: "plan it" below
hold-days = 14             # a hold's review date, from the day it is made

[workspace.metadata.ripe4rust.pins]
# Every pin has a reason, and a review date or a release condition, or both.
syn = { reason = "the parser every measure is written against; 3 is a migration of the visitors", review = "2027-01-31" }
probe-rs = { reason = "the rig's CLI is 0.31; the library moves with it", until = { crate = "probe-rs-tools", version = ">=0.32" } }
tokio = { reason = "the runtime the service ships on; a release is load-tested first", until = "the load test passes on the new release", review = "2027-02-28" }
memchr = { reason = "2.7.6 broke stage 2: <the gate's failure>", review = "2026-12-10", hold = "all" }
```

A pin without a reason, or without a review date or a release condition, is
an error, reported before any rule runs. A pin is released on its review date
or after it, or once the index holds a release, not yanked, that meets its
condition. A pin named `rust` on its line lets a patch release of the
toolchain through and holds a new minor. A freeze is open while the plan's
date is before `freeze-until`. A release condition is either
checkable -- a crate reaching a version, read from the index -- or in words.
One in words is honest but cannot be checked, so it must come with a review
date; a pin with neither is refused (ADR-APinConditionCanBeChecked). A pin
named `rust` holds the toolchain unit. A pin on a crate that is not in the
tree any more is a warning: remove it.

A key the policy does not know is an error, not ignored: a misspelt threshold
would otherwise apply its default without a word. A value of the wrong kind --
a threshold in words, a date that is no date, a `hold` other than `line` or
`all` -- is the error `policy-invalid-value`; every problem the policy has is
reported at once, before any rule runs. An empty `reason` is no reason, and
`gates = []` declares no gates.

## What a run prints

Every command prints one document: a table and text for a person, or, with
`--format json`, a JSON document for the agent -- on success and on failure
alike, so the agent always has one thing to read. Each carries `schema`,
which moves when the shape does, the command, and the diagnostics.

### Diagnostics

Every problem found on the way is a diagnostic (ADR-EveryProblemSaysWhatToDo):

```json
{
  "severity": "error",
  "code": "pin-without-reason",
  "where": "Cargo.toml:41",
  "what": "the pin on `syn` gives no reason",
  "do": "add `reason = \"...\"` to it: why the crate is held",
  "needs_person": true
}
```

- **severity** -- `error` stops the command (exit 2); `warning` and `notice`
  never change the exit code.
- **code** -- stable, one per kind of problem; `guide` lists every one with its
  remedy.
- **where** -- a `file:line` where there is one.
- **do** -- the next step, written for an agent with no context.
- **needs_person** -- true when only the workspace's owner can supply the fix:
  a pin's reason, a decision. The agent asks; it does not invent.

ripe4rust does not panic at its caller. A failure of its own is the
diagnostic `internal`, with what it was doing and where to report it.

The policy is read first, and its errors stop the command before any other
input is read; then the toolchain, whose unknown targets stop it too; then the
rest. An input that cannot be read stops the command at once, with every
diagnostic found up to it.

The problems the design already names, at least:

| code | severity | when |
|---|---|---|
| `policy-unknown-key` | error | a key in `[workspace.metadata.ripe4rust]` the tool does not know |
| `pin-without-reason` | error | a pin with no reason |
| `pin-without-release` | error | a pin with neither a review date nor a checkable condition |
| `pin-for-absent-crate` | warning | a pin naming a crate no longer in the tree |
| `target-unknown` | error | a `validate-beyond-gates` target the toolchain does not know |
| `gates-undeclared` | warning | no `gates`: every ripe unit reported as ask |
| `toolchain-unpinned` | notice | no channel pinned: no toolchain unit |
| `registry-unreachable` | error | the index or the API unreachable, nothing cached for the day |
| `registry-cache-old` | warning | `--offline` reading an earlier day's cache |
| `cargo-update-unrecognised` | error | a line of `cargo update --dry-run` the reader does not know |
| `cargo-deny-missing` | error | cargo-deny not installed |
| `cargo-deny-failed` | error | cargo-deny could not check |
| `source-unparsed` | warning | a source file the parse could not read |
| `plan-stale` | error | `check` or `hold` against a workspace changed outside the plan |
| `internal` | error | a failure of ripe4rust's own |
| `policy-invalid-value` | error | a key in `[workspace.metadata.ripe4rust]` whose value is not the kind it must be |
| `cargo-update-failed` | error | `cargo update --dry-run` could not run |
| `cargo-metadata-failed` | error | `cargo metadata` could not run |
| `workspace-unreadable` | error | a file of the workspace ripe4rust reads could not be read |
| `toolchain-unreachable` | error | the toolchain's releases, targets or version could not be read, nothing cached for the day |

Of these, the policy's own codes, `target-unknown` and `gates-undeclared` need
a person: each asks for a fact or a decision only the owner has.

### The plan

```
date 2026-10-30, registry read 2026-10-30, workspace 9f2c41d0
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
  "command": "plan",
  "date": "2026-10-30",
  "registry_read": "2026-10-30",
  "workspace": {
    "fingerprint": "9f2c41d0...",
    "lock": ["anyhow 1.0.102", "clap 4.6.6", "..."]
  },
  "gates": ["just stage1", "just stage2"],
  "diagnostics": [],
  "units": [
    {
      "id": "compatible:clap",
      "kind": "compatible",
      "crate": "clap",
      "from": "4.6.6",
      "to": "4.6.7",
      "moves": [
        { "crate": "clap", "from": "4.6.6", "to": "4.6.7" },
        { "crate": "clap_builder", "from": "4.6.6", "to": "4.6.7" }
      ],
      "outcome": "ripe",
      "urgent": false,
      "rule": 10,
      "suggestion": null,
      "summary": "Move clap to 4.6.7, with 3 crates it pulls in",
      "steps": {
        "apply": [{ "run": "cargo update --recursive -p clap" }],
        "check": "cargo ripe4rust check compatible:clap --plan plan.json",
        "gates": ["just stage1", "just stage2"],
        "revert": { "restore": ["Cargo.lock"], "run": "git restore -- Cargo.lock" },
        "split": [
          { "id": "compatible:clap/clap", "apply": [{ "run": "cargo update -p clap --precise 4.6.7" }] },
          { "id": "compatible:clap/clap_builder", "apply": [{ "run": "cargo update -p clap_builder --precise 4.6.7" }] }
        ]
      },
      "question": null,
      "evidence": {
        "line_in_use": { "last_release": "2026-10-02", "releases": 41 },
        "rust_version": "1.74",
        "advisories": [],
        "yanked_in_use": false
      }
    }
  ]
}
```

A breaking unit's `apply` names every manifest line that states the
dependency's requirement, as `file:line`, the requirement to write there, and
then the `cargo update -p <crate>` that takes it; its evidence carries the
footprint's locations, the migration's work list. The requirement written is
the target version itself, `3.0.9`, a caret requirement; a `rust-version` the
target needs raised is written the same way, at its own line. The toolchain
unit's `apply` is the channel to write in `rust-toolchain.toml`.

A compatible unit with a move held out of it, and the rest, whose moves are
taken one at a time so as not to take another unit's, apply one `cargo update
-p <crate> --precise <version>` per move; a compatible unit splits only where
it moves two crates or more. A crate the lock holds at several versions is
named `<crate>@<version>`. `revert` restores `Cargo.lock`, the manifests a
breaking unit edits too, and only `rust-toolchain.toml` for the toolchain.

The rule that decided and the evidence it read are always included: a caller
asking a person must be able to show why.

### The question of a unit that asks

A unit whose outcome is **ask** carries the question to put to the person,
ready to present, and what to do with each answer:

```json
"question": {
  "title": "Move smoltcp to 0.14.0?",
  "why": "it reaches thumbv7em-none-eabihf, which the gates cannot validate",
  "recommendation": "plan-it",
  "options": [
    { "answer": "migrate-now", "means": "apply it now and validate on the target",
      "then": "apply it as a ripe unit; the footprint's 42 locations are the work list" },
    { "answer": "plan-it", "means": "hold it until a planned migration",
      "then": "cargo ripe4rust hold breaking:smoltcp --plan plan.json --because \"<the person's words>\" --review <date>" },
    { "answer": "wait", "means": "not now",
      "then": "cargo ripe4rust hold breaking:smoltcp --plan plan.json --because \"<the person's words>\"" }
  ]
}
```

An urgent unit with no fix offers "replace the crate" and "accept it in
`deny.toml`"; a pin to revisit offers "unpin" and "keep it, with a new review
date". Every option's `then` is a step the agent can carry out without asking
again.

The answers, by the way a unit asks:

| the unit | options | recommended |
|---|---|---|
| an advisory row | `replace`, `accept` | `replace` |
| urgent, with no gates | `declare-gates`, `take-it` (`migrate-now` if breaking), `accept` | `declare-gates` |
| urgent | `take-it` (`migrate-now` if breaking), `accept` -- an urgent unit is never held | `take-it` |
| not urgent, with no gates | `declare-gates`, `take-it` (`migrate-now` if breaking), `wait` | `declare-gates` |
| a pin due (rule 5), or a pinned unit overripe | `unpin`, `keep-pinned` | `unpin` |
| breaking, overripe | `migrate-now`, `plan-it`, `wait` | `migrate-now` |
| breaking, otherwise | `migrate-now`, `plan-it`, `wait` | its verdict |
| anything else | `take-it`, `wait` | `take-it` |

## Commands

| command | does | exit codes |
|---|---|---|
| `cargo ripe4rust [plan]` | decides, and prints the plan | `0` a plan was produced, whatever it says; `2` none could be |
| `cargo ripe4rust check <unit> --plan <file>` | compares the workspace with what the unit should have done | `0` as planned; `1` not as planned; `2` could not check; `3` the plan is stale |
| `cargo ripe4rust hold <unit> --plan <file> --because <text> [--review <date>]` | prints the pin that holds the unit, and where it goes | `0` printed; `2` could not; `3` the plan is stale |
| `cargo ripe4rust guide` | prints the round, every diagnostic code and its remedy, and the document's fields | `0` |

Every command takes `--format table|json`. `plan` also takes `--date
<YYYY-MM-DD>`, for a reproducible plan, and `--offline`, to read the cache
only.

### check

`check` is what makes a step's result visible to the agent
(ADR-ACheckAfterEveryUnit). The plan holds the workspace it was made from: a
fingerprint of `Cargo.lock`, every manifest and `rust-toolchain.toml`, and the
lock's packages. Against them, `check` sorts every difference in the
workspace into the units it belongs to:

- every difference belongs to units of the plan, and the one named shows
  exactly its expected moves: **0**, with the units already applied and those
  still to go, so an interrupted round resumes where it stopped;
- the unit named shows nothing, or something else -- a move missing, another
  version, an extra move, a crate pinned on its line taken to a new one by a
  breaking unit: **1**, each difference listed, with what to do: revert, and
  the unit becomes **ask** with the differences attached;
- a difference no unit of the plan explains -- the workspace changed outside
  the plan: **3**, `plan-stale`: plan again, and go on with the new plan.

For a breaking unit, `check` reads the moves the edit made, which the plan
could not know, and reports them.

### hold

`hold` is how a round remembers (ADR-AFailureBecomesAPin). A unit whose gates
went red, or one a person said to wait on, is held by a pin in the manifest,
and the next round finds it there. ripe4rust prints the pin -- the reason
given, a review date `hold-days` ahead unless one is given, `hold = "all"` for
a compatible move -- and the section and line it belongs at; the agent adds it
and commits it. ripe4rust still edits no manifest.

## The round

What `guide` prints, and what the agent does, every time:

1. Run `cargo ripe4rust plan --format json > plan.json`.
   - Exit `2`: carry out each error's `do`; one that `needs_person` goes to
     the person. Then plan again.
2. Take the urgent units first, then the rest, in plan order.
3. For each **ripe** unit, one at a time:
   1. carry out `apply`;
   2. run `check`:
      - `0`: run `gates` in order. All green: commit, one commit per unit,
        `summary` as its subject in the workspace's own conventions. One red:
        carry out `revert`; a compatible unit of several moves is bisected
        through `split` (apply, check, gates each), its green moves committed;
        hold the one that failed -- or the unit, if it has no split -- with
        `hold --because "<the failing gate and its first error>"`, add the
        pin, commit it. A breaking unit that fails becomes **ask**, the
        failure attached; so does an urgent unit, still urgent -- it is never
        held, as holding it would keep what it clears.
      - `1`: carry out `revert`; the unit becomes **ask**, with `check`'s
        differences attached.
      - `3`: plan again, and go on with the new plan.
4. Report every **ripening** unit, with what will release it.
5. Put every **ask** unit's question to the person in one message. Carry out
   each answer's `then`.
6. The round is done when every unit is committed, held, answered or
   ripening. A last `plan` shows only those held, asked or ripening.

The applying stays with the agent, which knows the workspace's conventions for
commits and pushes. ripe4rust only decides and checks, so its answer for a
given set of inputs is the same every time, and it can be tested like the rest
of the family.

## Architecture

The family's shape: a technology-free `ripe4rust-core` and a `cargo-ripe4rust`
CLI.

- **core** -- the units, the version choice, the rules, the verdicts, the
  questions, the steps, the diagnostics, the plan; `check`'s sorting of
  differences, `hold`'s pin, `guide`'s text. `no_std`-friendly and free of
  I/O: every input arrives through a trait -- the index, the repositories, the
  lock moves, the workspace's metadata and files, the advisories, the
  toolchain's releases, the source scan, the clock -- so every rule and every
  diagnostic is tested against fakes, the date included.
- **cli** -- the real implementations: the sparse index and the API with their
  day cache, `cargo update --dry-run`, `cargo metadata`, `cargo deny`, the
  channel manifests, the `syn` parse, the workspace's files and their
  fingerprint, the system clock, and the table and JSON renderers.

## Delivery

In slices, each releasable and each held to the family's stages 1 and 2:

0. **The scaffolding.** `CLAUDE.md`, the `justfile`, the `xtask` gates, the
   score hook, CI.
1. **The core.** Units, version choice, rules, verdicts, questions, steps,
   diagnostics and the plan, against fakes; the table and JSON renderers, in
   the cli.
2. **The adapters, and `plan`.** `cargo metadata`, `cargo update --dry-run`,
   the index and the API with the cache, `cargo deny`, the clock, the
   fingerprint: `plan` end to end on recorded fixtures.
3. **The round's commands.** `check`, `hold` and `guide`.
4. **The parse.** Footprint and exposure, and the policy's override.
5. **The toolchain unit.** The channel manifests, `rust-toolchain.toml`, the
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
   `stale-months`, `small-footprint`, `moderate-footprint` and `hold-days` are
   guesses, and so is the grouping's "same day". The first dry round tunes
   them.
