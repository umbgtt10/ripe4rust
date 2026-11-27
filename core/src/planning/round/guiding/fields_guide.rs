// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::round::guiding::field_guide::FieldGuide;
use alloc::string::String;
use alloc::vec::Vec;

const FIELDS: [(&str, &str, &str); 124] = [
    (
        "every",
        "schema",
        "the shape of the document: 1; it moves when the shape does",
    ),
    ("every", "command", "the command that printed it"),
    (
        "every",
        "diagnostics",
        "every problem met on the way; a document for a failure carries `schema`, `command` and `diagnostics` alone",
    ),
    (
        "every",
        "diagnostics[].severity",
        "`error` stops the command, exit 2 -- 3 for `plan-stale` --; `warning` and `notice` never change the exit code",
    ),
    (
        "every",
        "diagnostics[].code",
        "the kind of problem, stable; `guide` lists every one with its remedy",
    ),
    (
        "every",
        "diagnostics[].where",
        "the `file:line` it is at, where there is one",
    ),
    ("every", "diagnostics[].what", "what happened"),
    (
        "every",
        "diagnostics[].do",
        "the next step, written for an agent with no context",
    ),
    (
        "every",
        "diagnostics[].needs_person",
        "whether only the workspace's owner can supply the fix: the agent asks, it does not invent",
    ),
    (
        "plan",
        "date",
        "the day the plan is for: `--date`, or today",
    ),
    (
        "plan",
        "registry_read",
        "the day the registry's answers are from",
    ),
    (
        "plan",
        "workspace",
        "the workspace the plan was made from, which `check` and `hold` compare the workspace with",
    ),
    (
        "plan",
        "workspace.fingerprint",
        "FNV-1a over `Cargo.lock`, every manifest and `rust-toolchain.toml`: whether anything changed",
    ),
    (
        "plan",
        "workspace.lock",
        "the lock's crates.io packages, each `<crate> <version>`",
    ),
    (
        "plan",
        "workspace.git",
        "the lock's crates of git sources, each `<crate> <source>#<rev>`, the rev short as cargo prints it",
    ),
    (
        "plan",
        "workspace.lines",
        "every line a unit's `apply` can edit, as the plan found it",
    ),
    ("plan", "workspace.lines[].where", "the line, `file:line`"),
    (
        "plan",
        "workspace.lines[].key",
        "the crate a requirement is for, `rust-version`, or `channel`",
    ),
    ("plan", "workspace.lines[].value", "what is written there"),
    (
        "plan",
        "gates",
        "the commands that validate a change, in order; each must exit 0",
    ),
    (
        "plan",
        "shared",
        "the crates the units share: their moves, each unit's computed alone, do not add up to the whole workspace's, so they depend on the order the units are applied in; `check` reports such a move a unit does not own, never as a difference",
    ),
    (
        "plan",
        "units",
        "one row per unit, in plan order: by crate, a crate's compatible unit before its breaking one, the toolchain as `rust`, then the rest, then the advisory rows",
    ),
    (
        "plan",
        "units[].id",
        "`<kind>:<crate>`, `rest`, or `advisory:<crate>@<version>` -- one row for every advisory and yanking against that version no unit clears; what `check` and `hold` name it by",
    ),
    (
        "plan",
        "units[].kind",
        "`compatible`, `breaking`, `git`, `toolchain` or `advisory`",
    ),
    (
        "plan",
        "units[].crate",
        "the crate it moves, a group's first member; null for the rest",
    ),
    (
        "plan",
        "units[].members",
        "the crates it moves directly: a group's members, `rust` for the toolchain",
    ),
    (
        "plan",
        "units[].from",
        "the version it leaves; a git unit's, the short rev its source leaves",
    ),
    (
        "plan",
        "units[].to",
        "the version it takes its own crate to, a git unit's the short rev it takes its source to; null for the rest, for a compatible unit whose own move a pin holds -- `evidence.held` names that move, and the table writes `held` --, and for one whose crate does not move, only what it pulls in -- the table writes `stays`",
    ),
    (
        "plan",
        "units[].source",
        "a git unit's source, its URL and its branch, tag or rev spec as cargo writes it, moved as a whole; null for every other kind",
    ),
    (
        "plan",
        "units[].moves",
        "the lock moves applying it makes; a breaking unit's tail is known only once applied",
    ),
    ("plan", "units[].moves[].crate", "the crate moved"),
    (
        "plan",
        "units[].moves[].from",
        "the version it leaves; null for a crate added",
    ),
    (
        "plan",
        "units[].moves[].to",
        "the version it takes; null for a crate removed",
    ),
    ("plan", "units[].outcome", "`ripe`, `ask` or `ripening`"),
    (
        "plan",
        "units[].urgent",
        "whether it clears an advisory or a yanked release: it goes first, and is never held",
    ),
    (
        "plan",
        "units[].rule",
        "the rule that decided it, 1 to 12; null for an advisory row",
    ),
    (
        "plan",
        "units[].suggestion",
        "what the rule suggests, by its label; null where it suggests nothing",
    ),
    (
        "plan",
        "units[].reasons",
        "the short reasons the table shows",
    ),
    ("plan", "units[].summary", "its commit's subject"),
    (
        "plan",
        "units[].steps",
        "what to do with it; null for an advisory row",
    ),
    (
        "plan",
        "units[].steps.apply",
        "the steps that apply it, in order",
    ),
    ("plan", "units[].steps.apply[].run", "a command to run"),
    (
        "plan",
        "units[].steps.apply[].edit",
        "the `file:line` to write at",
    ),
    (
        "plan",
        "units[].steps.apply[].key",
        "the key written there: the crate, `rust-version` or `channel`",
    ),
    ("plan", "units[].steps.apply[].value", "the value to write"),
    (
        "plan",
        "units[].steps.check",
        "the `check` to run once it is applied",
    ),
    (
        "plan",
        "units[].steps.gates",
        "the gates to run once `check` exits 0",
    ),
    ("plan", "units[].steps.revert", "how to take it back"),
    (
        "plan",
        "units[].steps.revert.restore",
        "the files to restore",
    ),
    (
        "plan",
        "units[].steps.revert.run",
        "the command that restores them",
    ),
    (
        "plan",
        "units[].steps.split",
        "its moves one at a time, for bisecting a compatible unit of two moves or more",
    ),
    (
        "plan",
        "units[].steps.split[].id",
        "the move's id, `<unit>/<crate>`, which `check` and `hold` take",
    ),
    (
        "plan",
        "units[].steps.split[].apply",
        "the steps that apply the move alone",
    ),
    (
        "plan",
        "units[].steps.split[].apply[].run",
        "a command to run",
    ),
    (
        "plan",
        "units[].question",
        "for a unit that asks, the question to put to the person; null otherwise",
    ),
    ("plan", "units[].question.title", "the question"),
    ("plan", "units[].question.why", "why it is asked"),
    (
        "plan",
        "units[].question.recommendation",
        "the answer recommended",
    ),
    (
        "plan",
        "units[].question.options",
        "the answers the person can give",
    ),
    (
        "plan",
        "units[].question.options[].answer",
        "the answer, by its label",
    ),
    (
        "plan",
        "units[].question.options[].means",
        "what the answer means",
    ),
    (
        "plan",
        "units[].question.options[].then",
        "the step that carries the answer out",
    ),
    ("plan", "units[].evidence", "what the decision read"),
    (
        "plan",
        "units[].evidence.line_in_use",
        "the line in use; null where the index knows none",
    ),
    (
        "plan",
        "units[].evidence.line_in_use.line",
        "the line, `2.x`, or `0.13.x`",
    ),
    (
        "plan",
        "units[].evidence.line_in_use.first_release",
        "the day its first release came out",
    ),
    (
        "plan",
        "units[].evidence.line_in_use.last_release",
        "the day its last release came out",
    ),
    (
        "plan",
        "units[].evidence.line_in_use.releases",
        "how many releases it has",
    ),
    (
        "plan",
        "units[].evidence.rust_version",
        "the `rust-version` the version taken needs",
    ),
    (
        "plan",
        "units[].evidence.advisories",
        "the advisories and yanked releases it clears",
    ),
    (
        "plan",
        "units[].evidence.yanked_in_use",
        "whether the version in use is yanked",
    ),
    (
        "plan",
        "units[].evidence.footprint",
        "every `file:line` naming the crate, read from the sources with `syn`: a migration's work list. What a macro expands to is not seen, nor a function a glob import alone brings in; where a file could not be parsed, a `source-unparsed` warning says so, and the list is a floor",
    ),
    (
        "plan",
        "units[].evidence.held_back",
        "the versions the toolchain unit would release",
    ),
    (
        "plan",
        "units[].evidence.held",
        "the moves a pin holds out of the unit",
    ),
    ("plan", "units[].evidence.held[].crate", "the crate held"),
    (
        "plan",
        "units[].evidence.held[].from",
        "the version it stays at",
    ),
    (
        "plan",
        "units[].evidence.held[].to",
        "the version it would take",
    ),
    (
        "plan",
        "units[].evidence.downgrades",
        "the moves to a lower version the unit makes, each named among its reasons; one leaving a release that is not yanked makes the unit ask",
    ),
    (
        "plan",
        "units[].evidence.downgrades[].crate",
        "the crate moved back",
    ),
    (
        "plan",
        "units[].evidence.downgrades[].from",
        "the version it leaves",
    ),
    (
        "plan",
        "units[].evidence.downgrades[].to",
        "the lower version it takes",
    ),
    (
        "plan",
        "units[].evidence.unfollowed_includes",
        "a breaking unit's: every `include!` the parse could not follow -- its path no literal, or generated code as `include_proto!` writes it -- in the members using the crate, as `file:line`; where there is one, the footprint is a floor, never small",
    ),
    ("check", "unit", "the unit, or the move of a split, checked"),
    (
        "check",
        "as_planned",
        "whether it shows exactly what it should: exit 0, or 1",
    ),
    (
        "check",
        "transitive",
        "a breaking unit's tail: the moves its edit made beyond its own, which the plan could not know",
    ),
    ("check", "transitive[].crate", "the crate moved"),
    (
        "check",
        "transitive[].from",
        "the version it left; null for a crate added",
    ),
    (
        "check",
        "transitive[].to",
        "the version it took; null for a crate removed",
    ),
    (
        "check",
        "differences",
        "how the unit is not as planned: revert it, and it becomes ask with them attached",
    ),
    (
        "check",
        "differences[].where",
        "`Cargo.lock`, or the `file:line` of an edit",
    ),
    ("check", "differences[].what", "what differs"),
    ("check", "differences[].do", "what to do about it"),
    (
        "check",
        "shared",
        "the moves of crates the units share that came out otherwise than the plan computed them, each unit alone: no difference, nothing to do",
    ),
    (
        "check",
        "applied",
        "the units applied, and the moves of a unit's split applied where the unit is only partly applied, in plan order",
    ),
    (
        "check",
        "to_go",
        "the units still to go, and the moves of a partly applied unit's split, in plan order",
    ),
    ("hold", "unit", "the unit, or the move of a split, held"),
    ("hold", "section", "the table the pins belong in"),
    (
        "hold",
        "where",
        "the `file:line` of that table; null where it is to be added at the end of `Cargo.toml`",
    ),
    ("hold", "pins", "the pins to write, one line each"),
    ("hold", "pins[].crate", "the crate the pin holds"),
    ("hold", "pins[].line", "the line to write under the table"),
    (
        "hold",
        "pins[].replaces",
        "the `file:line` of the pin on the crate this line takes the place of; null where it has none",
    ),
    ("hold", "do", "what to do with the pins"),
    (
        "guide",
        "version",
        "the release of ripe4rust the guide is of",
    ),
    (
        "guide",
        "round",
        "the round, a step a line, a sub-step indented under its step",
    ),
    ("guide", "commands", "every command"),
    ("guide", "commands[].usage", "its command line"),
    ("guide", "commands[].does", "what it does"),
    ("guide", "commands[].options", "what it takes"),
    (
        "guide",
        "commands[].options[].option",
        "the option, or the argument, as the usage writes it",
    ),
    ("guide", "commands[].options[].means", "what it means"),
    ("guide", "commands[].exits", "the codes it exits with"),
    ("guide", "commands[].exits[].code", "the code"),
    ("guide", "commands[].exits[].means", "what it means"),
    (
        "guide",
        "codes",
        "every diagnostic code, from the catalogue the diagnostics come from",
    ),
    ("guide", "codes[].code", "the code"),
    ("guide", "codes[].severity", "its severity"),
    (
        "guide",
        "codes[].needs_person",
        "whether only the workspace's owner can supply its fix",
    ),
    ("guide", "codes[].means", "what it means"),
    ("guide", "codes[].do", "its remedy"),
    (
        "guide",
        "fields",
        "every field of the documents of schema 1",
    ),
    (
        "guide",
        "fields[].document",
        "the document it is in: `plan`, `check`, `hold`, `guide`, or `every` for those all carry",
    ),
    (
        "guide",
        "fields[].field",
        "its path, an array's items as `[]`",
    ),
    ("guide", "fields[].means", "what it holds"),
];

/// Every field of the documents of schema 1, document by document: those
/// every document carries, then `plan`'s, `check`'s, `hold`'s and
/// `guide`'s, each in the order the document writes them.
pub struct FieldsGuide;

impl FieldsGuide {
    pub fn fields(&self) -> Vec<FieldGuide> {
        FIELDS
            .iter()
            .map(|(document, field, means)| FieldGuide {
                document: String::from(*document),
                field: String::from(*field),
                means: String::from(*means),
            })
            .collect()
    }
}
