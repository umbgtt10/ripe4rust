// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::round::guiding::command_guide::CommandGuide;
use crate::planning::round::guiding::exit_guide::ExitGuide;
use crate::planning::round::guiding::option_guide::OptionGuide;
use alloc::string::String;
use alloc::vec::Vec;

const FORMAT: (&str, &str) = (
    "--format table|json",
    "a table and text for a person, or one JSON document for an agent, on stdout, on success and on failure alike",
);
const UNIT: (&str, &str) = (
    "<unit>",
    "a unit by its `id` in the plan, or one of its moves by an `id` of its `steps.split`, `<unit>/<crate>`",
);
const PLAN: (&str, &str) = (
    "--plan <file>",
    "the plan `plan --format json` wrote, named from the directory the command runs in or by an absolute path",
);

const COMMANDS: [Entry; 5] = [
    Entry {
        usage: "cargo ripe4rust [plan] [--format table|json] [--date YYYY-MM-DD] [--offline]",
        does: "decides, for every upgrade the workspace could take, whether it is ripe, worth asking about, or not yet, and prints the plan; it never applies",
        options: &[
            FORMAT,
            (
                "--date YYYY-MM-DD",
                "the day that stands for today, in the rules and as the day of the registry's cache, so a plan is reproducible",
            ),
            (
                "--offline",
                "read the registry's cache alone, the latest day on or before the plan's date, and keep cargo and cargo-deny off the network",
            ),
        ],
        exits: &[
            (0, "a plan was produced, whatever it says"),
            (2, "no plan could be produced: carry out each error's `do`"),
        ],
    },
    Entry {
        usage: "cargo ripe4rust check <unit> --plan <file> [--format table|json]",
        does: "compares the workspace with what the unit should have done, and sorts every difference from the workspace the plan found into the units that explain it",
        options: &[UNIT, PLAN, FORMAT],
        exits: &[
            (
                0,
                "as planned: the units applied and those still to go are listed",
            ),
            (
                1,
                "not as planned: every difference is listed with its `do` -- carry out `revert`, and the unit becomes ask, the differences attached",
            ),
            (2, "could not check: carry out each error's `do`"),
            (
                3,
                "the plan is stale: plan again, and go on with the new plan",
            ),
        ],
    },
    Entry {
        usage: "cargo ripe4rust hold <unit> --plan <file> --because <text> [--review YYYY-MM-DD] [--format table|json]",
        does: "prints the pin that holds the unit, and the section and line it belongs at; ripe4rust writes no file, the agent adds the pin and commits it",
        options: &[
            UNIT,
            PLAN,
            (
                "--because <text>",
                "the pin's reason: the failing gate and its first error, or the person's words",
            ),
            (
                "--review YYYY-MM-DD",
                "the pin's review date; without it, `hold-days` after the plan's date",
            ),
            FORMAT,
        ],
        exits: &[
            (0, "the pin is printed"),
            (
                2,
                "it could not be: carry out each error's `do`; an urgent unit is never held, it becomes an urgent ask",
            ),
            (
                3,
                "the plan is stale: plan again, and go on with the new plan",
            ),
        ],
    },
    Entry {
        usage: "cargo ripe4rust guide [--format table|json]",
        does: "prints the round, every command, every diagnostic code with its remedy, and the fields of the documents, versioned with the binary",
        options: &[FORMAT],
        exits: &[(0, "the guide is printed")],
    },
    Entry {
        usage: "cargo ripe4rust --version",
        does: "prints `cargo-ripe4rust <version>`, the release of the binary, and nothing else; it reads nothing",
        options: &[("-V", "the same, shorter")],
        exits: &[(0, "the release is printed")],
    },
];

struct Entry {
    pub usage: &'static str,
    pub does: &'static str,
    pub options: &'static [(&'static str, &'static str)],
    pub exits: &'static [(u8, &'static str)],
}

/// Every command of ripe4rust with what it takes and the codes it exits
/// with; a command line refused is `usage-invalid`, exit 2, whatever the
/// command.
pub struct CommandsGuide;

impl CommandsGuide {
    pub fn commands(&self) -> Vec<CommandGuide> {
        COMMANDS
            .iter()
            .map(|entry| CommandGuide {
                usage: String::from(entry.usage),
                does: String::from(entry.does),
                options: entry
                    .options
                    .iter()
                    .map(|(option, means)| OptionGuide {
                        option: String::from(*option),
                        means: String::from(*means),
                    })
                    .collect(),
                exits: entry
                    .exits
                    .iter()
                    .map(|(code, means)| ExitGuide {
                        code: *code,
                        means: String::from(*means),
                    })
                    .collect(),
            })
            .collect()
    }
}
