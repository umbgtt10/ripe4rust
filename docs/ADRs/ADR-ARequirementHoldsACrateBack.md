# ADR-ARequirementHoldsACrateBack

- **Status:** Accepted
- **Date:** 2026-11-27

## Context

A breaking unit edits the requirement on its crate and runs `cargo update -p
<crate>`. cargo resolves the new requirement for the workspace's members; any
other package of the tree that requires the crate keeps the copy its own
requirement allows. Where one requires the line in use, the edit does not
move the crate: it adds the target beside it.

The first round that applied a plan met it. `breaking:prost`, 0.13.5 to
0.14.6, footprint three lines in one file, was called ripe by rule 11. But
`tonic` 0.12.3 requires `prost` `^0.13`: the edit added 0.14.6 beside 0.13.5,
and `check` found 0.13.5 still in the lock, not as planned. Nothing about the
migration was wrong; it could not land alone. What releases it is `tonic`'s
own upgrade -- its unit was in the same plan, asked.

The plan already reads what shows it: `cargo metadata`'s resolve names, for
every package of the lock, each dependency at its version with the
requirement that asked for it, which the verdict's second-copy test reads
already.

## Decision

A breaking unit's crate is **held back** by every package of the lock -- the
unit's own members aside, as a group moves together -- that depends on the
version in use with a requirement the unit's target does not meet.

- Each dependent is named among the unit's reasons, whatever rule decided
  it: "tonic 0.12.3 requires prost ^0.13: moving it adds 0.14.6 beside
  0.13.5", every dependent of one crate in one reason. The evidence lists
  them as `held_by`: the dependent, its version, the crate it requires and
  the requirement.
- A unit the rules would call ripe or ask **ripens**, keeping the rule that
  decided it, with the suggestion `until-its-dependents-move`: nothing about
  it is for a person to decide while the move cannot land, and the
  dependent's upgrade, a unit of its own, is what releases it. The next plan
  reads the lock that upgrade leaves.
- A held move clears no advisory and no yanked release (below), so the unit
  is urgent only for what another of its moves clears; such a unit keeps its
  ask, the dependents named: an urgent unit is never held. One already
  ripening -- a pin, a freeze, a young line -- keeps what holds it, and gains
  the reason.
- It is applied after drift, so an overripe unit its dependents hold ripens
  too, and before the no-gates downgrade, which reads only what is ripe.

### An advisory the held move would clear

A held-back move clears nothing: the version it leaves stays for the
dependent, and with it every advisory against that version. So no advisory
and no yanked release is credited to it. Where no other unit clears the
advisory, its row stays, urgent, "replace or accept"; it holds the move
that would clear it in its evidence's `held`, names the dependents among its
reasons and in `held_by`, and its question says why: the upgrade that would
clear it ripens until its dependents move.

The unit itself is then not urgent, and ripens with the rest. Keeping it an
urgent ask, its question saying the move alone will not clear the advisory,
was weighed and refused:

- an urgent unit is never held, so its answers are to take it or to accept
  the advisory. Taken, the edit adds the target beside the version in use,
  `check` finds that version still in the lock, the unit is reverted and
  asked again -- a loop no answer ends but the dependent's own upgrade;
- accepting is what the row already offers, so the person would be asked
  twice, urgently, about one advisory, by two rows that each say the other
  is the way out;
- urgent means a unit clears an advisory or a yanked release; this one does
  not, and saying it is urgent would put it ahead of the units that do.

The way out is the dependent's upgrade, which has a unit of its own. Once it
lands, the next plan finds the crate unheld, credits the breaking unit with
the advisory, and asks it urgently.

## Rejected alternatives

- **Ask it, "wait".** Rule 12's verdict could say wait. But the question has
  one answer while the dependent stays, and the person is already asked
  about the dependent's own unit; a second question about the same decision
  costs their attention for nothing.
- **Leave the version out as no candidate**, as a version needing a newer
  Rust is (ADR-AnUnbuildableVersionIsNoCandidate). The unit would vanish, and
  with it the line it is behind on; a dependent that never moves would hide
  the crate's new majors for good.
- **A rule of its own, between rules 6 and 7.** It would renumber rules 7 to
  13, which plans, tests and callers read; an outcome changed with the rule
  kept is what the no-gates downgrade and a move back already do.
- **Reading the dependents' newer releases** in the index to say which one
  releases the crate. The index's dependency lists would be one more input to
  read and trust for every dependent; the lock after the dependent's upgrade
  says it for certain, at the next plan.

## Consequences

A breaking unit no longer reads ripe where its edit can only duplicate the
crate, and its reason says which dependent holds it and what the move would
do. A rule number no longer says the outcome by itself: rule 11 can ripen.
`check` still fails such a unit applied regardless, the version in use still
in the lock. A package only a git source brings in is not in the lock's
registry packages, and holds nothing here.
