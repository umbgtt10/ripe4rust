# ADR-APinHoldsALine

- **Status:** Accepted
- **Date:** 2026-11-26

## Context

A pin first held "the unit". But most pins are about a new major: `syn` held on
2 because 3 is a migration. Held as a whole, that pin would also hold every
patch release of 2 -- fixes the reason for the pin says nothing against.
Pins can also name crates the workspace does not depend on directly, which a
compatible unit may move as part of another crate's update.

## Decision

A pin says what it holds:

- `hold = "line"`, the default: the move to a new line. The crate still takes
  its compatible releases.
- `hold = "all"`: every move. A compatible unit whose moves include the crate
  leaves its move out and applies the others one by one.

A pin on a crate that is not a direct dependency holds the same way. Its new
line can only arrive through a breaking unit's edit, which the plan cannot
foresee; `check` reports it after the edit, and the unit becomes ask.

## Rejected alternatives

- **Every pin holds everything.** The common pin would cost a workspace its
  patch releases for as long as the migration waits.
- **Pins on direct dependencies only.** A transitive crate that breaks a build
  could then not be held at all.

## Consequences

Rules 4 and 5 apply to a unit only where its pin holds what the unit moves.
`hold` writes `hold = "all"` for a compatible move that failed its gates.
