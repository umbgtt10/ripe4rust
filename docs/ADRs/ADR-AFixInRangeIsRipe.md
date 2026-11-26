# ADR-AFixInRangeIsRipe

- **Status:** Accepted
- **Date:** 2026-11-26

## Context

The first design's rule 1 answered every advisory with "ask, urgent". On
2026-11-25 RUSTSEC-2026-0592 turned two workspaces' cargo-deny gate red over
`url` 2.5.8, pulled in through a test-only dependency; the fix was 2.5.9, a
lock-only move inside the semver range. That is the safest upgrade there is and
the most pressing, and under rule 1 it would have waited for a person. A week
earlier `spin` 0.10.0 was yanked; 0.10.1 was the same kind of move.

## Decision

An advisory and a yanked release are judged by the unit that clears them:

1. the unit reaches a target the policy marks as needing more than the gates:
   **ask**, urgent -- apply, then validate there;
2. the unit is compatible: **ripe**, urgent;
3. the unit is breaking: **ask**, urgent -- migrate now.

All three hold even past a pin or a freeze. An advisory or a yanked release
that no unit clears gets a row of its own: **ask**, urgent, "replace the
crate, or accept it in `deny.toml`'s `[advisories] ignore` with its reason".

A yanked release in use is treated as an advisory: a maintainer withdrew it.

## Rejected alternatives

- **Ask every urgent unit.** A person approving a patch release for a
  published advisory adds a delay and no judgement.
- **Let a freeze hold urgent units.** A freeze protects a release from churn;
  an advisory is the one churn a release should not ship without.

## Consequences

The caller takes urgent ripe units first, gated like any other, so a red gate
still turns one into ask. Firmware workspaces keep a person in the loop for
every change, urgent included.
