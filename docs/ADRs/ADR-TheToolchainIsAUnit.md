# ADR-TheToolchainIsAUnit

- **Status:** Accepted
- **Date:** 2026-11-26

## Context

The workspaces this tool serves pin their compiler in `rust-toolchain.toml` and
move it by hand, release by release. It is the same judgement as a
dependency's -- is the new release ripe, does it reach firmware, is something
frozen -- and it is also what releases every dependency version the current
toolchain cannot build.

## Decision

The toolchain is a third kind of unit. It moves `rust-toolchain.toml`'s channel
to the newest stable release, and nothing else: a crate's `rust-version` moves
only with a crate that needs it. Its line is one minor (`1.100.x`); its youth
is judged by `toolchain-young-days`, by default 0, as the family takes a
stable release on its day. A pin named `rust` holds it. It reaches every
target, so in a workspace that names a target needing more than the gates it is
always asked. Its evidence lists the dependency versions it would release.

Its releases and their dates come from the Rust distribution's channel
manifests, read from the one file that lists them all,
`https://static.rust-lang.org/manifests.txt`: every manifest the distribution
published, one a line under the day it went out, so every stable release with
its day in one request, cached for the day like the registry's answers. A
release listed on several days is dated by the last listing, the earlier ones
being promotions ahead of its day. The list is read only where the channel
pins a release; unreachable with nothing cached, it stops the plan
(`toolchain-unreachable`), as the registry does.

A workspace that pins no release -- no `rust-toolchain.toml`, or a channel of
`stable`, a nightly, or anything but a full stable release such as `1.99.1` --
has no toolchain unit; the plan says so with the
notice `toolchain-unpinned`, and the toolchain in use, for choosing
candidates, is what `rustc --version` reports.

## Rejected alternatives

- **Leave the toolchain out.** The judgement would still be made, by hand, and
  the held-back versions would be reported with nothing to release them.
- **Move `rust-version` with it.** `rust-version` is a promise to a crate's
  users; raising it is a change of its own, made when a dependency needs it.
- **Read the stable channel's manifest.** It names one release, the newest,
  in about 900 KB, and the youth and the drift of a line need every release's
  day. The server honours a ranged request for a manifest's head, which gives
  its date, but `channel-rust-stable.toml` names its release deep in the file,
  and a manifest per release, `channel-rust-1.99.0.toml`, is a request per
  release, each found by trying names. The list is about 330 KB, once a day.

## Consequences

The plan can say "ripe" for a compiler release, and a caller applies it like
any other unit: change the channel, run the gates, commit or revert. A fourth
input port, the toolchain's releases, joins the core.
