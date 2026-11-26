// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::command::command_spec::CommandSpec;
use crate::command::format::Format;
use crate::command::request::Request;
use ripe4rust_core::diagnostics::diagnostic::Diagnostic;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::values::date::Date;

const DATE: &str = "a date, YYYY-MM-DD";
const FORMAT: &str = "`--format` takes `table` or `json`";
const PLAN: &str = "--plan";
const BECAUSE: &str = "--because";

/// What follows a command on the line, read against what the command
/// takes: its unit, and each option with its value after it or after `=`.
/// Anything it does not take, a value of the wrong kind, or what it cannot go
/// without missing, is `usage-invalid`, with the command's usage.
pub struct OptionReader {
    spec: CommandSpec,
}

impl OptionReader {
    pub fn new(spec: CommandSpec) -> Self {
        Self { spec }
    }

    pub fn date(&self, flag: &str, value: Option<&str>) -> Result<Date, Diagnostic> {
        let value = value
            .ok_or_else(|| self.usage(format!("`{flag}` takes {DATE}, and was given none")))?;
        Date::parse(value).map_err(|_| self.usage(format!("`{flag}` takes {DATE}, not `{value}`")))
    }

    pub fn format(&self, value: Option<&str>) -> Result<Format, Diagnostic> {
        match value {
            Some("json") => Ok(Format::Json),
            Some("table") => Ok(Format::Table),
            Some(other) => Err(self.usage(format!("{FORMAT}, not `{other}`"))),
            None => Err(self.usage(format!("{FORMAT}, and was given none"))),
        }
    }

    pub fn read(&self, words: &[&str], format: Format) -> Result<Request, Diagnostic> {
        let mut request = Request {
            command: String::from(self.spec.name),
            format,
            date: None,
            offline: false,
            unit: None,
            plan: None,
            because: None,
            review: None,
        };
        let mut words = words.iter();
        while let Some(word) = words.next() {
            if self.spec.unit && !word.starts_with('-') {
                request.unit = Some(self.unit(word, request.unit.as_deref())?);
                continue;
            }
            let (flag, inline) = word
                .split_once('=')
                .map_or((*word, None), |(flag, value)| (flag, Some(value)));
            let mut value = || inline.or_else(|| words.next().copied());
            match flag {
                "--offline" if inline.is_none() && self.knows(flag) => request.offline = true,
                "--format" => request.format = self.format(value())?,
                "--date" if self.knows(flag) => request.date = Some(self.date(flag, value())?),
                "--review" if self.knows(flag) => request.review = Some(self.date(flag, value())?),
                PLAN if self.knows(flag) => {
                    request.plan = Some(self.text(flag, "a file", value())?);
                }
                BECAUSE if self.knows(flag) => {
                    request.because = Some(self.text(flag, "the reason", value())?);
                }
                _ => {
                    return Err(
                        self.usage(format!("`{word}` is not an option of `{}`", self.spec.name))
                    );
                }
            }
        }
        self.complete(request)
    }

    pub fn text(&self, flag: &str, what: &str, value: Option<&str>) -> Result<String, Diagnostic> {
        value
            .filter(|value| !value.trim().is_empty())
            .map(String::from)
            .ok_or_else(|| self.usage(format!("`{flag}` takes {what}, and was given none")))
    }

    pub fn usage(&self, what: String) -> Diagnostic {
        DiagnosticCode::UsageInvalid.diagnose(None, what, String::from(self.spec.usage))
    }

    fn complete(&self, request: Request) -> Result<Request, Diagnostic> {
        let required = |flag: &str| self.spec.required.contains(&flag);
        let missing = [
            (self.spec.unit && request.unit.is_none(), "the id of a unit"),
            (required(PLAN) && request.plan.is_none(), "`--plan <file>`"),
            (
                required(BECAUSE) && request.because.is_none(),
                "`--because <text>`",
            ),
        ]
        .into_iter()
        .find_map(|(missing, what)| missing.then_some(what));
        match missing {
            Some(what) => Err(self.usage(format!(
                "`{}` takes {what}, and was given none",
                self.spec.name
            ))),
            None => Ok(request),
        }
    }

    fn knows(&self, flag: &str) -> bool {
        self.spec.options.contains(&flag)
    }

    fn unit(&self, word: &str, given: Option<&str>) -> Result<String, Diagnostic> {
        match given {
            Some(_) => Err(self.usage(format!(
                "`{word}` is a second unit, and `{}` takes one",
                self.spec.name
            ))),
            None => Ok(String::from(word)),
        }
    }
}
