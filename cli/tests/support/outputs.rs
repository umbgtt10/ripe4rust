// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::system::model::command_output::CommandOutput;

pub fn failed(status: i32, stderr: &str) -> CommandOutput {
    CommandOutput {
        status: Some(status),
        stdout: String::new(),
        stderr: String::from(stderr),
    }
}

pub fn printed(stdout: &str, stderr: &str) -> CommandOutput {
    CommandOutput {
        status: Some(0),
        stdout: String::from(stdout),
        stderr: String::from(stderr),
    }
}
