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

**In development.** The core decides -- the units, the version choice, the
rules, the steps and the questions, tested against fakes of every input -- and
the renderers print a plan; the command still exits 2 until it reads a real
workspace. Version 0.0.1, on crates.io, holds the vocabulary only. The design
is in [`docs/DESIGN.md`](docs/DESIGN.md).

## License

MIT -- see [LICENSE](LICENSE).
