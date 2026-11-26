# ADR-AFailureBecomesAPin

- **Status:** Accepted
- **Date:** 2026-11-26

## Context

A unit whose gates go red is reverted. The next round, run by an agent that
knows nothing of the last, would plan the same unit, apply it, watch it fail
the same way, and revert it again -- round after round. A person's "not now"
on an asked unit has the same problem: nothing keeps it.

## Decision

A round remembers through the workspace's own manifest, with a pin.
`cargo ripe4rust hold <unit> --plan <file> --because <text> [--review <date>]`
prints the pin that holds the unit, and the section and line it belongs at:

- the reason is the text given: the failing gate and its first error, or the
  person's words;
- the review date is the one given, or `hold-days` after the plan's date;
- a compatible move is held with `hold = "all"`, since a pin otherwise holds
  only a move to a new line.

The agent adds the pin and commits it, like any change. ripe4rust still edits
no manifest. A pin made this way is an ordinary pin: its review date brings it
back as "revisit the pin", so a hold is never permanent.

## Rejected alternatives

- **A state file of ripe4rust's own.** One more file to keep, outside the
  place every other tool's configuration lives, and invisible in a review of
  the manifest.
- **Write the pin itself.** The one write would make ripe4rust an updater of
  manifests, the line the design does not cross.

## Consequences

Pins carry `hold`, defaulting to the move to a new line. An urgent unit is
never held: holding it would keep what it clears, so when it fails it becomes
an urgent ask instead.
