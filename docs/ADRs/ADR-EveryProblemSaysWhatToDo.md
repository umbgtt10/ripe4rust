# ADR-EveryProblemSaysWhatToDo

- **Status:** Accepted
- **Date:** 2026-11-26

## Context

An agent with no context cannot interpret an error it has never seen, and it
cannot notice what is not said at all. A misspelt threshold silently keeps its
default; a pin on a crate long gone silently does nothing; a source file the
parse skipped silently lowers a footprint. Each makes a plan wrong with no
trace.

## Decision

Every problem ripe4rust meets, fatal or not, is a diagnostic:

- **severity** -- `error` stops the command with exit 2; `warning` and
  `notice` never change the exit code;
- **code** -- stable, one per kind of problem, each listed by `guide` with its
  remedy;
- **where** -- a `file:line` where there is one;
- **what** -- what happened;
- **do** -- the next step, written for an agent with no context;
- **needs_person** -- true when only the workspace's owner can supply the fix,
  such as a pin's reason; the agent then asks rather than invents.

With `--format json` the diagnostics are part of the one document every
command prints, on failure too; in text they read
`error[<code>] <where>: <what>. Do: <do>`. A key in the policy the tool does
not know is an error. ripe4rust does not panic at its caller: a failure of its
own is the diagnostic `internal`, with what it was doing and where to report
it.

## Rejected alternatives

- **Messages on standard error only.** Readable by a person, but an agent
  would have to parse prose, and a change of wording would break it.
- **Ignore unknown policy keys, for forward compatibility.** The cost of a
  silently ignored typo is a wrong plan; a newer key in an older tool is an
  error that says to upgrade the tool.

## Consequences

Adding a kind of problem means adding a code, its remedy in `guide`, and its
test. The design's table of codes is the minimum the first release carries.
