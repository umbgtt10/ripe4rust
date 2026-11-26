# ADR-AnUnbuildableVersionIsNoCandidate

- **Status:** Accepted
- **Date:** 2026-11-26

## Context

The first design had a rule, eighth of eleven: "the new version needs a newer
Rust than the toolchain -- ripening, until the toolchain moves". Coming after
the rules that ask, it let a breaking unit in a published crate's public API be
asked about (rule 6 then) although it could not be built at all: a person would
be asked to decide an upgrade nobody could take.

## Decision

Needing a newer Rust than the toolchain is not an outcome but a filter: such a
version is never a unit's target. The same filter drops yanked releases and
pre-releases. A breaking unit moves to the newest settled line the toolchain
can build; a compatible unit to what cargo picks, which under resolver 3
already respects `rust-version`.

The versions the filter held back are recorded as evidence on the toolchain
unit, which is the upgrade that would release them.

A version the toolchain builds but the workspace's declared `rust-version`
does not cover stays a candidate: its unit raises `rust-version` too, and for a
published crate that is asked (rule 8), since its users see it.

## Rejected alternatives

- **Keep the rule, earlier in the order.** It would answer "ripening" for a
  unit whose better target -- an older release of the same new line, buildable
  today -- was never considered.

## Consequences

No row asks about an upgrade that cannot be taken. Rule numbers shift: the
table has no toolchain rule. The toolchain unit carries, as its evidence,
everything it would release.
