# ADR-StalenessNeverFailsAGate

- **Status:** Accepted
- **Date:** 2026-11-26

## Context

The first design left open a `--fail-on urgent` flag, so a stage could fail
on advisories through ripe4rust, instead of or beside cargo-deny.

## Decision

No plan fails anything: the command exits 0 whenever it produced a plan,
whatever the plan says, and 2 only when it could not produce one. cargo-deny
stays the gate on advisories, yanked releases, licences and sources.

## Rejected alternatives

- **`--fail-on urgent`.** Two tools would answer the same question, and differ
  -- cargo-deny reads its own ignore list -- so a stage could be red in one
  and green in the other over the same advisory.

## Consequences

One tool, one question. A workspace being behind stays information for an
upgrade round, never a defect in the change being gated.
