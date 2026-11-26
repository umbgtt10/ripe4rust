# ADR-CargoComputesTheLockMoves

- **Status:** Accepted
- **Date:** 2026-11-26

## Context

A compatible unit is "the semver-compatible lock-file moves a direct
dependency pulls in". To report it, ripe4rust has to know which crates move,
and to which versions, if the unit is applied. That is dependency resolution:
feature unification, target filters, `rust-version`-aware choices under
resolver 3, the `[patch]` and `[replace]` sections, the lock's existing pins.

## Decision

ripe4rust asks cargo. A compatible unit's moves are what
`cargo update --recursive -p <crate>` reports when asked only to report; the
whole workspace's are what a reporting `cargo update` reports, and what no
unit reaches is the last unit, the rest. Reporting writes nothing, so "it
never applies" holds.

Units are taken in plan order, by crate name. A transitive move that several
units share belongs to the first, because applying the first moves it for the
others.

A breaking unit's moves are not computed: they need the manifest edited, and
ripe4rust edits no manifest. The plan names the direct crate and its target;
the caller sees the rest when it applies.

## Rejected alternatives

- **Resolve in ripe4rust.** Re-implementing cargo's resolver is a project of
  its own, and every place it differed from cargo would put a move in the plan
  that applying does not make, or leave one out.
- **Edit a copy of the manifest to resolve breaking units too.** A copy of the
  workspace for every breaking unit costs a full resolution each, and writes
  where the tool promises to write nothing. What a caller needs to decide is
  the target, not the transitive tail.

## Consequences

Compatible units are exact: what the plan says is what applying does. The
lock-moves port in the core is faked in tests like every other input, and the
adapter is one process call per direct dependency. Under resolver 3, the
`rust-version` filter for compatible moves is cargo's, not ripe4rust's.
