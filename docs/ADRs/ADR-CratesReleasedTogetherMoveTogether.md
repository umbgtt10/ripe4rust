# ADR-CratesReleasedTogetherMoveTogether

- **Status:** Accepted
- **Date:** 2026-11-26

## Context

Some crates only work at matching versions: a framework and its derive
macros, a family released in lockstep. Taken as separate units, the first
fails its gates because the second has not moved, both are reverted and asked,
and the round learns nothing. The registry does not say which crates belong
together.

## Decision

Crates form one unit, a group, when

- they share a repository and released the same version on the same day, or
- one requires an exact version (`=x.y.z`) of the other.

A group is judged as one unit: its footprint and exposure are its members'
together, its youth its youngest member's, and an advisory on any member is
the group's.

## Rejected alternatives

- **Groups declared in the policy.** Possible, and the same drift as any list
  kept by hand; the evidence is in the registry and the graph.
- **Group by name prefix.** `serde` and `serde_json` share a prefix but not a
  release.

## Consequences

The grouping is a heuristic, and "the same day" is a threshold; the first dry
round tunes it with the others.
