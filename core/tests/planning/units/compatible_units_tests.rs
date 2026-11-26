// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::version;
use crate::support::units::added;
use crate::support::units::dependency;
use crate::support::units::git;
use crate::support::units::moved;
use crate::support::units::source;
use ripe4rust_core::inputs::advisory::Advisory;
use ripe4rust_core::inputs::lock_move::LockMove;
use ripe4rust_core::planning::units::compatible_units::CompatibleUnits;
use ripe4rust_core::planning::units::concern::Concern;
use ripe4rust_core::planning::units::dependency_moves::DependencyMoves;
use ripe4rust_core::planning::units::source_moves::SourceMoves;
use ripe4rust_core::planning::units::unit::Unit;
use ripe4rust_core::planning::units::unit_id::UnitId;
use ripe4rust_core::unit_kind::UnitKind;
use ripe4rust_core::values::requirement::Requirement;

fn compatible_unit(name: &str, from: &str, to: &str) -> Unit {
    Unit {
        id: UnitId::of(UnitKind::Compatible, name),
        kind: UnitKind::Compatible,
        name: Some(String::from(name)),
        members: vec![String::from(name)],
        from: Some(version(from)),
        to: Some(version(to)),
        moves: vec![moved(name, from, to)],
        held: Vec::new(),
        git: None,
    }
}

fn moves_of(name: &str, in_use: &str, moves: Vec<LockMove>) -> DependencyMoves {
    DependencyMoves {
        dependency: dependency(name, in_use),
        moves,
    }
}

#[test]
fn build_forms_the_rest_from_the_moves_no_unit_reaches() {
    // Arrange
    let dependencies = vec![moves_of(
        "clap",
        "4.6.6",
        vec![moved("clap", "4.6.6", "4.6.7")],
    )];
    let workspace = vec![
        moved("clap", "4.6.6", "4.6.7"),
        moved("libc", "0.2.177", "0.2.178"),
        added("windows-link", "0.2.1"),
    ];

    // Act
    let units = CompatibleUnits::new(Vec::new(), Vec::new()).build(dependencies, workspace);

    // Assert
    assert_eq!(
        units[1],
        Unit {
            id: UnitId::rest(),
            kind: UnitKind::Compatible,
            name: None,
            members: Vec::new(),
            from: None,
            to: None,
            moves: vec![
                moved("libc", "0.2.177", "0.2.178"),
                added("windows-link", "0.2.1")
            ],
            held: Vec::new(),
            git: None,
        }
    );
}

#[test]
fn build_gathers_a_dependency_s_moves_into_its_unit() {
    // Arrange
    let dependencies = vec![moves_of(
        "clap",
        "4.6.6",
        vec![
            moved("clap", "4.6.6", "4.6.7"),
            moved("clap_builder", "4.6.6", "4.6.7"),
        ],
    )];

    // Act
    let units = CompatibleUnits::new(Vec::new(), Vec::new()).build(dependencies, Vec::new());

    // Assert
    assert_eq!(
        units,
        vec![Unit {
            id: UnitId::of(UnitKind::Compatible, "clap"),
            kind: UnitKind::Compatible,
            name: Some(String::from("clap")),
            members: vec![String::from("clap")],
            from: Some(version("4.6.6")),
            to: Some(version("4.6.7")),
            moves: vec![
                moved("clap", "4.6.6", "4.6.7"),
                moved("clap_builder", "4.6.6", "4.6.7")
            ],
            held: Vec::new(),
            git: None,
        }]
    );
}

#[test]
fn build_gives_a_shared_move_to_the_first_unit_by_name() {
    // Arrange
    let dependencies = vec![
        moves_of(
            "anyhow",
            "1.0.102",
            vec![
                moved("anyhow", "1.0.102", "1.0.104"),
                moved("memchr", "2.7.5", "2.7.6"),
            ],
        ),
        moves_of(
            "regex",
            "1.11.0",
            vec![
                moved("memchr", "2.7.5", "2.7.6"),
                moved("regex", "1.11.0", "1.11.1"),
            ],
        ),
    ];

    // Act
    let units = CompatibleUnits::new(Vec::new(), Vec::new()).build(dependencies, Vec::new());

    // Assert
    assert_eq!(
        units[0].moves,
        vec![
            moved("anyhow", "1.0.102", "1.0.104"),
            moved("memchr", "2.7.5", "2.7.6")
        ]
    );
    assert_eq!(units[1].moves, vec![moved("regex", "1.11.0", "1.11.1")]);
}

#[test]
fn build_leaves_a_move_pinned_all_out_of_the_unit_and_holds_it() {
    // Arrange
    let dependencies = vec![moves_of(
        "regex",
        "1.11.0",
        vec![
            moved("memchr", "2.7.5", "2.7.6"),
            moved("regex", "1.11.0", "1.11.1"),
        ],
    )];

    // Act
    let units = CompatibleUnits::new(vec![String::from("memchr")], Vec::new())
        .build(dependencies, Vec::new());

    // Assert
    assert_eq!(units[0].moves, vec![moved("regex", "1.11.0", "1.11.1")]);
    assert_eq!(units[0].held, vec![moved("memchr", "2.7.5", "2.7.6")]);
}

#[test]
fn build_of_a_dependency_that_does_not_move_itself_takes_it_to_no_version() {
    // Arrange
    let dependencies = vec![moves_of(
        "tokio",
        "1.48.0",
        vec![moved("mio", "1.0.4", "1.0.5")],
    )];

    // Act
    let units = CompatibleUnits::new(Vec::new(), Vec::new()).build(dependencies, Vec::new());

    // Assert
    assert_eq!(units[0].from, Some(version("1.48.0")));
    assert_eq!(units[0].to, None);
    assert_eq!(units[0].held, Vec::new());
}

#[test]
fn build_of_a_dependency_whose_own_move_is_held_takes_no_version() {
    // Arrange
    let dependencies = vec![moves_of(
        "memchr",
        "2.5.0",
        vec![
            moved("memchr", "2.5.0", "2.7.6"),
            moved("libc", "0.2.177", "0.2.178"),
        ],
    )];

    // Act
    let units = CompatibleUnits::new(vec![String::from("memchr")], Vec::new())
        .build(dependencies, Vec::new());

    // Assert
    assert_eq!(units[0].from, Some(version("2.5.0")));
    assert_eq!(units[0].to, None);
    assert_eq!(units[0].moves, vec![moved("libc", "0.2.177", "0.2.178")]);
    assert_eq!(units[0].held, vec![moved("memchr", "2.5.0", "2.7.6")]);
}

#[test]
fn build_of_nothing_left_for_the_rest_forms_none() {
    // Arrange
    let dependencies = vec![moves_of(
        "clap",
        "4.6.6",
        vec![moved("clap", "4.6.6", "4.6.7")],
    )];

    // Act
    let units = CompatibleUnits::new(Vec::new(), Vec::new())
        .build(dependencies, vec![moved("clap", "4.6.6", "4.6.7")]);

    // Assert
    assert_eq!(units.len(), 1);
}

#[test]
fn build_skips_a_dependency_with_nothing_left_to_move() {
    // Arrange
    let dependencies = vec![
        moves_of(
            "serde",
            "1.0.228",
            vec![
                moved("serde", "1.0.228", "1.0.229"),
                moved("serde_derive", "1.0.228", "1.0.229"),
            ],
        ),
        moves_of(
            "serde_derive",
            "1.0.228",
            vec![moved("serde_derive", "1.0.228", "1.0.229")],
        ),
    ];

    // Act
    let units = CompatibleUnits::new(Vec::new(), Vec::new()).build(dependencies, Vec::new());

    // Assert
    assert_eq!(
        units
            .iter()
            .map(|unit| unit.id.to_string())
            .collect::<Vec<_>>(),
        vec![String::from("compatible:serde")]
    );
}

#[test]
fn build_with_sources_forms_a_git_unit_for_each_claiming_in_plan_order() {
    // Arrange
    let dependencies = vec![
        moves_of(
            "anyhow",
            "1.0.100",
            vec![moved("anyhow", "1.0.100", "1.0.102")],
        ),
        moves_of("itoa", "1.0.10", vec![moved("itoa", "1.0.10", "1.0.19")]),
    ];
    let sources = vec![SourceMoves {
        source: source("9625c3d2", "ce129c85"),
        name: String::from("gitcore"),
        version: version("0.1.0"),
        crates: vec![String::from("gitcore"), String::from("gitmacros")],
        moves: vec![
            moved("anyhow", "1.0.100", "1.0.102"),
            moved("itoa", "1.0.10", "1.0.19"),
        ],
    }];

    // Act
    let units = CompatibleUnits::new(Vec::new(), Vec::new()).build_with_sources(
        dependencies,
        sources,
        vec![moved("anyhow", "1.0.100", "1.0.102")],
    );

    // Assert
    assert_eq!(
        units,
        vec![
            compatible_unit("anyhow", "1.0.100", "1.0.102"),
            git(
                "gitcore",
                &["gitcore", "gitmacros"],
                vec![moved("itoa", "1.0.10", "1.0.19")]
            ),
        ]
    );
}

#[test]
fn holds_a_move_pinned_all_that_clears_no_concern() {
    // Arrange
    let units = CompatibleUnits::new(vec![String::from("memchr")], Vec::new());

    // Act
    let held = units.holds(&moved("memchr", "2.7.5", "2.7.6"));

    // Assert
    assert!(held);
}

#[test]
fn holds_no_move_pinned_all_that_clears_an_advisory() {
    // Arrange
    let concern = Concern::Advisory(Advisory {
        id: String::from("RUSTSEC-2026-0003"),
        name: String::from("memchr"),
        version: version("2.7.5"),
        patched: vec![Requirement::parse(">=2.7.6").expect("a requirement")],
        unaffected: Vec::new(),
    });
    let units = CompatibleUnits::new(vec![String::from("memchr")], vec![concern]);

    // Act
    let held = units.holds(&moved("memchr", "2.7.5", "2.7.6"));

    // Assert
    assert!(!held);
}
