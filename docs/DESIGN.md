# ripe4rust -- design

Status: settled 2026-11-26; the agent's contract added the same day. Every
slice of "Delivery" is implemented: the scaffolding, the core with the
renderers, the adapters with `plan` end to end, the round's commands, `check`,
`hold` and `guide`, the parse of the sources, and the toolchain unit, released
as 0.1.0. Next is the first dry round across real workspaces. The decisions that were not
obvious each have an ADR under `docs/ADRs/`.

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
  `toolchain:rust`; `rest` for the last compatible unit;
  `advisory:<crate>@<version>` for a crate version with advisories or a
  yanking no unit clears, every one of them in that one row; such a row's kind
  is `advisory`. A group goes by its
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
| the toolchain's releases and their dates | the Rust distribution's list of the channel manifests it published, `manifests.txt` |
| footprint and public exposure | the workspace's sources, parsed with `syn` |
| pins, policy and the gates | `[workspace.metadata.ripe4rust]` in `Cargo.toml` |
| today's date | injected, so a plan is reproducible |

The registry is read politely: the sparse index for everything it carries,
the API only for a direct dependency's repository, at most one request a
second, with a user agent that names the tool, its version and its repository
and nothing personal. Every answer is cached for the day it was read, under
`~/.cache/ripe4rust/<date>/`; a plan names that date, and with the same cache
and the same date it is the same plan (ADR-TheIndexForVersionsAndDates). The
requests go through a `curl` process, not an HTTP crate
(ADR-TheRegistryThroughCurl). The toolchain's releases are read the same way,
one request a day, and kept in the same cache.

cargo-deny is required. Without it, or when it fails, no plan is produced: a
plan blind to advisories would call ripe what it should call urgent.

`cargo update --dry-run` reports in text, not JSON. ripe4rust reads it
strictly: a line it does not recognise stops the plan with a diagnostic,
rather than leave a move out of it (ADR-CargoComputesTheLockMoves).

### How the cli reads them

Every adapter is a process or a file behind a seam of the cli's own -- a
`CommandRunner`, a `FileSystem`, a `Pause` -- so each is tested on output
recorded from the pinned toolchain, and none of the tests reaches the network.
Where the sections above are silent, the adapters read the inputs this way:

- **`cargo metadata`** runs with `--format-version 1 --locked`, so it never
  writes the lock file, and with `--offline` in an offline plan. A member's
  dependencies and a locked package's are the crates.io packages its resolve
  node names, each with the requirement that asked for it: the first declared
  for that crate, from crates.io, that the resolved version meets, or `*` where
  none does. A crate reaches a `validate-beyond-gates` target where `cargo
  metadata --filter-platform <target>` resolves it at all, through any member
  and any kind of dependency: the conservative reading, so a crate only a host
  tool uses still asks. A lock `--locked` would have to write is refused with
  cargo's refusal quoted, without its advice to drop `--locked`, since
  ripe4rust writes no lock. One it would have to create, the workspace having
  none, is `lock-missing`, which needs a person: a library may leave its lock
  out on purpose, so whether to commit one is the owner's call, and with their
  word `cargo generate-lockfile` writes it, to commit before planning again.
  One no longer in step with the manifests, which it would have to update, is
  `lock-outdated`: the remedy is to bring the lock in step with `cargo update
  --workspace`, which moves no locked version the manifests still allow,
  commit it and plan again. Any other failure is `cargo-metadata-failed`, with
  cargo's error on one line; output it cannot read is `internal`.
- **`cargo update --dry-run`** runs as `--recursive -p <crate>@<version>` for
  a direct dependency, the version naming the copy in use, and with
  `--offline` in an offline plan. Every line of what it prints must be a move
  -- `Updating` or `Downgrading <crate> v<from> -> v<to>`, `Adding <crate>
  v<to>`, `Removing <crate> v<from>`, with no note or one of cargo's own,
  `(available: v...)`, `(latest: v...)`, `(requires Rust ...)` -- or a line
  that moves nothing: the index or a git source updated, the `Locking` summary,
  a wait for cargo's file lock, an `Unchanged` crate, a `note:` or a
  `warning:`. Anything else is `cargo-update-unrecognised`, quoting the line;
  a failure is `cargo-update-failed`.
- **`cargo deny`** runs as `cargo deny --format json --locked --workspace
  check --audit-compatible-output advisories`, with `--offline` in an offline
  plan. The advisories in force are the advisory diagnostics of its log, on
  stderr -- `vulnerability`, `unmaintained`, `unsound`, `notice`, one for each
  crate and version it names -- which leaves out what `deny.toml` ignores;
  the ranges each is clear in, `patched` and `unaffected`, come from the audit
  report on stdout, which the log does not carry. An advisory the report does
  not hold is cleared by no version. It exits 1 when it found advisories, so
  0 and 1 are a check made; an error in its log, or any other end, is
  `cargo-deny-failed` with that error on one line, and cargo answering `no such
  command` is `cargo-deny-missing`. Yanked releases are read from the index,
  not from cargo-deny's own yanked check.
- **The registry** is read through `curl` (ADR-TheRegistryThroughCurl): a
  crate's index file at `https://index.crates.io/` under the path its name's
  length gives it, and the API at `https://crates.io/api/v1/crates/<crate>?include=`,
  which answers the crate without its versions. The API waits a second before
  every request after the first; a cached answer costs no request and no wait.
  The cache is `ripe4rust/` under `$XDG_CACHE_HOME`, or `~/.cache` where it is
  not set, or `%LOCALAPPDATA%` on Windows, one directory a day,
  `<date>/index/<crate>` and `<date>/api/<crate>`, each the body as it came; a
  404 is kept as an empty answer, so an offline plan reads what the online one
  did. The day is the plan's date, `--date` included: online, an answer the
  day lacks is fetched and kept under it; `--offline` reads the latest day on or
  before the plan's date, and an answer it lacks is `registry-unreachable`. A
  cache that cannot be read or written, or no directory to keep it in, is
  `registry-cache-unusable`.
- **The workspace's files** are found where `cargo metadata --format-version 1
  --no-deps` says they are -- the root and every member's manifest --, which
  resolves nothing, so it needs no network and moves no lock; each is named
  from the root with `/` on every system. They are parsed with `toml` (MIT OR
  Apache-2.0, its `parse` feature alone), which keeps every key's span: a
  policy key's line is the line its key is written on, and the section's own
  that of its `ripe4rust` key, in a table header or a dotted key alike. A bare
  date, a float and an integer beyond 64 bits arrive as text, for the policy's
  readers to refuse where text is not what the key takes; a `ripe4rust` that is
  no table is `workspace-unreadable`. A crate's requirement lines are found in
  every kind of dependency table, every target's, and `[workspace.dependencies]`:
  the line of `crate = "..."` or of a table's `version`, a renamed crate found
  by its `package`; one with no version of its own -- a path, a git source,
  `workspace = true` -- states none. The lines a plan records are these for
  every crate a manifest names, each with the requirement written there, the
  members' `rust-version` lines with theirs, and the channel's, in `file:line`
  order. A member's `rust-version` line is its own,
  or `[workspace.package]`'s where it inherits it. `rust-toolchain.toml` gives
  its channel and that line, an empty channel where it names none.
- **The fingerprint** is FNV-1a, 64 bits, written by hand rather than taken from
  a crate: over `Cargo.lock`, every manifest and `rust-toolchain.toml`, each by
  its name from the root, then its length and bytes, or a mark where it is
  absent, as sixteen hex digits. It tells `check` that the workspace changed;
  it seals nothing, so a cryptographic hash would buy no more.
- **The clock** is today in UTC, the system's time counted from the epoch with
  the core's date arithmetic. `--date` stands for today everywhere: in the
  rules, and as the day of the cache read and written.
- **The toolchain** is `rustc --version` and `rustc --print target-list`, run
  in the workspace's directory so `rust-toolchain.toml` chooses the compiler;
  a failure is `toolchain-unreachable`.
- **The toolchain's releases** are read only where the channel pins a stable
  release, from `https://static.rust-lang.org/manifests.txt`, through the same
  `curl` and day cache as the registry's answers, as
  `<date>/toolchain/manifests.txt`. The file lists every channel manifest the
  distribution published, one a line under the day it went out,
  `static.rust-lang.org/dist/<date>/channel-rust-<name>.toml`: a `<name>` that
  is a full stable version is a release of that day, and every other line -- a
  nightly, a beta or another pre-release, a line's alias such as `1.100`,
  `stable` itself -- is passed over. A release listed on several days is dated
  by the last: 1.49.0 is listed in October 2020 as well as on its day,
  2020-12-31, so an earlier listing is a promotion ahead of the release. About
  330 KB, it is the lightest answer that holds every release with its day: the
  stable channel's manifest names one release in about 900 KB; a ranged
  request for its head, which the server honours, gives its date but not the
  release, deep in the file; and a manifest per release is a request per
  release, found by trying names. Only a 200 is an answer -- a 404 is no empty
  one, as it is for the registry -- and the list is kept only once it reads,
  so a captive portal's page is never cached; one naming no stable release, or
  a stable line whose day is no date, is `internal`. Any other status, curl
  failing, or `--offline` with nothing cached, is `toolchain-unreachable`,
  which stops the plan as `registry-unreachable` does: the table makes it an
  error, and a plan blind to the toolchain's releases would offer no toolchain
  unit, and name no version held back for one, without a word. The cache being
  the registry's, an offline plan reads the list of the day it reads the
  registry's answers on, a cache that cannot be read or written is
  `registry-cache-unusable`, and curl not on the PATH `curl-missing`.
- **The sources** are parsed with `syn` 2 (MIT OR Apache-2.0, with `full` and
  `visit`), `proc-macro2`'s span locations giving every name its line: each
  member's targets as the same `cargo metadata --no-deps` lists them --
  library, binaries, tests, benches, examples, build script --, and the
  modules each declares, followed from its root as the compiler finds them:
  `name.rs`, then `name/mod.rs`, beside a file that owns its directory -- a
  root, a `mod.rs`, a file a `#[path]` names -- or under a directory of the
  file's own name otherwise, inline modules adding theirs; a `#[path]` beside
  the declaring file, `..` resolved. `cfg` is not evaluated: every module
  declared is read, once a target, and named from the root. A dependency is
  known by the name its code calls it, read from what `cargo metadata` says
  of the manifest: the name `package = "..."` renames it to, or the package's,
  hyphens as underscores. A crates.io one's name stands for its package; a
  path or git dependency's, and the member's own library's, are known but out
  of scope. Where not one file of all it tried could be parsed, the scan
  failed as a whole and answers that it knows nothing, which the plan reads as
  "Footprint and exposure" says.

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
unaffected; it clears a yanked release when it takes the crate off it. What
no unit clears gets a row of its own, one per crate version, which lists
every advisory against that version and its yanking, as a compatible unit
lists all it clears: **ask**, urgent, "replace the crate, or accept it in
`deny.toml`'s `[advisories] ignore` with its reason".

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

### A move back

cargo's resolver may take a crate to a lower version inside a unit: a newer
release of one crate can require an older one of another. Every such move is
named among the unit's reasons, "moves <crate> back from <from> to <to>", and
listed in its evidence as `downgrades` (ADR-AMoveBackIsAsked). One that
leaves a release that is not yanked turns a unit the rules call ripe into
**ask**, its rule and urgency kept, with the suggestion "moves back" -- after
the no-gates downgrade, so an ungated workspace is still asked for its gates
first. One that leaves a yanked release is the cure: named, "moves <crate>
back from the yanked <from> to <to>", its outcome unchanged. A unit that asks
or ripens for another reason keeps its outcome, the move back among its
reasons.

## Footprint and exposure

One `syn` parse of the workspace's sources answers both
(ADR-OneParseForFootprintAndExposure).

- **Footprint** -- the files and lines that name the crate: paths, `use`
  items, macros, derives and attributes, through renames and globs. Every
  location is kept, as `file:line`: it is the work list of a migration.
- **Exposure** -- whether the crate's items reach a published crate's public
  API: public signatures, public fields, `pub use` re-exports, and the crate's
  traits derived or implemented on public types, counted where the item is
  reachable from the crate's `lib.rs` through `pub mod` declarations or `pub
  use` re-exports.

A parse can be wrong about exposure (a type alias, a macro that expands to a
public item). The policy can override it, crate by crate: `public` exposes the
crate in every published member that uses it, `private` in none. A source file
the parse cannot read is a warning, `source-unparsed`, at the line its parse
stopped at -- its first line where it could not be read at all, the `mod` item
declaring it where it is not there: the footprint may be low. Where the file
belongs to a member's library, that member's public API is not read in full,
and the parse cannot say that no item reaches it through the file it could not
read: the exposure of every crate the member uses is not known there, and a
breaking unit of one the policy is silent on asks under rule 8, "exposure in
<member>'s public API not known", as for a scan that failed as a whole.

Where the scan fails as a whole -- not one source file of the workspace could
be parsed --, it answers that it knows neither, and the plan reads that
conservatively. A footprint not known is never small, so rule 11 calls no unit
ripe, and the verdict is "wait: footprint not known". A crate the policy is
silent on may be in the public API of any published member that uses it, so
its breaking unit asks under rule 8, "exposure in <member>'s public API not
known". The plan carries the warning `source-unscanned`, and an override in the
policy still decides.

Where the lines above leave it open, the parse reads a file this way:

- **What a name stands for.** A file's names resolve through its `use` and
  `extern crate` items -- renames, groups, `self`, `as _`, and one another,
  `use a::b; use b::C;` --, its own items, variants and generic parameters,
  the standard library's prelude and crates, and its member's dependencies. A
  binding or an item of the file's own shadows a dependency of the same name.
  A name declared anywhere in a file is taken as in scope throughout it: the
  inline modules of one file are not told apart.
- **What counts toward the footprint.** A path whose first name is a
  dependency's, starting a longer path -- `itoa::Buffer` -- or from the root,
  `::itoa`, or a name bound to one; a dependency's own name alone, in an
  expression or a type, is a variable's or a parameter's, and does not count.
  A `use` item counts at its first name's line and at each name it brings; an
  `extern crate` item at its own. A macro counts by its path, by a name bound
  to one, and -- called by its name alone, where neither the standard library
  nor the target defines one of that name -- for every crate the file
  glob-imports and the target's `#[macro_use]` crates. The tokens a macro or an
  attribute is given, which `syn` leaves unparsed, count where a dependency
  starts a path in them or a name bound to one appears. A derive counts by its
  path, by a name bound to the crate or through a glob; an attribute by its
  path, `#[tokio::main]`, or by the crate's own name, `#[serde(...)]`. A doc
  comment never counts.
- **Globs.** A name nothing declares that a glob may have brought -- starting a
  longer path, or written in capitals as a type, a variant or a constant is --
  counts for every crate the file glob-imports; the standard library's names
  and the built-in attributes, derives and macros never do. A function called
  by its name alone reads as a variable's, so one a glob alone brings in is not
  seen.
- **What exposes a crate.** A library's -- a `lib`, `rlib` or `dylib` target's
  -- `pub` items, `pub(crate)` and narrower aside, reachable from its root --
  in a module `pub mod` declarations reach, inline modules included, or
  through `pub use` re-exports, the facade `mod inner; pub use inner::Thing;`
  -- show their users: a function's signature, a struct's or a union's `pub` fields, every
  variant of an enum, a trait's bounds and its items' signatures, types and
  bounds, a type alias, a constant's or a static's type, their derives, a `pub
  use` and a `pub extern crate`; never a body or a private field. An
  implementation is public wherever in the library it is written: a trait's --
  its generics, the trait's path, the type, its associated types -- where
  neither the trait nor the type is the library's own and private, and a public
  type's own `pub` methods and constants. The library's types and traits are
  told by name, public where any definition of that name is. A proc-macro or C
  library exposes nothing.
- **What a re-export reaches.** A `pub use` reachable from the root reaches
  what it names wherever the library defines it, told by name as the
  library's types are: `inner::Thing` and `inner::Thing as Other` every item
  named `Thing`, and a `use` binding that name, so a re-export re-exported is
  followed to the item; `inner::*`, or a module a `pub use` names, every item
  of each module of that name -- a file's module named as its `mod` item names
  it, `#[path]` or not --, and of the `pub mod`s in it. What is reached is
  followed in turn, until nothing more is. A `use` short of `pub` re-exports
  nothing.
- **What it cannot see.** What a macro expands to -- generated items, a
  derive's expansion, `include!` --, as the ADR accepts; and a dependency whose
  library is named otherwise than its package, which the manifest does not
  say. The policy's `exposure` is the remedy for
  exposure; the footprint is a floor.

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

`published` names the crates others depend on as libraries; a subcommand's
library, kept for its own tests, is not one.

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
alike, so the agent always has one thing to read, and on stdout either way.
Each carries `schema`, which moves when the shape does, the command, and the
diagnostics.

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

- **severity** -- `error` stops the command (exit 2, or 3 for `plan-stale`,
  which `check` and `hold` exit with); `warning` and `notice` never change the
  exit code.
- **code** -- stable, one per kind of problem; `guide` lists every one with its
  remedy.
- **where** -- a `file:line` where there is one.
- **do** -- the next step, written for an agent with no context.
- **needs_person** -- true when only the workspace's owner can supply the fix:
  a pin's reason, a decision. The agent asks; it does not invent.

In text a diagnostic reads `error[<code>] <where>: <what>. Do: <do>`, and a
`check` difference `<where>: <what>. Do: <do>`. The full stop closes `what`
only where it has no ending of its own: a tool's error quoted at the end of it
often ends in a full stop, a question mark or an exclamation mark, and the text
never doubles one.

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
| `lock-outdated` | error | `Cargo.lock` no longer matches the manifests, and `cargo metadata --locked` will not update it |
| `lock-missing` | error | the workspace has no `Cargo.lock`, and `cargo metadata --locked` will not create it: whether to commit one is the owner's call |
| `workspace-unreadable` | error | a file of the workspace ripe4rust reads could not be read |
| `toolchain-unreachable` | error | the toolchain's releases, targets or version could not be read, nothing cached for the day |
| `curl-missing` | error | curl is not on the PATH, and the registry and the toolchain's releases are read through it |
| `registry-cache-unusable` | error | the day cache cannot be read or written, or there is no directory for it |
| `source-unscanned` | warning | not one source file of the workspace could be parsed, so the scan failed as a whole: no footprint or exposure is known |
| `usage-invalid` | error | a command or an option ripe4rust does not know, or an option's value of the wrong kind |
| `plan-unreadable` | error | the plan file `--plan` names is not there, or is no plan `plan` wrote |
| `plan-schema-unknown` | error | the plan file is of a `schema` this release does not read |
| `unit-unknown` | error | `check` or `hold` names an id that is no unit of the plan nor a move of a unit's split, or an advisory row, which applies nothing |
| `unit-urgent` | error | `hold` names an urgent unit, which is never held: it becomes an urgent ask |

Of these, the policy's own codes, `target-unknown`, `gates-undeclared`,
`lock-missing` and `unit-urgent` need a person: each asks for a fact or a
decision only the owner has.

### The plan

```
date 2026-10-30, registry read 2026-10-30, workspace 9f2c41d0
unit                          kind        from     to       outcome       reasons
anyhow (+1 released with it)  compatible  1.0.102  1.0.104  ripe, urgent  clears RUSTSEC-2026-0190
clap (+3 transitive)          compatible  4.6.6    4.6.7    ripe          compatible
rust                          toolchain   1.99.1   1.100.0  ripe          toolchain; releases 2 versions held back
smoltcp                       breaking    0.13.1   0.14.0   ask           reaches thumbv7em-none-eabihf; plan it: footprint 42 lines in 7 files
syn                           breaking    2.0.119  3.0.9    ripening      pinned: the parser ...; review 2027-01-31
the rest                      compatible  --       --       ripe          2 moves no unit reaches
```

```json
{
  "schema": 1,
  "command": "plan",
  "date": "2026-10-30",
  "registry_read": "2026-10-30",
  "workspace": {
    "fingerprint": "9f2c41d0...",
    "lock": ["anyhow 1.0.102", "clap 4.6.6", "..."],
    "lines": [
      { "where": "Cargo.toml:12", "key": "clap", "value": "4.6" },
      { "where": "rust-toolchain.toml:2", "key": "channel", "value": "1.99.1" }
    ]
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

Beyond the example, a unit carries its `members` -- a group's crates, `rust`
for the toolchain -- and its `reasons`, the table's last column; an `apply`
step is `{ "run": "..." }` or `{ "edit": "<file:line>", "key": "...",
"value": "..." }`; `evidence` also names the line in use with its first
release, the footprint's locations, the versions the toolchain unit would
release, the moves a pin holds out of the unit, and the moves to a lower
version it makes. The workspace's `lines` are
every line a unit's `apply` can edit, as the plan found it written: each
crates.io dependency's requirement in every manifest, `key` the crate; each
member's `rust-version`, its own or the one it inherits; and
`rust-toolchain.toml`'s `channel` -- what `check` compares the manifests and
the toolchain file with. Every object's keys come in the order shown, at
every depth: a unit's `members` after `crate` and its
`reasons` after `suggestion`; the evidence's `footprint`, `held_back`,
`held` and `downgrades` after `yanked_in_use`; the line in use as `line`, `first_release`,
`last_release`, `releases`. A document for a failure carries `schema`,
`command` and `diagnostics` alone.

In the table, one parenthesis after the crate counts a group's other members
as `+N released with it` and the crates a compatible unit pulls in as `+N
transitive`, `tonic (+1 released with it, +16 transitive)`; the workspace is named by its fingerprint's
first eight characters, and the diagnostics follow the table, one a line.

A compatible unit whose own move a pin holds -- its crate pinned with `hold =
"all"` -- takes that crate to no version: its `to` is null, the table writes
`held` in its place, and `evidence.held` names the move, so it never reads as a
move to the version in use. It still applies the moves it pulls in, one by one.
One whose crate does not move, only what it pulls in, takes it to no version
either: its `to` is null and the table writes `stays`, so it never reads as
`serde 1.0.229 -> 1.0.229`, and its question is "Move the N crates <crate>
pulls in?", as its summary says. Neither changes how the unit is judged,
grouped or applied: grouping and the evidence's `rust_version` read the
version in use as the one it lands on.

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

### What `check` prints

```json
{
  "schema": 1,
  "command": "check",
  "unit": "compatible:itoa",
  "as_planned": false,
  "transitive": [],
  "differences": [
    { "where": "Cargo.lock",
      "what": "`memchr` moved 2.5.0 -> 2.6.0, a move no unit of the plan makes",
      "do": "carry out the unit's `revert`, `git restore -- Cargo.lock`: compatible:itoa becomes ask, with these differences attached" }
  ],
  "applied": ["compatible:itoa"],
  "to_go": ["compatible:memchr"],
  "diagnostics": []
}
```

`transitive` is a breaking unit's tail, the moves its edit made beyond its
own, each as `moves` writes it; `where` is `Cargo.lock` for a move and a
manifest's `file:line` for an edit. A check that could not be made, or that
found the plan stale, prints the failure document -- `schema`, `command` and
`diagnostics` --, a stale plan's each a `plan-stale` at its `file:line`. In
text: `check <unit>: as planned` or `not as planned`, the tail as
`transitive: ...`, each difference as `<where>: <what>. Do: <do>`, the units
`applied:` and `to go:`, then the diagnostics.

## Commands

| command | does | exit codes |
|---|---|---|
| `cargo ripe4rust [plan]` | decides, and prints the plan | `0` a plan was produced, whatever it says; `2` none could be |
| `cargo ripe4rust check <unit> --plan <file>` | compares the workspace with what the unit should have done | `0` as planned; `1` not as planned; `2` could not check; `3` the plan is stale |
| `cargo ripe4rust hold <unit> --plan <file> --because <text> [--review <date>]` | prints the pin that holds the unit, and where it goes | `0` printed; `2` could not; `3` the plan is stale |
| `cargo ripe4rust guide` | prints the round, every diagnostic code and its remedy, and the document's fields | `0` |
| `cargo ripe4rust --version` | prints `cargo-ripe4rust <version>`, `-V` alike | `0` |

Every command but `--version`, which takes nothing and prints its one line,
takes `--format table|json`. `plan` also takes `--date
<YYYY-MM-DD>`, for a reproducible plan, and `--offline`, to read the cache
only -- and to keep cargo and cargo-deny off the network too, so an offline
plan reaches none. With no command, the command is `plan`; the name cargo
passes a subcommand first is skipped, so `cargo ripe4rust` and
`cargo-ripe4rust` read alike. An option takes its value after it or after `=`.
A command or an option ripe4rust does not know is `usage-invalid`, an error,
exit 2, the document in the format the line asked for; an unknown command's
`do` points at `guide`. Every command of the design runs, so the transitional
`command-unavailable` of slices 1 and 2 is gone. `--plan`
names the plan file from the directory the command runs in, or by an absolute
path; `check` and `hold` take one unit, its id or a split id. `main` is
a thin shim over the library's entry point, which takes every seam it reaches
the system through.

### check

`check` is what makes a step's result visible to the agent
(ADR-ACheckAfterEveryUnit). The plan holds the workspace it was made from: a
fingerprint of `Cargo.lock`, every manifest and `rust-toolchain.toml`, the
lock's packages, and every line a unit's `apply` can edit with what it said.
The fingerprint alone tells that something changed, not what; the lines are
what lets `check` tell a requirement a unit wrote from one edited by hand.
Against them, `check` sorts every difference in the
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

Where the lines above leave it open, the differences are sorted this way:

- **What is read, in order.** The plan file first: one that is not there, not
  JSON, not `plan`'s document, or holds no units is `plan-unreadable`, and a
  `schema` other than 1 is `plan-schema-unknown`. Then the unit: an id that is
  no unit of the plan nor an id of a unit's `steps.split` is `unit-unknown`, and
  so is an advisory row, which applies nothing. Then the policy as it stands,
  whose errors stop the command as they stop `plan` and whose warnings the
  document carries. Then the workspace: the fingerprint, and where it is the
  plan's nothing has changed; otherwise `Cargo.lock`'s crates.io packages --
  read from the file, not from cargo, so a lock a manifest edited by hand no
  longer matches still reads -- and the lines a unit can edit.
- **A move shows in the lock** where the version it leaves is gone and the one
  it takes is there, whatever else the lock holds of the crate: a move to a copy
  the lock already held shows as the old copy gone.
- **A line is the same line** where it is the same key, in the same file, in
  the same place among that key's lines there, so a line an edit above it moved
  down is still itself. A line rewritten to the value a unit's edit writes there
  is that unit's.
- **The lock and the lines are sorted apart**, because they change apart. The
  lock is what applying a unit moves, and cargo, not the plan, decides what it
  moves: a lock move no unit of the plan makes -- an extra move, another
  version, a crate gone -- is the step's doing, and the unit named is not as
  planned, **1**. The manifests and `rust-toolchain.toml` are written by hand,
  and a unit edits them only at the lines its `apply` names: a line no unit's
  edit explains -- rewritten, new or gone -- is a change outside the plan,
  **3**, one `plan-stale` for each. A change outside those lines -- the pin a
  `hold` printed, a comment -- is no difference.
- **A breaking unit's tail.** Where a breaking unit is applied, the lock moves
  no move of the plan accounts for are its tail: the unit named's `transitive`
  moves when it is that unit, and otherwise an earlier one's, so a compatible
  unit checked after a migration is not blamed for it. A tail move of a crate a
  pin holds -- off its line, or at all with `hold = "all"` -- is a difference.
- **Applied and still to go**, in plan order, advisory rows left out: a unit
  every move and edit of which shows is applied; one none of which shows is
  still to go; one partly applied is listed by the ids of its split where it
  has one, and as still to go where it has none.

### hold

`hold` is how a round remembers (ADR-AFailureBecomesAPin). A unit whose gates
went red, or one a person said to wait on, is held by a pin in the manifest,
and the next round finds it there. ripe4rust prints the pin -- the reason
given, a review date `hold-days` ahead unless one is given, `hold = "all"` for
a compatible move -- and the section and line it belongs at; the agent adds it
and commits it. ripe4rust still edits no manifest.

Where the lines above leave it open, `hold` works this way:

- **What is read, in order.** The plan and the unit, as `check` reads them; an
  urgent unit -- an advisory row is one -- is refused at once, `unit-urgent`, a
  decision for the person: it becomes an urgent ask. Then the policy as it
  stands and the workspace, sorted as `check` sorts it. No step is being
  checked, so a lock move no unit makes is a change outside the plan too,
  unless it is the tail of a breaking unit applied: any change no unit explains
  is `plan-stale`, **3**.
- **What the pins hold**, so the next plan holds the unit: a compatible unit's
  every crate it moves, with `hold = "all"` -- a pin on its crate alone would
  leave the rest of its moves to be judged again --, and a split move's crate
  the same; a breaking unit's crate on its line, the default, which holds a
  group by its first member; the toolchain as `rust`, on its line for a new
  minor and with `hold = "all"` for a patch release.
- **The pin** is one line of the pins table, `<crate> = { reason = "...",
  review = "YYYY-MM-DD" }`, `hold = "all"` added where it holds every move:
  the reason in a TOML basic string, quotes, backslashes and control characters
  escaped; the review the one `--review` gives, or `hold-days` after the plan's
  date, by the policy as it stands. A crate pinned already gets a line in place
  of its pin that keeps what that pin said: its reason first, then the new one,
  its `until`, the sooner review, and `hold = "all"` where either held every
  move.
- **Where it goes**: under `[workspace.metadata.ripe4rust.pins]`, at the line
  of that table's header or `pins` key; where the manifest has none, in that
  table added at the end of the workspace's `Cargo.toml`.

```json
{
  "schema": 1,
  "command": "hold",
  "unit": "compatible:memchr",
  "section": "[workspace.metadata.ripe4rust.pins]",
  "where": "Cargo.toml:41",
  "pins": [
    { "crate": "memchr",
      "line": "memchr = { reason = \"2.7.6 broke stage 2: ...\", review = \"2026-12-10\", hold = \"all\" }",
      "replaces": null }
  ],
  "do": "put the pin under `[workspace.metadata.ripe4rust.pins]` at Cargo.toml:41, and commit it",
  "diagnostics": []
}
```

`where` is `null` where the table is to be added, and `replaces` names the pin
a line takes the place of. In text: `hold <unit>: under <section> at
<file:line>`, or `in a new <section> at the end of Cargo.toml` followed by the
section's header, then each pin's line and `Do: ...`.

### guide

`guide` prints, for the release it is part of: the round below; every command
with its usage, what it does, what it takes and the codes it exits with; every
diagnostic code with its severity, whether it needs a person, what it means and
its remedy; and every field of the documents of schema 1, document by document
-- `every` for those all of them carry, then `plan`'s, `check`'s, `hold`'s and
`guide`'s, an array's items written `[]`. It reads nothing and exits 0.

The codes come from the catalogue the diagnostics come from: `DiagnosticCode`
gives each its label, severity and whether it needs a person, which
`diagnose` stamps on every diagnostic, and its entry -- what it means and its
remedy --, which only `guide` prints: a diagnostic's own `do` is that remedy
said of the case at hand. `DiagnosticCode::ALL` holds every code, in the order of its label; a
test reads the enum's declaration from its source and holds `ALL` to it, so a
code cannot be added without a guide entry, and another prints every
document's keys and holds each to a field of the guide.

```json
{
  "schema": 1,
  "command": "guide",
  "version": "0.1.0",
  "round": ["1. Run `cargo ripe4rust plan --format json > plan.json`.", "..."],
  "commands": [
    { "usage": "cargo ripe4rust check <unit> --plan <file> [--format table|json]",
      "does": "compares the workspace with what the unit should have done, ...",
      "options": [{ "option": "<unit>", "means": "..." }, "..."],
      "exits": [{ "code": 0, "means": "as planned: ..." }, "..."] }
  ],
  "codes": [
    { "code": "plan-stale", "severity": "error", "needs_person": false,
      "means": "the workspace changed outside the plan: a change no unit of it explains",
      "do": "plan again -- `cargo ripe4rust plan --format json > plan.json` -- and go on with the new plan" }
  ],
  "fields": [{ "document": "check", "field": "as_planned", "means": "..." }],
  "diagnostics": []
}
```

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
      - `2`: carry out each error's `do`, then run `check` again.
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
  distribution's list of channel manifests, the `syn` parse, the workspace's
  files and their fingerprint, the system clock, and the table and JSON
  renderers.

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

Every slice is in place. Next is a first dry round across real workspaces:
plans only, nothing applied, to tune the thresholds and the grouping before
any caller acts on a plan.

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
