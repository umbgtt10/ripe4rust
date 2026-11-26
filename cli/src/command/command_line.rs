// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::command::format::Format;
use crate::command::refusal::Refusal;
use crate::command::request::Request;
use ripe4rust_core::diagnostics::diagnostic::Diagnostic;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::values::date::Date;

const SUBCOMMAND: &str = "ripe4rust";
const PLAN: &str = "plan";
const LATER: [&str; 3] = ["check", "hold", "guide"];
const USAGE: &str =
    "run `cargo ripe4rust [plan] [--format table|json] [--date YYYY-MM-DD] [--offline]`";
const DATE: &str = "`--date` takes a date, YYYY-MM-DD";
const FORMAT: &str = "`--format` takes `table` or `json`";

/// The command line: `cargo ripe4rust [plan] [--format table|json] [--date
/// YYYY-MM-DD] [--offline]`, the subcommand's own name cargo passes first
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
                    "use `cargo ripe4rust plan`, the one command this release runs; `check`, `hold` and `guide` come with a later release",
                ),
            )));
        }
        if command != PLAN {
            return Err(refused(Self::usage(format!(
                "`{command}` is not a command of ripe4rust"
            ))));
        }
        Self::options(options, format).map_err(refused)
    }

    fn asked_format(words: &[&str]) -> Format {
        let json = words.windows(2).any(|pair| pair == ["--format", "json"])
            || words.contains(&"--format=json");
        if json { Format::Json } else { Format::Table }
    }

    fn date(value: Option<&str>) -> Result<Date, Diagnostic> {
        let value = value.ok_or_else(|| Self::usage(format!("{DATE}, and was given none")))?;
        Date::parse(value).map_err(|_| Self::usage(format!("{DATE}, not `{value}`")))
    }

    fn format(value: Option<&str>) -> Result<Format, Diagnostic> {
        match value {
            Some("json") => Ok(Format::Json),
            Some("table") => Ok(Format::Table),
            Some(other) => Err(Self::usage(format!("{FORMAT}, not `{other}`"))),
            None => Err(Self::usage(format!("{FORMAT}, and was given none"))),
        }
    }

    fn options(options: &[&str], format: Format) -> Result<Request, Diagnostic> {
        let mut request = Request {
            command: String::from(PLAN),
            format,
            date: None,
            offline: false,
        };
        let mut words = options.iter();
        while let Some(word) = words.next() {
            let (flag, inline) = word
                .split_once('=')
                .map_or((*word, None), |(flag, value)| (flag, Some(value)));
            let mut value = || inline.or_else(|| words.next().copied());
            match flag {
                "--offline" if inline.is_none() => request.offline = true,
                "--format" => request.format = Self::format(value())?,
                "--date" => request.date = Some(Self::date(value())?),
                _ => {
                    return Err(Self::usage(format!(
                        "`{word}` is not an option of `{PLAN}`"
                    )));
                }
            }
        }
        Ok(request)
    }

    fn usage(what: String) -> Diagnostic {
        DiagnosticCode::UsageInvalid.diagnose(None, what, String::from(USAGE))
    }
}
