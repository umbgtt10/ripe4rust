# ADR-AGitSourceIsAsked

- **Status:** Accepted
- **Date:** 2026-11-26

## Context

The first design left git dependencies out of scope: their moves were "the
workspace's own business". But `cargo update --dry-run` moves them with
everything else, and it prints them in a shape of their own:

```
Updating etheram-core v0.1.0 (https://github.com/<owner>/etheram-core?branch=main#99217a13) -> #8f433341
```

The strict reader did not know the line, answered `cargo-update-unrecognised`,
and four of the first dry round's workspaces got no plan at all. Recorded
from cargo, the shape holds more than one case: `Downgrading` where the
version went down, `Adding` and `Removing <crate> v<version>
(<source>#<rev>)` where a crate joins or leaves the source, and never the
version a moved crate takes, even when it changed.

## Decision

A git source moving is a unit of a fourth kind, **git**:

- **One unit a source.** Every crate from one git source -- one URL and its
  branch, tag or rev spec -- moves as one unit, as cargo moves a source as a
  whole: one `cargo update -p <crate>` on any of them moves them all. The unit
  goes by its first crate in the lock, `git:<crate>`, and is shown from rev
  to rev, the short revs cargo prints.
- **What it pulls in** is what `cargo update --dry-run -p <crate>@<version>`
  moves in the registry, claimed in plan order with the compatible units', as
  their transitive moves are. It never splits: what it pulls in is what the
  new rev needs.
- **Always asked.** A git source makes no version promise -- a branch can
  move anywhere, its versions are not published, and cargo does not even say
  which version a moved crate takes -- so whether to take a new rev is a
  person's call: rule 13 asks it, "git source: no version promise, a person
  decides". It is never ripe. It is urgent only where its registry moves
  clear an advisory or a yanked release, which rule 3 asks, urgent; pins, a
  freeze and rule 7 come before rule 13 as for any unit.
- **check** reads the lock's crates of git sources with their source and
  short rev: the unit is applied where its source is at the rev it takes.
  A source at a rev no git unit takes it to is a move no unit makes.
- **hold** pins its first crate with `hold = "all"`, and a pin on any crate of
  a source holds the git unit whole, whatever its `hold`: a source has no line
  to hold.

## Rejected alternatives

- **Pass git lines over.** The reader would no longer be strict, and the
  move would stay in the lock's next update without a word.
- **Leave git moves to the rest.** The rest applies `--precise` versions,
  which a git source does not have, and is ripe where a git move is not.
- **Ripe where the version is unchanged.** cargo does not print the version
  a moved crate takes, and an unchanged version promises nothing on a branch.
- **A unit per crate.** cargo cannot move one crate of a source alone.

## Consequences

Workspaces depending on a git branch get a plan, with the source's move
asked. The plan document gains `workspace.git` and a unit's `source`, and a
git unit's `from` and `to` are revs, not versions; older plans, holding
neither, still read.
