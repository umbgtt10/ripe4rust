# ADR-AUnitIsCheckedOnWhatItOwns

- **Status:** Accepted
- **Date:** 2026-11-27

## Context

A plan is made from one snapshot of the lock. Each compatible unit's moves are
what `cargo update --dry-run --recursive -p <crate>` reports on that lock,
alone (ADR-CargoComputesTheLockMoves), and the rest is what `cargo update
--dry-run` moves for the whole workspace beyond them. The round applies the
units one at a time, in plan order, and checks each against the plan
(ADR-ACheckAfterEveryUnit).
Applied one after another, the units move the lock under the units after
them, and the moves cargo makes then are not always the moves it computed for
each alone.

The first round that applied a plan, across a real workspace, met four checks
that said not as planned; three of them with nothing wrong:

- one unit took `wasip2` from 1.0.3 to 1.0.4; the next unit, applied on that
  lock, removed `wasip2`, as cargo does for the whole workspace. Its check
  found 1.0.3 gone and 1.0.4 not there, a move no unit made: not as planned.
  The rest would have named the removal, but a move was left out of a later
  unit, and of the rest, by the crate and the version it leaves alone, so the
  whole workspace's removal of 1.0.3 went with the first unit's move of it;
- a unit removed `r-efi` 6.0.0 -- the lock held 5.3.0 beside it --, which
  cargo keeps for the whole workspace. Applied on the lock the units before
  it left, it kept it: its move missing, not as planned;
- the rest's moves were crates only removed and added, which come only with
  the units that need them; with two of those units reverted, nothing came,
  and `apply` had no step to take: not as planned. Two of its additions were
  no move beyond the units at all: `rand`, in two versions, which cargo prints
  for the whole workspace as two removals and two additions where each unit's
  report printed one version moving;
- the fourth was a breaking unit, and another matter: a dependent's
  requirement held its crate where it was, so the edit added a second copy
  (ADR-ARequirementHoldsACrateBack).

The three turned ripe units into asks: an agent trusting `check` reverts
what it should have committed.

## Decision

`check` holds a unit to what it owns, and the plan says which crates no unit
owns alone.

- **Moves are claimed version by version.** A later unit's move, and the
  whole workspace's, is left out where the moves taken before it leave the
  version it leaves and reach the one it reaches. A crate in two versions is
  told apart however cargo prints it, and a later unit taking a version to
  another place than an earlier one keeps its move.
- **A rest with no step is no unit.** The rest is formed where one of its moves
  takes a crate from a version to another, a move a `--precise` step takes.
  Crates only added or removed come with the units: cargo adds or removes a
  crate only as another's move needs it.
- **The plan names the shared crates**, in `shared`: those whose moves, each
  unit's computed alone, do not add up to the whole workspace's -- a unit
  leaves a version the whole workspace keeps or reaches one it does not, two
  units leaving one version take it to different places, or the whole
  workspace moves a version no unit does. A crate pinned with `hold = "all"`
  is never shared: every move of it stays a difference.
- **A unit owns** every move of a crate the units do not share, and of a
  shared one, the move of its own crate, the move of a split, and the rest's
  moves its `--precise` steps take. `check` holds it to those: a move it owns
  not made, a change no move explains of a crate the units do not share, or of
  its own crate, is a difference, **1**, as before. A move it does not own
  that the lock does not show, and a change of a shared crate no move
  explains, are no difference; they are reported in the check's `shared`
  where the unit moves the crate. A shared crate only other units move is
  theirs to say: reported at every later check, it would only repeat what
  an earlier one said.
  `hold` counts such a change as no change outside the plan. The exit codes
  are unchanged: **0** as planned, **1** not as planned, **2** an error to act
  on, **3** stale.

## Rejected alternatives

- **Order-aware expectations**, each unit's moves computed on the lock the
  units before it leave. cargo computes them only on a lock that is written,
  and ripe4rust writes none; `cargo update --dry-run -p a -p b` comes close, at
  a resolution a unit. But the round does not apply every unit: an ask waits
  on a person, a red gate reverts its unit, a breaking unit's tail is known
  only once applied. Expectations computed for every unit applied would be
  wrong from the first that is not, which is the case the round met.
- **A unit naming the earlier units it depends on.** cargo reports what moves,
  not why; the cause of a joint move is in the graph, and `check` would still
  have to decide what an absent dependency means.
- **Leniency for every crate two units name.** Nearly every transitive crate --
  `serde`, `syn`, `libc` -- is in several units' trees, mostly moved alike;
  `check` would hold a unit to its own crate and nothing else.
- **Holding a shared crate to the versions the plan names.** Applied in order,
  the units can take a crate to a version no unit's report alone chose: one
  between the lock's and the whole workspace's.

## Consequences

A ripe unit is no longer made an ask by the units beside it. `check` still
catches what a step does that it should not: a move of a crate the units do
not share, another version of its own crate or none, a split's move not made,
the rest's step not taken, a pin crossed, a line edited by hand. A shared
crate's move is reported at whatever version it reaches -- a release published
between the plan and the round included --, and the gates are what validate
it. A unit that owns none of its moves is listed applied by them all. The
plan document gains `shared`, after `gates`, and the check document `shared`,
after `differences`; a plan written before reads as sharing none, and is
checked as strictly as it was.
