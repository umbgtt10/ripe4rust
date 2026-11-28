// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::planning::round::guiding::commands_guide::CommandsGuide;

#[test]
fn commands_are_plan_check_hold_guide_and_version_each_with_its_exit_codes() {
    // Arrange & Act
    let commands = CommandsGuide.commands();

    // Assert
    assert_eq!(
        commands
            .iter()
            .map(|command| (
                command.usage.split(' ').nth(2).unwrap_or_default(),
                command
                    .exits
                    .iter()
                    .map(|exit| exit.code)
                    .collect::<Vec<u8>>()
            ))
            .collect::<Vec<_>>(),
        vec![
            ("[plan]", vec![0, 2]),
            ("check", vec![0, 1, 2, 3]),
            ("hold", vec![0, 2, 3]),
            ("guide", vec![0]),
            ("--version", vec![0]),
        ]
    );
}

#[test]
fn commands_name_every_option_each_takes() {
    // Arrange & Act
    let commands = CommandsGuide.commands();

    // Assert
    let options = commands
        .iter()
        .map(|command| {
            command
                .options
                .iter()
                .map(|option| option.option.as_str())
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<_>>();
    assert_eq!(
        options,
        vec![
            vec![
                "--format table|json",
                "--date YYYY-MM-DD",
                "--offline",
                "--refresh"
            ],
            vec!["<unit>", "--plan <file>", "--format table|json"],
            vec![
                "<unit>",
                "--plan <file>",
                "--because <text>",
                "--review YYYY-MM-DD",
                "--format table|json"
            ],
            vec!["--format table|json"],
            vec!["-V"],
        ]
    );
}

#[test]
fn commands_say_check_exits_as_planned_whatever_it_tells_of_a_re_point() {
    // Arrange & Act
    let commands = CommandsGuide.commands();

    // Assert
    assert_eq!(
        commands
            .get(1)
            .and_then(|check| check.exits.first())
            .map(|exit| exit.means.as_str()),
        Some(
            "as planned: the units applied and those still to go are listed, a move of a crate the units share, of those the unit moves, that came out otherwise is reported, and so is every dependency re-pointed between versions the lock already held -- told, never a difference"
        )
    );
}
