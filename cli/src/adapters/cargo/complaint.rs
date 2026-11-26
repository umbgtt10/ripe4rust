// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::system::model::command_output::CommandOutput;
use crate::system::model::launch_failure::LaunchFailure;

const ERROR: &str = "error";
const ERROR_PREFIX: &str = "error: ";

/// What a program that failed said, on one line: from its first error to
/// the end, or everything it said where it names no error.
pub struct Complaint;

impl Complaint {
    pub fn line(&self, text: &str) -> String {
        text.split_whitespace().collect::<Vec<&str>>().join(" ")
    }

    pub fn of(&self, output: &CommandOutput) -> String {
        let lines = output
            .stderr
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>();
        let from = lines
            .iter()
            .position(|line| line.starts_with(ERROR))
            .unwrap_or(0);
        let said = self.line(
            &lines[from..]
                .iter()
                .map(|line| line.strip_prefix(ERROR_PREFIX).unwrap_or(line))
                .collect::<Vec<&str>>()
                .join(" "),
        );
        match (said.is_empty(), output.status) {
            (false, _) => said,
            (true, Some(status)) => format!("it exited with status {status}, saying nothing"),
            (true, None) => String::from("it was ended by a signal"),
        }
    }

    pub fn unlaunched(&self, failure: &LaunchFailure) -> String {
        match failure {
            LaunchFailure::Missing { program } => format!("`{program}` is not on the PATH"),
            LaunchFailure::Failed { program, detail } => {
                format!("`{program}` could not be started: {detail}")
            }
        }
    }
}
