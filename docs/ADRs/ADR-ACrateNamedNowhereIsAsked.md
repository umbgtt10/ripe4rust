# ADR-ACrateNamedNowhereIsAsked

- **Status:** Accepted
- **Date:** 2026-11-26

## Context

Rule 11 takes a breaking upgrade without asking when its footprint is under
`small-footprint`. In the first dry round two breaking units had a footprint
of 0 in a parse that read every file: one crate was reached only through
another crate's re-export, the other was a dev-dependency nothing used. Each
read "footprint 0 lines in 0 files", and in a gated workspace rule 11 would
have called it ripe -- the smallest footprint there is.

A footprint of 0 is not a small migration. It says the crate is named
nowhere: possibly an unused dependency, possibly one needed for what the
code never names -- a feature it turns on in another crate, a side effect of
linking it, a type reached through a re-export.

## Decision

A breaking unit whose crate a complete parse finds named nowhere -- a
footprint of zero, with no include the parse could not follow -- is never
ripe by its footprint: rule 11 leaves it, and it is asked.

- Its verdict is "migrate now": there is no work list, so a migration costs
  a requirement line and a build.
- Its reasons say "named nowhere in the sources: possibly an unused
  dependency", whatever rule decided it.
- Its question offers `remove` beside its other answers: remove the crate
  from the manifests that declare it, run the gates, commit if green, and put
  the question again if red.

"Migrate now" stays the recommendation: removing a dependency is a change of
the workspace's design, not an upgrade, and the gates do not prove a feature
or a side effect unneeded.

## Rejected alternatives

- **Ripe, as rule 11 had it.** It upgrades a crate that may be dead weight,
  and says nothing of it.
- **Recommend removing it.** The parse cannot see a feature or a side
  effect, so it cannot tell an unused crate from one used without its name.
- **Hold it.** Nothing about the upgrade itself is unripe.

## Consequences

A person sees each crate the sources never name, once, with the choice to
remove it. A footprint that is a floor -- code included the parse could not
follow -- is not "named nowhere": it waits, its footprint not known.
