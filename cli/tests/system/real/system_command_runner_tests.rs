// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::system::model::invocation::Invocation;
use cargo_ripe4rust::system::model::launch_failure::LaunchFailure;
use cargo_ripe4rust::system::real::system_command_runner::SystemCommandRunner;
use cargo_ripe4rust::system::traits::command_runner::CommandRunner;
use std::env::temp_dir;

fn invocation(program: &str, args: &[&str]) -> Invocation {
    Invocation {
        program: String::from(program),
        args: args.iter().map(|arg| String::from(*arg)).collect(),
        dir: temp_dir(),
    }
}

#[test]
fn run_all_of_more_commands_than_run_at_once_answers_each_in_their_order() {
    // Arrange
    let invocations = (0..12)
        .map(|at| match at % 3 {
            0 => invocation("cargo", &["--version"]),
            1 => invocation("no-such-program-4rust", &[]),
            _ => invocation("rustc", &["--version"]),
        })
        .collect::<Vec<Invocation>>();

    // Act
    let outputs = SystemCommandRunner.run_all(&invocations);

    // Assert
    assert_eq!(
        outputs
            .iter()
            .map(|output| match output {
                Ok(printed) => printed.stdout.split(' ').next().unwrap_or_default(),
                Err(_) => "missing",
            })
            .collect::<Vec<&str>>(),
        ["cargo", "missing", "rustc"].repeat(4)
    );
}

#[test]
fn run_of_a_failing_command_captures_its_stderr_and_status() {
    // Arrange & Act
    let output = SystemCommandRunner
        .run(&invocation("cargo", &["no-such-subcommand-4rust"]))
        .expect("cargo runs");

    // Assert
    assert_eq!(output.status, Some(101));
    assert!(
        output.stderr.contains("no such command"),
        "{}",
        output.stderr
    );
}

#[test]
fn run_of_a_program_that_is_not_installed_is_missing() {
    // Arrange & Act
    let output = SystemCommandRunner.run(&invocation("no-such-program-4rust", &[]));

    // Assert
    assert_eq!(
        output,
        Err(LaunchFailure::Missing {
            program: String::from("no-such-program-4rust")
        })
    );
}

#[test]
fn run_of_a_succeeding_command_captures_its_stdout_and_zero() {
    // Arrange & Act
    let output = SystemCommandRunner
        .run(&invocation("cargo", &["--version"]))
        .expect("cargo runs");

    // Assert
    assert_eq!(output.status, Some(0));
    assert!(output.stdout.starts_with("cargo "), "{}", output.stdout);
}
