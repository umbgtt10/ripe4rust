# ADR-AMoveBackIsAsked

- **Status:** Accepted
- **Date:** 2026-11-26

## Context

A compatible unit applies what `cargo update --dry-run --recursive -p <crate>`
reports, and cargo's resolver may take a crate to a lower version on the way:
a newer release of one crate can require an older one of another. In the
first dry round a compatible unit of over a hundred moves took one crate from
0.1.7 back to 0.1.6 -- a release that was not yanked -- and the plan said
nothing of it: the unit read as ripe, and the move back was one line among
the others.

A move back is not what an upgrade promises. It can drop a fix the newer
release made, and it is invisible in a unit named for another crate.

## Decision

Every move inside a unit to a lower version of a crate is named among the
unit's reasons, "moves <crate> back from <from> to <to>", and listed in its
evidence as `downgrades`.

One that leaves a release that is not yanked makes a unit the rules call
ripe **ask**, keeping the rule that called it ripe and its urgency, with the
suggestion `moves-back`: a person decides whether the move back is wanted.
It is applied after the no-gates downgrade, so a workspace with no gates is
still asked to declare them first.

One that leaves a yanked release is the cure, not a regression: it is named,
"moves <crate> back from the yanked <from> to <to>", and the outcome is
unchanged. Such a move clears the yanked release, so the unit is urgent by
rule 2 already.

A unit that asks or ripens for another reason keeps its outcome, the move
back named among its reasons.

## Rejected alternatives

- **Hold the unit.** The rest of its moves are still worth taking, and a
  hold hides the question it raises.
- **Ask every move back, yanked or not.** Leaving a yanked release is what
  the maintainer asked for, and asking it would delay the cure.
- **Split the move back out of the unit.** cargo moves the crates together;
  a unit without it would not be what cargo applies.

## Consequences

The agent stops for a person on a unit it would otherwise have taken, and
the question names the move back. A plan with no move back is unchanged.
