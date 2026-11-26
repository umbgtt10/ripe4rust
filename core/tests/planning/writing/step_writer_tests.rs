// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::units::added;
use crate::support::units::breaking;
use crate::support::units::compatible;
use crate::support::units::moved;
use ripe4rust_core::planning::units::concern::Concern;
use ripe4rust_core::planning::units::unit::Unit;
use ripe4rust_core::planning::units::unit_id::UnitId;
use ripe4rust_core::planning::writing::revert::Revert;
use ripe4rust_core::planning::writing::split_step::SplitStep;
use ripe4rust_core::planning::writing::step::Step;
use ripe4rust_core::planning::writing::step_writer::StepWriter;
use ripe4rust_core::planning::writing::steps::Steps;
use ripe4rust_core::unit_kind::UnitKind;
use ripe4rust_core::values::location::Location;
use ripe4rust_core::values::version::Version;

fn at(file: &str, line: u32) -> Location {
    Location {
        file: String::from(file),
        line,
    }
}

fn gates() -> Vec<String> {
    vec![String::from("just stage1"), String::from("just stage2")]
}

fn run(command: &str) -> Step {
    Step::Run {
        command: String::from(command),
    }
}

fn writer() -> StepWriter {
    StepWriter::new(gates(), vec![String::from("syn")])
}

#[test]
fn spec_of_a_crate_held_at_one_version_is_its_name() {
    // Arrange & Act
    let spec = writer().spec("clap", Some(&Version::new(4, 6, 6)));

    // Assert
    assert_eq!(spec, "clap");
}

#[test]
fn spec_of_a_crate_held_at_several_versions_names_its_own() {
    // Arrange & Act
    let spec = writer().spec("syn", Some(&Version::new(2, 0, 119)));

    // Assert
    assert_eq!(spec, "syn@2.0.119");
}

#[test]
fn steps_of_a_breaking_unit_edit_its_lines_then_update_it() {
    // Arrange
    let edits = vec![
        Step::Edit {
            at: at("Cargo.toml", 21),
            key: String::from("widget"),
            value: String::from("3.0.9"),
        },
        Step::Edit {
            at: at("core/Cargo.toml", 7),
            key: String::from("rust-version"),
            value: String::from("1.88"),
        },
    ];

    // Act
    let steps = writer().steps(&breaking("widget", "2.0.119", "3.0.9"), &edits);

    // Assert
    let mut apply = edits;
    apply.push(run("cargo update -p widget"));
    assert_eq!(
        steps,
        Some(Steps {
            apply,
            check: String::from("cargo ripe4rust check breaking:widget --plan plan.json"),
            gates: gates(),
            revert: Revert {
                restore: vec![
                    String::from("Cargo.lock"),
                    String::from("Cargo.toml"),
                    String::from("core/Cargo.toml"),
                ],
                run: String::from("git restore -- Cargo.lock Cargo.toml core/Cargo.toml"),
            },
            split: Vec::new(),
        })
    );
}

#[test]
fn steps_of_a_compatible_unit_update_it_recursively_and_split_its_moves() {
    // Arrange
    let unit = compatible(
        "clap",
        vec![
            moved("clap", "4.6.6", "4.6.7"),
            moved("clap_builder", "4.6.6", "4.6.7"),
        ],
    );

    // Act
    let steps = writer().steps(&unit, &[]);

    // Assert
    assert_eq!(
        steps,
        Some(Steps {
            apply: vec![run("cargo update --recursive -p clap")],
            check: String::from("cargo ripe4rust check compatible:clap --plan plan.json"),
            gates: gates(),
            revert: Revert {
                restore: vec![String::from("Cargo.lock")],
                run: String::from("git restore -- Cargo.lock"),
            },
            split: vec![
                SplitStep {
                    id: String::from("compatible:clap/clap"),
                    apply: vec![run("cargo update -p clap --precise 4.6.7")],
                },
                SplitStep {
                    id: String::from("compatible:clap/clap_builder"),
                    apply: vec![run("cargo update -p clap_builder --precise 4.6.7")],
                },
            ],
        })
    );
}

#[test]
fn steps_of_a_crate_the_lock_holds_twice_name_its_version() {
    // Arrange
    let unit = Unit {
        id: UnitId::rest(),
        kind: UnitKind::Compatible,
        name: None,
        members: Vec::new(),
        from: None,
        to: None,
        moves: vec![
            moved("syn", "2.0.119", "2.0.120"),
            added("unicode-ident", "1.0.20"),
        ],
        held: Vec::new(),
    };

    // Act
    let steps = writer().steps(&unit, &[]);

    // Assert
    let steps = steps.expect("steps");
    assert_eq!(
        steps.apply,
        vec![run("cargo update -p syn@2.0.119 --precise 2.0.120")]
    );
    assert_eq!(steps.check, "cargo ripe4rust check rest --plan plan.json");
}

#[test]
fn steps_of_a_group_update_every_member() {
    // Arrange
    let mut unit = compatible("serde", vec![moved("serde", "1.0.228", "1.0.229")]);
    unit.members.push(String::from("serde_json"));

    // Act
    let steps = writer().steps(&unit, &[]);

    // Assert
    assert_eq!(
        steps.map(|steps| steps.apply),
        Some(vec![run("cargo update --recursive -p serde -p serde_json")])
    );
}

#[test]
fn steps_of_a_single_move_have_no_split() {
    // Arrange
    let unit = compatible("anyhow", vec![moved("anyhow", "1.0.102", "1.0.104")]);

    // Act
    let steps = writer().steps(&unit, &[]);

    // Assert
    assert_eq!(steps.map(|steps| steps.split), Some(Vec::new()));
}

#[test]
fn steps_of_a_unit_with_a_held_move_apply_the_others_one_by_one() {
    // Arrange
    let mut unit = compatible(
        "regex",
        vec![
            moved("regex", "1.11.0", "1.11.1"),
            moved("aho-corasick", "1.1.3", "1.1.4"),
        ],
    );
    unit.held = vec![moved("memchr", "2.7.5", "2.7.6")];

    // Act
    let steps = writer().steps(&unit, &[]);

    // Assert
    assert_eq!(
        steps.map(|steps| steps.apply),
        Some(vec![
            run("cargo update -p regex --precise 1.11.1"),
            run("cargo update -p aho-corasick --precise 1.1.4"),
        ])
    );
}

#[test]
fn steps_of_an_advisory_row_are_none() {
    // Arrange
    let row = Concern::Yanked {
        name: String::from("spin"),
        version: Version::new(0, 10, 0),
    }
    .row();

    // Act
    let steps = writer().steps(&row, &[]);

    // Assert
    assert_eq!(steps, None);
}

#[test]
fn steps_of_the_toolchain_write_its_channel() {
    // Arrange
    let unit = Unit {
        id: UnitId::of(UnitKind::Toolchain, "rust"),
        kind: UnitKind::Toolchain,
        name: Some(String::from("rust")),
        members: vec![String::from("rust")],
        from: Some(Version::new(1, 99, 1)),
        to: Some(Version::new(1, 100, 0)),
        moves: Vec::new(),
        held: Vec::new(),
    };
    let edits = vec![Step::Edit {
        at: at("rust-toolchain.toml", 9),
        key: String::from("channel"),
        value: String::from("1.100.0"),
    }];

    // Act
    let steps = writer().steps(&unit, &edits);

    // Assert
    let steps = steps.expect("steps");
    assert_eq!(steps.apply, edits);
    assert_eq!(
        steps.revert,
        Revert {
            restore: vec![String::from("rust-toolchain.toml")],
            run: String::from("git restore -- rust-toolchain.toml"),
        }
    );
}
