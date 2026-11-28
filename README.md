# ripe4rust

**Is this upgrade ripe?**

`cargo ripe4rust` will decide, for every upgrade a Rust workspace could take,
one of three things:

| outcome | meaning |
|---|---|
| **ripe** | safe to take without asking: apply it, run the gates, commit if green |
| **ask** | worth taking, but a person decides: presented with a suggestion and the evidence |
| **ripening** | not yet: listed with its reason and what will release it |

It decides; it never applies. Never upgrading is wrong, and upgrading
everything the moment it is published is not right either -- ripe4rust makes
the judgement in between explicit, rule by rule, so it can be automated where
it is safe and asked where it is not.

## Install

```sh
cargo install cargo-ripe4rust
cargo install --locked cargo-deny
```

It reads the advisories through `cargo deny`, and the crates.io registry and
the Rust distribution's list of releases through `curl`: both are needed on
`PATH`. In a workspace, `cargo ripe4rust guide` prints the round.

## Status

**0.1.4, the applied rounds' fixes.** `cargo ripe4rust plan [--format table|json] [--date
YYYY-MM-DD] [--offline | --refresh]` reads a real workspace -- cargo, cargo-deny, rustc,
the crates.io registry and the Rust distribution's list of releases, through
`curl` and cached for the day, `--refresh` fetching them again for a release
published since the day's first plan -- and prints the plan. `cargo ripe4rust check
<unit> --plan <file>` reads the plan back and the workspace again, and says
whether the unit applied is as planned -- exit 0, or 1 with every difference
and what to do -- or the plan is stale, 3. `cargo ripe4rust hold <unit> --plan
<file> --because <text>` prints the pin that holds a unit whose gates went red,
or that a person said to wait on, and where it goes; ripe4rust still writes no
file. `cargo ripe4rust guide` prints the round, every command, every diagnostic
code with its remedy and the documents' fields. The sources are parsed with
`syn`, so a breaking upgrade's footprint -- every line naming the crate, the
work list of its migration -- and whether the crate is in a published member's
public API are known. A `rust-toolchain.toml` pinning an older stable release
is judged as a unit of its own, with the dependency versions the newer one
would release, and a git dependency's source moving is a unit of its own,
always asked. A first dry round across real workspaces, plans only, found
what 0.1.1 fixes, and the first round that applied a plan what 0.1.2 does:
`check` holds a unit to the moves it owns where the units share a crate, and
a breaking unit whose crate a dependent's requirement holds back ripens until
the dependent moves, clearing no advisory meanwhile. The rounds after it found
what 0.1.3 fixes: `plan --refresh` fetches the day's answers again, for a
release published since the day's first plan, and `check` names every
dependency re-pointed between versions the lock already held, told and never
a difference. 0.1.4 lets a pin with a review date hold until that date, however
far behind its crate falls. Tuning the thresholds and the grouping is next. The
design is in [`docs/DESIGN.md`](docs/DESIGN.md).

## License

MIT -- see [LICENSE](LICENSE).
