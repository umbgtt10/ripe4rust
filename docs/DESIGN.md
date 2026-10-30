# ripe4rust -- design

Status: proposal. Nothing here is implemented.

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

It decides; it never applies. The plan it writes is the input to whoever does
the upgrading -- an agent, a script, a person.

## What it is not

- Not an updater. It edits no manifest, moves no lock file, makes no commit.
- Not a gate that fails on staleness. A workspace being behind is information,
  not a defect in the change being gated. (An option to fail on *urgent* items
  is an open point.)
- Not a vulnerability scanner. It reads advisories, through cargo-deny, as one
  input among several.

## Vocabulary

- **Unit** -- the smallest upgrade worth one commit:
  - a **breaking** unit: one direct dependency moved to a new major (for
    `0.x`, a new minor), with whatever the lock file moves along with it;
  - a **compatible** unit: the semver-compatible lock-file moves a direct
    dependency pulls in, grouped by that dependency. One commit per transitive
    crate would be hundreds; one for everything would not bisect.
- **Line** -- every release of one major (`2.x`), or one minor for `0.x`
  (`0.13.x`).
- **Pin** -- a dependency held back on purpose, with its reason and what
  releases it.
- **Policy** -- the workspace's facts and thresholds the rules read.
- **Plan** -- the output: one row per unit, with its outcome and the reasons.

## Inputs

| signal | from |
|---|---|
| available versions, release dates, yanked flags, `rust-version` | the crates.io index and API |
| the dependency graph, duplicates, which targets reach a crate | `cargo metadata` |
| advisories against the version in use | `cargo deny check advisories --format json` |
| the toolchain in use | `rust-toolchain.toml` |
| footprint: files and lines that name the crate | the workspace's sources, parsed |
| public exposure: the crate's types in a published crate's public API | the same parse |
| pins and policy | `[workspace.metadata.ripe4rust]` in `Cargo.toml` |
| today's date | injected, so a plan is reproducible |

## The rules

Applied in order to every unit; the first that matches decides.

| # | when | outcome | suggestion |
|---|---|---|---|
| 1 | an advisory names the version in use | **ask**, urgent | migrate now -- even past a pin |
| 2 | pinned, release condition not met, review date not reached | **ripening** | -- |
| 3 | pinned, review date reached | **ask** | revisit the pin |
| 4 | a freeze window is open | **ripening** | until the window closes |
| 5 | the unit reaches a target the policy marks as needing more than the gates (firmware, say) | **ask** | apply, then validate there |
| 6 | breaking, and the crate's types are in a published crate's public API | **ask** | the upgrade is a breaking change of ours too |
| 7 | breaking, and the new line is young (under `young-days` or `young-releases`) | **ripening** | until it settles |
| 8 | the new version needs a newer Rust than the toolchain | **ripening** | until the toolchain moves |
| 9 | compatible | **ripe** | -- |
| 10 | breaking, footprint under `small-footprint` | **ripe** | -- (a red gate turns it into ask) |
| 11 | breaking | **ask** | migrate now / plan it / wait, by the verdict below |

### The verdict for a breaking unit that needs asking

- **Migrate now**: the line in use has had no release for `stale-months`
  while the new one is active, or migrating removes a second copy of the crate
  from the tree.
- **Plan it**: the new line has settled and the footprint is moderate.
- **Wait**: anything else, with the reason.

### Against drift

Rule 2 must not become "never". A pin carries a review date or a release
condition, and two escalations override `ripening`:

- the workspace is two majors or more behind;
- the line in use has had no release for `stale-months` while a newer line is
  active.

Either turns the unit into **ask** with the suggestion "overripe". Rule 1
already overrides everything.

## Pins and policy

Both live in the workspace's `Cargo.toml`, where `cargo metadata` hands them
over and every other tool in the family keeps its configuration:

```toml
[workspace.metadata.ripe4rust]
# Crates published to crates.io: a breaking dependency in their public API
# makes the upgrade a breaking change of theirs (rule 6).
published = ["mytool-core"]
# Targets whose code the gates alone cannot validate (rule 5).
validate-beyond-gates = ["thumbv7em-none-eabihf"]
# A freeze: everything ripening until the date (rule 4).
freeze-until = "2027-01-15"
# Thresholds, with these defaults.
young-days = 30
young-releases = 3
stale-months = 6
small-footprint = 10   # lines naming the crate

[workspace.metadata.ripe4rust.pins]
# Every pin has a reason, and a review date or a release condition, or both.
syn = { reason = "the parser every measure is written against; 3 is a migration of the visitors", review = "2027-01-31" }
probe-rs = { reason = "the rig's CLI is 0.31; the library moves with it", until = "the rig's CLI moves to 0.32" }
```

A pin without a reason, or without a review date or a release condition, is
an error, reported before any rule runs. A release condition in words is
honest but cannot be checked; it falls back to the review date, and a pin with
neither is refused.

## The plan

A table for a person, JSON for a caller -- one document either way:

```
unit                  kind        from      to        outcome    reasons
syn                   breaking    2.0.119   3.0.9     ripening   pinned: the parser ...; review 2027-01-31
clap (+3 transitive)  compatible  4.6.6     4.6.7     ripe       compatible
smoltcp               breaking    0.13.1    0.14.0    ask        reaches thumbv7em-none-eabihf; plan it: footprint 42 lines in 7 files
anyhow                compatible  1.0.102   1.0.104   ask        urgent: RUSTSEC-2026-0190
```

```json
{
  "date": "2026-10-30",
  "units": [
    {
      "crate": "syn",
      "kind": "breaking",
      "from": "2.0.119",
      "to": "3.0.9",
      "moves": ["syn"],
      "outcome": "ripening",
      "suggestion": null,
      "rule": 2,
      "evidence": {
        "line_in_use": { "last_release": "2026-07-15", "releases": 90 },
        "new_line": { "first_release": "2026-07-18", "releases": 10 },
        "rust_version": "1.71",
        "footprint": { "files": 11, "lines": 20 },
        "public": false,
        "duplicate_in_tree": true,
        "advisories": []
      }
    }
  ]
}
```

The rule that decided and the evidence it read are always included: a caller
asking a person must be able to show why.

## How a caller uses it

The plan is the input to an upgrade round, done the same way every time:

1. Run `cargo ripe4rust --format json`.
2. For each **ripe** unit, one at a time: apply it (the workspace's own
   command, `cargo update -p <crate>` for a compatible unit), run every
   mandatory gate, commit if green -- one commit per unit, so each is revertible
   -- and if red, revert and treat the unit as **ask**, with the failure
   attached. A compatible group that fails is bisected: split, find the crate
   that broke it, hold that one, commit the rest.
3. Gather every **ask** unit into one question, each with its suggestion and
   the evidence that led to it.
4. Report every **ripening** unit, with what will release it.

The applying stays in the workspace, which knows its own gates. ripe4rust
only decides, so its answer for a given set of inputs is the same every time,
and it can be tested like the rest of the family.

## Architecture

The family's shape: a technology-free `ripe4rust-core` and a `cargo-ripe4rust`
CLI.

- **core** -- the units, the rules, the verdicts, the plan. `no_std`-friendly
  and free of I/O: every input arrives through a trait -- the registry, the
  workspace's metadata, the advisories, the source scan, the clock -- so every
  rule is tested against fakes, the date included.
- **cli** -- the real implementations: the crates.io API (with a cache, and an
  honest user agent naming the tool and nothing personal), `cargo metadata`,
  `cargo deny`, a `syn` parse of the sources for footprint and exposure, the
  system clock, and the table and JSON renderers.

Exit codes: `0` when a plan was produced, whatever it says; `2` when it could
not be (a pin without a reason, an unreadable manifest, the registry
unreachable).

## Open points

1. **Footprint from a parse or a grep?** A `syn` parse finds paths and uses
   precisely; a text search is cheaper and good enough for "small or not".
2. **Failing on urgent items.** A `--fail-on urgent` would let a stage gate on
   advisories through ripe4rust instead of cargo-deny, or beside it. Probably
   not: one tool, one question.
3. **Release conditions that can be checked.** "When crate X reaches version
   Y" could be evaluated; free text cannot. Worth a small grammar, or not?
4. **Groups across crates.** Some crates move together (a framework and its
   macros). The registry does not say so; the graph mostly does. Grouping by
   shared release could merge units that otherwise fail apart.
5. **The registry's rate limits and offline runs.** A cache keyed by date
   keeps a round reproducible and polite; how stale may it be?
6. **Where the young and stale thresholds come from.** Defaults are guesses;
   the first rounds across real workspaces should tune them.
