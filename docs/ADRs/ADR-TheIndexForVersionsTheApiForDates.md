# ADR-TheIndexForVersionsTheApiForDates

- **Status:** Accepted
- **Date:** 2026-11-26

## Context

The rules need, for every crate in the lock, its versions, their yanked flags
and their `rust-version`; and, for direct dependencies only, release dates --
whether a line is young, whether the line in use has gone stale. crates.io
serves the first through its sparse index, a static, cached file per crate,
and the dates through its API, which asks clients for at most one request a
second and a user agent that identifies them.

## Decision

- The sparse index answers versions, yanked flags and `rust-version` for every
  crate.
- The API is called only for direct dependencies' release dates, at most one
  request a second, with a user agent naming the tool, its version and its
  repository, and nothing personal. If the index carries release dates by the
  time this is built, the API is not called at all.
- Every answer is cached for the day it was read. A plan names the date its
  registry answers are from; the same cache and the same injected date give
  the same plan. A run with the registry unreachable and nothing cached for the
  day exits 2.

## Rejected alternatives

- **The API for everything.** Hundreds of calls for a large lock, one a
  second, to learn what the index serves faster and cheaper.
- **The crates.io database dump.** Complete and offline, but hundreds of
  megabytes a day for a few dozen crates.

## Consequences

A round costs one index read per crate in the lock and one API call per
direct dependency, cached for the day. Plans are reproducible from the cache,
which tests use as their fake.
