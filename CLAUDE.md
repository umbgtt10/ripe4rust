# ripe4rust

## Meaning

`ripe4rust` answers one question: is this upgrade ripe? `cargo ripe4rust` will decide, for every upgrade a Rust
workspace could take -- a dependency to a new compatible version or a new major, the toolchain to a new stable
release -- one of three things: **ripe**, safe to take without asking; **ask**, worth taking but a person decides;
**ripening**, not yet, with its reason and what will release it. Its caller is an agent with no context, so every
unit carries the steps that apply it, the check that confirms it and the question to put when it asks.

It decides; it never applies. It edits no manifest, moves no lock file and makes no commit. It is not a gate either:
a workspace being behind is information, not a defect in the change being gated.

The design is settled: `docs/DESIGN.md` holds it, and `docs/ADRs/` the decisions that were not obvious. The
implementation came in slices, each held to stages 1 and 2 from its first commit, as `docs/DESIGN.md`'s "Delivery"
lists them: 0 the scaffolding, 1 the core, 2 the adapters and `plan`, 3 `check`, `hold` and `guide`, 4 the parse, 5
the toolchain unit. Every slice is in place: the core decides, against fakes of every port, and `cargo ripe4rust
plan` reads a real workspace through the adapters -- cargo, cargo-deny, rustc, the registry through `curl` with its
day cache, and the toolchain's releases from the Rust distribution's list of channel manifests through the same --
and prints the plan as a table or JSON; `cargo ripe4rust check <unit> --plan <file>` reads the plan back and the
workspace again, and sorts every difference into the units that explain it; `hold` prints the pin that holds a unit,
and where it goes; `guide` prints the round, every command, every diagnostic code with its remedy and the documents'
fields; the sources are parsed with `syn`, every member's targets and the modules they declare, for each breaking
unit's footprint -- every `file:line` naming its crate, a migration's work list -- and its exposure in a published
member's public API, `pub use` re-exports followed; and a channel pinning an older stable release is a toolchain
unit, its evidence the versions the newer one would release. The next step is the first dry round across real
workspaces: plans only, nothing applied, to tune the thresholds and the grouping. Version 0.1.0, on crates.io,
is all of it.

It is self-contained.

## Boundary Rule

This repository is **SELF-CONTAINED**.

The LLM **SHALL NOT cross its boundaries without asking**.

That means:
- do not inspect, edit, or rely on files outside `ripe4rust/` unless the user explicitly asks
- do not pull assumptions from sibling repositories or crates
- do not propose cross-repository changes by default

The repository is public: nothing private about the workspaces it is run against belongs in it.

## Layout

A workspace of three members, with a virtual root that holds no package:

| member | what it is |
|---|---|
| `core/` | package `ripe4rust-core`, library `ripe4rust_core`: the units, the version choice, the rules, the verdicts, the questions, the steps, the diagnostics and the plan. `#![no_std]`, no dependency, no technology. The ports in `traits/`, the data they hand over in `inputs/`, the hand-written arithmetic -- versions, requirements, dates -- in `values/`, the codes in `diagnostics/`, and the plan, from the policy to the questions, in `planning/` -- the version choice in `planning/units/choice/`, and the round's commands in `planning/round/`: `check`'s sorting of differences, `hold`'s pin and `guide`'s text. |
| `cli/` | package `cargo-ripe4rust`: the `cargo-ripe4rust` binary, `main.rs` a thin shim over the library `cargo_ripe4rust`'s entry point, `command::ripe4rust::Ripe4rust`. `src/command/` the command line and the four commands; `src/adapters/` the real implementations of the core's ports -- `cargo/` (`cargo metadata`, `cargo update --dry-run`, `cargo deny`), `registry/` (the index and the API through `curl`, with the day cache), `workspace/` (the workspace's files, `Cargo.lock`, the fingerprint, the plan document read back in `plan/`, and the parse of the sources in `sources/` -- `walking/` a target's modules, `naming/` what a file's names stand for and the lines naming a dependency, `exposure/` a library's public API), `toolchain/` (`rustc`, and the distribution's list of channel manifests through `curl`, with the day cache), `clock/`; `src/system/` the seams they reach the system through -- `traits/`, the records in `model/`, the real implementations in `real/`; `src/render/` the renderers. Its tests replay output recorded from the pinned toolchain, under `tests/fixtures/`, scrubbed of local paths; none reaches the network. |
| `xtask/` | gate orchestration, run as `cargo xtask`. `publish = false`. |

Each `tests/` mirrors its `src/`, one registry, `all_tests.rs`, per package.

## Quality Gates

### Mandatory after every change to any member's `src/` or `tests/`

`just stage1`
`just stage2`

If either gate is not green, the work is not complete.

Stage 1 is `cargo fmt`, clippy pedantic with `-D warnings` on every target, and the tests -- cargo built-ins only.
Stage 2 is `cargo xtask stage2`, the gates in this order, all on, nothing baselined:

| gate | asks |
|---|---|
| technology-free core | does the core stay `no_std`, dependency-free and free of technology |
| stern4rust | do the house coding rules hold (every member) |
| `cargo shear` | is every declared dependency used, in the section that uses it |
| `cargo dry4rust` | did this change add duplicated code to `core/src` or `cli/src` (floor 25) |
| `cargo crap4rust` | is any function complex and untested (threshold 15) |
| `cargo twin4rust` | does every source file have a mirrored test file |
| `cargo iceberg4rust` | is any file's private implementation risk at or above 7 |
| `cargo deny` | do the dependencies carry an advisory, a refused licence or a foreign source |

`just stage1` starts with `just sync`: `cargo update`, every dependency to its
newest compatible version, in the lock file only, while the tool is in
development. It needs the network. A requirement in a manifest never moves, so a
published crate's users can still resolve what they could; what it moves is
committed on its own. Breaking versions are a separate decision, not taken here.

The technology-free gate runs first, because its fix -- moving code into `cli` -- changes what every gate behind it
measures, and it needs no installed tool: `ripe4rust-core`'s manifest may name no crate in any section (read from
`cargo metadata`), `core/src/lib.rs` must carry `#![no_std]` and `#![forbid(unsafe_code)]`, and no file under
`core/src` or `core/tests` may reach `std::{env,fs,io,net,os,process,thread}`, a printing macro, a file-system method
of `Path` or `extern crate std` (read with syn, so string literals are data). It is iceberg4rust's gate, in
`xtask/src/gates/boundary/`. It cannot see a use inside a macro's arguments, and a method of the core's own named like
a file-system one would be reported.

The stern gate runs the release on PATH, as every other gate runs its tool: `cargo stern4rust` 0.24.0 holds all
thirty-nine of its rules here, nothing skipped and nothing unconfigured, so there is no stern baseline.
`docs/header.txt` holds the header every `.rs` file carries, and `stern4rust.toml` names it -- in the config rather
than the gate, so a hand-run of `cargo stern4rust` checks exactly what the gate checks.

dry4rust makes one run from the workspace root, with `xtask/`, `core/tests/` and `cli/tests/` excluded by full path,
so a copy from one crate into the other is seen. `dry4rust-baseline.json` records no group and may never gain one.
Re-record it only to drop a group that is gone, at the same floor: `cargo dry4rust --path "$PWD" --exclude
"$PWD/xtask" --exclude "$PWD/core/tests" --exclude "$PWD/cli/tests" --min-nodes 25 --baseline
"$PWD/dry4rust-baseline.json" baseline`.

The iceberg ceiling is a ratchet just above the worst file -- `xtask/src/main.rs`, 6.89, when the gates arrived:
lower it when the score improves, never raise it to turn a red build green.

cargo-deny runs **last**, and is the one gate that can turn red with nothing
changed: its advisories come from the RustSec database, so a newly published
one fails an untouched checkout, and it fetches that database over the network.
That is the point of it, and running last keeps it from hiding what the house
gates say about a change. `deny.toml` is the same file in every *4rust
repository and slotgate: licences MIT, Apache-2.0 and Unicode-3.0 -- the union
of what their trees carry -- crates.io as the only source, duplicate versions a
warning, and a wildcard requirement refused except a path dependency in an
unpublished crate. An accepted advisory goes in `[advisories] ignore` with its
id and its reason. The gate fetches the advisory database first, on its own and
up to three times, then checks `--frozen` -- locked and offline -- so a fetch
that keeps failing reports the gate as unable to run, never as an advisory
found. It decodes cargo-deny's exit code, one bit per failed check, into the
checks' names; a `deny.toml` it cannot read exits 1, as advisories does, and the
streamed output says which.

`rust-toolchain.toml` pins the compiler the gates pass with, 1.99.1; bump it in its own commit, with the gates.
`rust-version` is the minimum a user's own toolchain needs, and moves only when the code needs it.

`cargo install just`, `cargo install cargo-stern4rust --locked`, `cargo install cargo-dry4rust --locked`,
`cargo install cargo-crap4rust`, `cargo install cargo-twin4rust`, `cargo install cargo-iceberg4rust`,
`cargo install --locked cargo-shear --version 1.14.0`, `cargo install --locked cargo-deny --version 0.20.2`,
`cargo install cargo-llvm-cov` (crap4rust's coverage). Running `cargo ripe4rust` itself needs `curl` on the PATH, and
cargo-deny. `.github/workflows/ci.yml` runs both stages on Linux, Windows
and macOS.

### Every commit carries core's scores

The owner's rule of 2026-11-11: every commit message carries the grip and
braintax of `core`, and their ratio, scored as the commit stages it.
`.githooks/commit-msg` runs `cargo xtask grip-braintax` on the message's file,
which writes the scores in as a paragraph of their own above the trailers:
`core`'s `grip_absolute_total` over its `total_braintax` and their ratio, under
a title naming the releases of `cargo-grip4rust` and `cargo-braintax4rust` --
scores of different releases do not compare. The files under `core/` that the
index holds are exported into a scratch directory and scored there, so nothing
unstaged counts; `cli` and `xtask` are not scored. An amend scores again, in
place of the earlier scores; a message with no words is left alone, so git
still aborts its commit; a scoring that fails stops the commit, and
`--no-verify` skips it, deliberately. Git runs a repository's own hooks only
where a clone asks it to: `just hooks`, once a clone, on each machine.
`cargo xtask grip-braintax` with no file prints the scores.

`cargo install cargo-grip4rust`
`cargo install cargo-braintax4rust`

## This tool's rules

`docs/DESIGN.md` is the authority; these are the rules every slice keeps.

- It never applies: no manifest edited, no lock file moved, no commit made. It asks cargo what a move would do with
  `cargo update --dry-run`, which writes nothing (`ADR-CargoComputesTheLockMoves`).
- The core is technology-free: `#![no_std]`, no dependency, and nothing under `core/` touches the network, the file
  system, a process, the environment or stdout. Every input arrives through a trait -- the index, the repositories,
  the lock moves, the workspace's metadata and files, the advisories, the toolchain's releases, the source scan, the
  clock -- so every rule and every diagnostic is tested against fakes. Contracts go in `core/src/traits/`, their fakes
  in `core/tests/support/`, their implementations in `cli/`.
- Today's date is injected, so a plan is reproducible; no test reads the clock.
- No test reaches the network: the adapters are tested on recorded fixtures.
- Every problem is a diagnostic with a stable code, where it is and what to do about it; ripe4rust does not panic at
  its caller (`ADR-EveryProblemSaysWhatToDo`).
- No plan fails anything: a plan exits 0 whatever it says (`ADR-StalenessNeverFailsAGate`).

## Strict Rules

### Test preservation

**NEVER remove a test without explicitly asking the user first.** If a change makes a test fail, update the
assertion to the new correct behaviour; a test moved to another file is moved, not deleted.

### Assertion integrity

**NEVER relax a test assertion without explicitly asking the user first.** Relaxing means replacing a specific
assertion with a weaker one that would pass regardless of correctness.

### No ignored tests

**NEVER `#[ignore]` a test.** A test a stage cannot run goes in a package that stage leaves out.

### Docs follow the code

Every change adds its entry under `[Unreleased]` in `CHANGELOG.md`, and keeps `docs/DESIGN.md` true. An ADR is a
snapshot of a decision as it stands: change it when the decision changes.

## Orthogonality, trait surface and cognitive complexity

**When changing productive code, always maximize orthogonality and testable surface through traits, and minimize
cognitive complexity.**

- prefer extracting behavior behind traits so individual pieces can be tested and swapped independently
- prefer small, focused methods with a single responsibility over large methods with many branches
- prefer named structs with methods over free functions operating on external state
- when `crap4rust` or a reviewer flags a function as too complex, reduce it by extracting internal structs with
  methods and adding integration coverage — not by extracting standalone helper functions
- never increase cognitive complexity to pass a test; find the root cause and fix it there
- make constructors depend on traits, not directly on concrete implementations
- ALL dependencies are injected through the SINGLE constructor and stored in the struct
- apply the same split recursively to nested dependencies: trait first, state/data model second, concrete
  implementation third

## User coding standards

- one object per file; records where record-home puts them
- a record -- public fields and derives, no behaviour -- gets no test of its own
- no unnecessary comments in code
- unit tests are not allowed. Only integration tests are
- consolidate scattered functions inside structs as appropriate
- no `&mut` input parameters; prefer return values
- only use `pub mod` in `mod.rs` and `lib.rs`
- split test files so there is one test file per source file, named `<source file name>_tests.rs`
- in `all_tests.rs`, reference test files one by one without `#[path = ...]`
- apply AAA (`Arrange`, `Act`, `Assert`) structure to tests with blank-line separation between the three sections
- use `// Arrange & Act` if there is no separate `Arrange`
- use `// Act & Assert` if there is no separate `Act`
- add the repository copyright and license header to every Rust source file
- tests should be named as follows `<method under test>_<test description>_<result>`
- do not use fully qualified paths; use `use` imports instead
