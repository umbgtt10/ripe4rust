# ADR-TheIndexForVersionsAndDates

- **Status:** Accepted
- **Date:** 2026-11-26

## Context

The rules need, for every crate in the lock, its versions, their yanked flags,
their `rust-version` and, for direct dependencies, their release dates --
whether a line is young, whether the line in use has gone stale. Grouping
crates released together also needs a direct dependency's repository.
crates.io serves the sparse index, a static, cached file per crate, and an API
that asks clients for at most one request a second and a user agent that
identifies them.

When this was first decided, release dates were expected to need the API.
Checked on 2026-11-26, the sparse index carries them: every entry has a
`pubtime`, beside `vers`, `yanked` and `rust_version`.

## Decision

- The sparse index answers versions, yanked flags, `rust-version` and release
  dates (`pubtime`), for every crate in the lock. Its files are static, and
  cargo itself fetches many at once: those the day's cache lacks are fetched
  at most eight at a time, the answers kept in the crates' order.
- The API is called only for a direct dependency's repository, for grouping:
  at most one request a second, with a user agent naming the tool, its version
  and its repository, and nothing personal.
- Every answer is cached for the day it was read, under
  `~/.cache/ripe4rust/<date>/`. A plan names the date its registry answers are
  from; the same cache and the same injected date give the same plan.
  `--offline` reads the cache alone, an earlier day's with a warning. With the
  registry unreachable and nothing cached for the day, no plan is produced
  (`registry-unreachable`).

## Rejected alternatives

- **The API for everything.** Hundreds of calls for a large lock, one a
  second, to learn what the index serves faster and cheaper.
- **The crates.io database dump.** Complete and offline, but hundreds of
  megabytes a day for a few dozen crates.

## Consequences

A round costs one index read per crate in the lock and one API call per
direct dependency, cached for the day. Plans are reproducible from the cache,
which tests use as their fake.
