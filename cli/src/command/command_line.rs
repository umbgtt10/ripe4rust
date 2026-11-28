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
const VERSION: &str = "--version";
const VERSION_SHORT: &str = "-V";
const COMMANDS: [CommandSpec; 5] = [
    CommandSpec {
        name: PLAN,
        usage: "run `cargo ripe4rust [plan] [--format table|json] [--date YYYY-MM-DD] [--offline | --refresh]`",
        options: &["--format", "--date", "--offline", "--refresh"],
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
    CommandSpec {
        name: "hold",
        usage: "run `cargo ripe4rust hold <unit> --plan <file> --because <text> [--review YYYY-MM-DD] [--format table|json]`",
        options: &["--format", "--plan", "--because", "--review"],
        unit: true,
        required: &["--plan", "--because"],
    },
    CommandSpec {
        name: "guide",
        usage: "run `cargo ripe4rust guide [--format table|json]`",
        options: &["--format"],
        unit: false,
        required: &[],
    },
    CommandSpec {
        name: VERSION,
        usage: "run `cargo ripe4rust --version`",
        options: &[],
        unit: false,
        required: &[],
    },
];

/// The command line: `cargo ripe4rust [plan] [--format table|json] [--date
/// YYYY-MM-DD] [--offline | --refresh]`, `cargo ripe4rust check <unit> --plan <file>
/// [--format table|json]`, `cargo ripe4rust hold <unit> --plan <file>
/// --because <text> [--review YYYY-MM-DD] [--format table|json]` or `cargo
/// ripe4rust guide [--format table|json]` or `cargo ripe4rust --version`,
/// `-V` alike, the subcommand's own name cargo passes first skipped. Anything it does not know is refused with a
/// diagnostic, in the format the line asked for.
pub struct CommandLine;

impl CommandLine {
    pub fn read(&self, args: &[String]) -> Result<Request, Refusal> {
        let words = args.iter().map(String::as_str).collect::<Vec<&str>>();
        let words = words.strip_prefix(&[SUBCOMMAND]).unwrap_or(&words);
        let format = Self::asked_format(words);
        let (command, options) = match words.split_first() {
            Some((&(VERSION | VERSION_SHORT), options)) => (VERSION, options),
            Some((word, options)) if !word.starts_with('-') => (*word, options),
            _ => (PLAN, words),
        };
        let refused = |diagnostic: Diagnostic| Refusal {
            command: String::from(command),
            format,
            diagnostic,
        };
        let spec = COMMANDS
            .iter()
            .find(|spec| spec.name == command)
            .ok_or_else(|| {
                refused(DiagnosticCode::UsageInvalid.diagnose(
                    None,
                    format!("`{command}` is not a command of ripe4rust"),
                    String::from(
                        "run `cargo ripe4rust guide`, which names every command and what each takes",
                    ),
                ))
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
