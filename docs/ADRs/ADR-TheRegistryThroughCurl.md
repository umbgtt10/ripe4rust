# ADR-TheRegistryThroughCurl

- **Status:** Accepted
- **Date:** 2026-11-26

## Context

The registry is read over HTTPS: the sparse index for every crate in the lock,
the API for a direct dependency's repository (ADR-TheIndexForVersionsAndDates).
A Rust HTTP client needs a TLS stack. rustls, the one written in Rust, pulls in
`ring`, whose ISC licence the family's `deny.toml` refuses; that file is the
same in every repository, so an exception for one crate is not an option.
native-tls links the system's TLS -- OpenSSL on Linux, with its headers at build
time -- a different stack on each of the three systems CI runs. Every other
input ripe4rust reads already comes from a process: cargo, cargo-deny, rustc.

## Decision

HTTP goes through a `curl` process, behind the cli's `Http` seam, so the
adapters that use it are tested on recorded output like the rest:

```
curl --silent --show-error --location --max-time 60
     --user-agent "cargo-ripe4rust/<version> (+https://github.com/umbgtt10/ripe4rust)"
     --write-out "\n%{http_code}" <url>
```

The body comes on stdout, and the status on a line of its own after it. A 200
is the answer; a 404 is an answer too, that the registry holds nothing for the
crate, kept in the day's cache as an empty one; any other status, or curl
failing -- a name it cannot resolve, a connection refused, a time-out --, is
`registry-unreachable`, with what curl said. curl not on the PATH is
`curl-missing`, whose remedy says where curl comes from: it ships with
Windows 10 and later, macOS and most Linux distributions. An offline plan reads
the cache alone and runs no curl at all.

## Rejected alternatives

- **An HTTP crate over rustls.** `ring`'s licence is refused, and the refusal is
  the family's, not this repository's to lift.
- **An HTTP crate over native-tls.** System libraries at build time, and three
  TLS stacks behind one adapter, for a few hundred requests a day.
- **Asking cargo.** No cargo command prints a crate's index file, with every
  release's `yanked`, `rust_version` and `pubtime`; `cargo info` writes text
  for a person, about one version.

## Consequences

The dependency tree holds no HTTP or TLS crate, so the licence check stays
as it is everywhere. A run spends one process on each index file and API answer
not yet cached for the day, so a large lock's first plan of a day takes a while
and every later one none. ripe4rust trusts curl's TLS and the system's
certificates, as cargo trusts its own; the cache is written by ripe4rust, not by
curl.
