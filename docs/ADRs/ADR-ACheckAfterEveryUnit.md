# ADR-ACheckAfterEveryUnit

- **Status:** Accepted
- **Date:** 2026-11-26

## Context

An agent applying a unit has no way to tell, by itself, whether the step did
what the plan meant: `cargo update` can move more than the plan said, or less,
or something else when the registry moved overnight; an edit to a manifest can
miss a line; a breaking unit's transitive moves are unknown until applied. And
between planning and applying, the workspace can change -- another round, a
person's commit -- leaving the agent to act on a plan that no longer holds.
Unnoticed, each of these ends in a commit that is not what it says.

## Decision

The plan records the workspace it was made from: a fingerprint of
`Cargo.lock`, every manifest and `rust-toolchain.toml`, the lock's packages
and versions, the dependencies the lock writes with a version
(ADR-ARepointIsToldNeverJudged), and every line a unit's `apply` can edit --
each requirement, each `rust-version`, the channel -- with what it said. `cargo ripe4rust check <unit> --plan <file>` reads the
workspace again and sorts every difference into the units of the plan that
explain it:

- **0** -- every difference belongs to units of the plan, and the unit named
  shows exactly its expected moves. The output lists the units already applied
  and those still to go, so an interrupted round resumes where it stopped.
- **1** -- the unit named shows nothing, or not what it should: a move
  missing, another version, an extra move, a crate pinned on its line taken to
  a new one. Every difference is listed, with the step: revert, and the unit
  becomes ask.
- **3** -- a difference no unit of the plan explains: the plan is stale. Plan
  again and go on with the new plan.
- **2** -- it could not check: an unreadable plan, an unknown unit.

For a breaking unit, `check` reports the transitive moves the edit made,
which the plan could not know.

## Rejected alternatives

- **Compare with the version control's last commit.** It would tie ripe4rust
  to git, and the last commit is not the plan's base once units are applied
  and committed one by one.
- **Trust the agent to read the lock.** The agent has no context to know what
  "right" looks like; the plan does.

## Consequences

The plan carries the lock's package list, a few hundred lines in a large
workspace, and the editable lines beside it: the fingerprint says the
workspace changed, not where, so without them a requirement edited by hand
could not be told from one a unit wrote. `check`'s sorting is core logic over two lock states and the plan,
tested against fakes; reading the files and hashing them is the CLI's. Its
exit codes are a step's result for the agent, not a gate's verdict on a
change (ADR-StalenessNeverFailsAGate).
