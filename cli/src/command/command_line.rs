// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::command::command_spec::CommandSpec;
use crate::command::format::Format;
use crate::command::option_reader::OptionReader;
use crate::command::refusal::Refusal;
use crate::command::request::Request;
use ripe4rust_core::diagnostics::diagnostic::Diagnostic;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;

const SUBCOMMAND: &str = "ripe4rust";
const PLAN: &str = "plan";
const LATER: [&str; 2] = ["hold", "guide"];
const COMMANDS: [CommandSpec; 2] = [
    CommandSpec {
        name: PLAN,
        usage: "run `cargo ripe4rust [plan] [--format table|json] [--date YYYY-MM-DD] [--offline]`",
        options: &["--format", "--date", "--offline"],
        unit: false,
        required: &[],
    },
    CommandSpec {
        name: "check",
        usage: "run `cargo ripe4rust check <unit> --plan <file> [--format table|json]`",
        options: &["--format", "--plan"],
        unit: true,
        required: &["--plan"],
    },
];

/// The command line: `cargo ripe4rust [plan] [--format table|json] [--date
/// YYYY-MM-DD] [--offline]` or `cargo ripe4rust check <unit> --plan <file>
/// [--format table|json]`, the subcommand's own name cargo passes first
/// skipped. A command this release does not run yet, or anything it does
/// not know, is refused with a diagnostic, in the format the line asked for.
pub struct CommandLine;

impl CommandLine {
    pub fn read(&self, args: &[String]) -> Result<Request, Refusal> {
        let words = args.iter().map(String::as_str).collect::<Vec<&str>>();
        let words = words.strip_prefix(&[SUBCOMMAND]).unwrap_or(&words);
        let format = Self::asked_format(words);
        let (command, options) = match words.split_first() {
            Some((word, options)) if !word.starts_with('-') => (*word, options),
            _ => (PLAN, words),
        };
        let refused = |diagnostic: Diagnostic| Refusal {
            command: String::from(command),
            format,
            diagnostic,
        };
        if LATER.contains(&command) {
            return Err(refused(DiagnosticCode::CommandUnavailable.diagnose(
                None,
                format!("`{command}` is not in this release of ripe4rust yet"),
                String::from(
                    "use `cargo ripe4rust plan` or `cargo ripe4rust check`, the commands this release runs; `hold` and `guide` come with a later release",
                ),
            )));
        }
        let spec = COMMANDS
            .iter()
            .find(|spec| spec.name == command)
            .ok_or_else(|| {
                refused(
                    OptionReader::new(COMMANDS[0])
                        .usage(format!("`{command}` is not a command of ripe4rust")),
                )
            })?;
        OptionReader::new(*spec)
            .read(options, format)
            .map_err(refused)
    }

    fn asked_format(words: &[&str]) -> Format {
        let json = words.windows(2).any(|pair| pair == ["--format", "json"])
            || words.contains(&"--format=json");
        if json { Format::Json } else { Format::Table }
    }
}
