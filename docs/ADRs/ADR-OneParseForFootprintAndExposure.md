# ADR-OneParseForFootprintAndExposure

- **Status:** Accepted
- **Date:** 2026-11-26

## Context

Two rules read the workspace's sources. Rule 11 takes a breaking upgrade
without asking when the code naming the crate is small; rule 8 asks when the
crate's types are in a published crate's public API. The first design left
open whether the footprint should come from a parse or a text search.

## Decision

One `syn` parse of the workspace's sources answers both.

- **Footprint:** the files and lines that name the crate -- paths, `use`
  items, macros, derives and attributes, through renames (`package = ...`,
  `use x as y`) and globs.
- **Exposure:** whether the crate's items reach a published crate's public
  API -- public signatures, public fields, `pub use` re-exports, the crate's
  traits derived or implemented on public types -- counted where the item is
  reachable from the crate's `lib.rs` through `pub mod` declarations or `pub
  use` re-exports, the facade `mod inner; pub use inner::Thing;` included.

The footprint keeps every location as `file:line`: an asked breaking unit's
evidence carries them, the work list of its migration. A source file the parse
cannot read is the warning `source-unparsed`: the footprint may be low.

Where the parse is wrong -- a type alias, a macro expanding to a public item --
the policy overrides it per crate: `exposure = { serde = "public" }`.

## Rejected alternatives

- **A text search for the footprint.** Cheaper, but blind to renames and
  imports, and exposure needs the parse anyway.
- **Exposure declared only.** cargo's `public = true` on a dependency would
  say it, but is unstable; a list kept by hand drifts from the code it
  describes. Declaration stays as the override.

## Consequences

The family's tools already walk `syn` trees; this one reuses the shape. The
parse is the CLI's; the core receives a footprint and an exposure per crate
through a port, faked in tests.
