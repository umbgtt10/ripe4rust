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

## Status

**In development.** `cargo ripe4rust plan [--format table|json] [--date
YYYY-MM-DD] [--offline]` reads a real workspace -- cargo, cargo-deny, rustc,
and the crates.io registry through `curl`, cached for the day -- and prints the
plan. `cargo ripe4rust check <unit> --plan <file>` reads the plan back and the
workspace again, and says whether the unit applied is as planned -- exit 0, or
1 with every difference and what to do -- or the plan is stale, 3. `cargo
ripe4rust hold <unit> --plan <file> --because <text>` prints the pin that holds
a unit whose gates went red, or that a person said to wait on, and where it
goes; ripe4rust still writes no file. `cargo ripe4rust guide` prints the round,
every command, every diagnostic code with its remedy and the documents' fields.
The parse of the sources and the toolchain's releases are still to come: until
then the plan reads them conservatively and says so. Version 0.0.1, on crates.io, holds the vocabulary only. The
design is in [`docs/DESIGN.md`](docs/DESIGN.md).

## License

MIT -- see [LICENSE](LICENSE).
