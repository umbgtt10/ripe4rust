# ADR-ARepointIsToldNeverJudged

- **Status:** Accepted
- **Date:** 2026-11-28

## Context

`check` compares the lock a plan recorded with the lock now, package by
package: a crate's version added, removed or moved (ADR-ACheckAfterEveryUnit).
It never saw an edge -- which version of a crate a package depends on.

The first rounds that applied a plan, on 2026-11-27, met what that misses.
`cargo update -p proptest` took the `windows-sys` dependency of `errno`,
`rustix` and `tempfile` from 0.61.2 to 0.52.0. Both versions were already in
the lock, and no version was added or removed. The next unit, `cargo update -p
tonic`, took them back to 0.61.2. `check` said "as planned" both times and
named none of it; the only witness was the agent reading the raw diff of
`Cargo.lock`. Like a crate the units share (ADR-AUnitIsCheckedOnWhatItOwns),
such a move depends on the order the units are applied in.

Cargo writes a dependency with its version -- `"windows-sys 0.52.0"`, a source
after it where the version alone is ambiguous -- only where the lock holds the
crate at more than one version. A bare `"windows-sys"` means it holds one, and
an edge to a crate held once cannot move between locked versions.

## Decision

The plan records the lock's versioned edges, and `check` tells every one
re-pointed between versions the lock already held. It judges none of them.

- **The plan's `workspace.edges`**, after `git`: every dependency
  `Cargo.lock` writes with a version, of every package -- members, crates.io
  packages, a git source's crates -- on a crates.io package, as `<crate>
  <version> -> <dependency> <version>`, in order. Bare edges are left out:
  they cannot re-point.
- **A re-point** is an edge whose dependent is in both locks at the same
  version, and whose dependency moved from one version to another, both held
  by the plan's lock and by the lock now. `check` reports each in the new
  `repointed`, after `shared`: "`errno` 0.3.14's `windows-sys` 0.61.2 ->
  0.52.0, both already in the lock". An edge whose old version left the lock,
  or whose new one joined it, is explained by that version's move, and is not
  reported.
- **Told, never judged.** A re-point is not a difference: `as_planned`, the
  exit code and whether the plan is stale are what they would be without it.
  It never fails a check and never makes a plan stale.
- **Cumulative**, against the plan's lock, as everything `check` sorts is: a
  re-point an earlier unit made is reported at every later check while it
  stands, and one a later unit undoes is reported no more. The agent names in
  the unit's commit message the re-points its check lists that the check
  before did not, and those it no longer lists; the gates judge them, as they
  judge everything else the unit changed.
- **An older plan** -- written by 0.1.2 or earlier -- has no `edges`. It is
  read and checked as before, and the check says once, with the notice
  `plan-without-edges`, that edges cannot be compared until the next plan.

## Rejected alternatives

- **A re-point as a difference**, exit 1. The step did what cargo does: the
  resolver chose among versions the lock already held, as the units around it
  let it. Reverting would throw away a unit that is right, and asking would
  put to a person what the build and the tests answer -- whether the
  dependents build and pass against the version they now take. That is the
  gates' question.
- **A re-point as a stale plan**, exit 3. Nothing changed outside the plan: a
  unit of it moved the edge.
- **Holding a unit to the edges it moves.** Which edges a unit re-points
  depends on the units applied before it, which the plan cannot know: each
  unit's moves are computed alone (ADR-AUnitIsCheckedOnWhatItOwns).
- **Every edge, bare ones too.** A lock of a few hundred packages has a few
  thousand edges; the plan would grow by them for nothing, as a bare edge
  cannot re-point between locked versions.
- **The edges from `cargo metadata`'s resolve**, which the plan reads already.
  `check` reads the lock as a file, without cargo, so a lock a manifest edited
  by hand no longer matches still reads; the plan records what `check`
  compares, read the same way.
- **The re-points since the last check alone.** `check` keeps no state
  between runs, and holds everything to the plan; a delta would need one.

## Consequences

A dependency re-pointed between locked versions is named in the check that
first sees it, and in the commit of the unit that made it; the gates still
decide whether it holds. A plan grows by one line per versioned edge, a handful
in most workspaces. A plan written before still reads, its check saying once
that it cannot compare edges.
