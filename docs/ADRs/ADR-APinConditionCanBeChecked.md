# ADR-APinConditionCanBeChecked

- **Status:** Accepted
- **Date:** 2026-11-26

## Context

A pin holds a dependency back until something happens. The first design took
that something in words -- "the rig's CLI moves to 0.32" -- which is honest but
cannot be checked, so it fell back to a review date.

## Decision

A release condition takes one of two forms:

- **checkable:** `until = { crate = "probe-rs-tools", version = ">=0.32" }`,
  met when the index holds a release of that crate matching the requirement;
- **in words:** `until = "..."`, never evaluated, so it must come with a
  `review` date.

A pin needs a reason, and a review date or a checkable condition. Anything
less is an error, reported before any rule runs. A met condition or a reached
review date turns the pin's unit into **ask**, "revisit the pin".

## Rejected alternatives

- **A grammar for conditions.** Versions of other crates are what pins in
  these workspaces wait for; anything richer can be written in words with a
  date.

## Consequences

A pin waiting on another crate's release ends on its own, reported the day the
release appears.
