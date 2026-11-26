# ADR-TheAgentNeedsNoContext

- **Status:** Accepted
- **Date:** 2026-11-26

## Context

ripe4rust's caller is meant to be an agent that knows nothing of the
workspace, of the tool or of earlier rounds, and that runs an upgrade round
with no person watching. The design as first settled decided well and showed
why, but left the agent to work out the rest: the procedure lived in a design
document it would never read, a unit said what to decide but not what to do,
nothing confirmed that an applied unit did what the plan said, nothing noticed
a workspace changed under a plan, problems had no prescribed form, a failed
unit was forgotten by the next round, and "ask" had no defined question or
follow-up. The owner's requirement: a full automation tool that gives an agent
with no previous context proper feedback whenever anything unexpected happens,
the only exception being when the agent needs the owner's approval.

## Decision

ripe4rust keeps deciding and never applying, and everything it prints is
written for that agent:

- every unit carries its steps -- apply, check, the gates, revert, and for a
  compatible unit the split that bisects it;
- `check` compares the workspace with what a unit should have done, against
  the workspace the plan was made from (ADR-ACheckAfterEveryUnit);
- every problem is a diagnostic saying where it is and what to do, and
  whether only a person can do it (ADR-EveryProblemSaysWhatToDo);
- `hold` prints the pin that makes a round remember a failure or a person's
  "wait" (ADR-AFailureBecomesAPin);
- an **ask** unit carries its question, ready to present, and for each answer
  the step that carries it out;
- `guide` prints the round, every diagnostic code and its remedy, versioned
  with the binary;
- with `--format json`, every command prints one document, on success and on
  failure alike.

The agent stops for a person only at an **ask** unit or a diagnostic marked as
needing one.

## Rejected alternatives

- **ripe4rust runs the round itself** -- applies, runs the declared gates,
  commits or reverts, bisects, and stops only at ask. Literally fuller
  automation, but the tool would then have to know each workspace's commit
  conventions, its pre-commit checks and its push rules, which the agent
  already keeps; and a tool that writes, commits and runs arbitrary gates is
  no longer deterministic or testable against fakes. The owner chose the
  contract over the runner.
- **Leave the procedure in the documentation.** An agent with no context reads
  what the tool prints, not what a repository explains.

## Consequences

The plan grows: steps, questions, the workspace's fingerprint and lock. Two
commands join `plan`: `check` and `hold`, and `guide` documents them all. The
core gains the steps, the questions, the diagnostics, `check`'s sorting and
`hold`'s pin, each tested against fakes like the rules. A workspace declares
its gates in the policy; without them nothing is ripe.
