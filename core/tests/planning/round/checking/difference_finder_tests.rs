// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::recorded::lines;
use crate::support::recorded::lines_with;
use crate::support::recorded::lock;
use crate::support::recorded::plan;
use crate::support::recorded::transitive;
use crate::support::releases::date;
use crate::support::surveyed::at;
use crate::support::units::moved;
use ripe4rust_core::inputs::lock_entry::LockEntry;
use ripe4rust_core::inputs::written_line::WrittenLine;
use ripe4rust_core::planning::pins_and_policy::hold::Hold;
use ripe4rust_core::planning::pins_and_policy::pin::Pin;
use ripe4rust_core::planning::round::checking::accountable::Accountable;
use ripe4rust_core::planning::round::checking::difference_finder::DifferenceFinder;
use ripe4rust_core::planning::round::current_state::CurrentState;
use ripe4rust_core::planning::round::sorting::Sorting;
use ripe4rust_core::planning::round::target_finder::TargetFinder;

const GADGET_REVERT: &str = "carry out the unit's `revert`, `git restore -- Cargo.lock Cargo.toml`: breaking:gadget becomes ask, with these differences attached";

fn found(unit: &str, lock: Vec<LockEntry>, lines: Vec<WrittenLine>, pins: &[Pin]) -> Vec<String> {
    let plan = plan();
    let current = CurrentState {
        lock,
        lines,
        git: Vec::new(),
        edges: Vec::new(),
    };
    let sorting = Sorting::new(&plan, &current);
    let target = TargetFinder::new(&plan).find(unit).expect("a unit");
    let accountable = Accountable::new(&target, sorting.ownership(), sorting.lock());
    DifferenceFinder::new(&target, &sorting, pins, &accountable)
        .differences(&sorting.sort())
        .into_iter()
        .map(|difference| format!("{}: {}", difference.at, difference.what))
        .collect()
}

fn gadget_moved(tail: &[(&str, &str)]) -> Vec<LockEntry> {
    let mut entries = vec![
        ("gadget", "0.14.0"),
        ("libc", "0.2.177"),
        ("widget", "1.4.0"),
    ];
    entries.extend_from_slice(tail);
    entries.sort_unstable();
    lock(&entries)
}

fn pin(hold: Hold) -> Pin {
    Pin {
        name: String::from("gizmo"),
        reason: String::from("3 breaks the build"),
        review: Some(date("2027-01-31")),
        until: None,
        hold,
        at: at("Cargo.toml", 41),
    }
}

#[test]
fn differences_of_a_crate_gone_say_it_left_the_lock() {
    // Arrange
    let current = lock(&[
        ("gadget", "0.13.1"),
        ("gizmo", "2.7.6"),
        ("libc", "0.2.177"),
    ]);

    // Act
    let differences = found("compatible:widget", current, lines(), &[]);

    // Assert
    assert_eq!(
        differences,
        vec!["Cargo.lock: `widget` 1.4.0 left the lock, where the unit moves it to 1.4.1"]
    );
}

#[test]
fn differences_of_a_crate_moved_elsewhere_name_where_it_went() {
    // Arrange
    let current = lock(&[
        ("gadget", "0.13.1"),
        ("gizmo", "2.7.6"),
        ("libc", "0.2.177"),
        ("widget", "1.4.2"),
    ]);

    // Act
    let differences = found("compatible:widget", current, lines(), &[]);

    // Assert
    assert_eq!(
        differences,
        vec!["Cargo.lock: `widget` moved 1.4.0 -> 1.4.2, not to 1.4.1 as the unit moves it"]
    );
}

#[test]
fn differences_of_a_tail_move_of_a_crate_held_whole_name_the_pin() {
    // Arrange
    let current = gadget_moved(&[("gizmo", "2.7.9")]);

    // Act
    let differences = found(
        "breaking:gadget",
        current,
        lines_with(&[(22, "0.14.0")]),
        &[pin(Hold::All)],
    );

    // Assert
    assert_eq!(
        differences,
        vec![
            "Cargo.lock: `gizmo` moved 2.7.5 -> 2.7.9, which its pin holds with `hold = \"all\"`: 3 breaks the build"
        ]
    );
}

#[test]
fn differences_of_a_tail_move_off_a_line_its_pin_holds_name_the_pin() {
    // Arrange
    let current = gadget_moved(&[("gizmo", "3.0.0")]);

    // Act
    let differences = found(
        "breaking:gadget",
        current,
        lines_with(&[(22, "0.14.0")]),
        &[pin(Hold::Line)],
    );

    // Assert
    assert_eq!(
        differences,
        vec![
            "Cargo.lock: `gizmo` moved 2.7.5 -> 3.0.0, off the 2.x line its pin holds: 3 breaks the build"
        ]
    );
}

#[test]
fn differences_of_a_tail_move_on_the_line_its_pin_holds_are_none() {
    // Arrange
    let current = gadget_moved(&[("gizmo", "2.7.9")]);

    // Act
    let differences = found(
        "breaking:gadget",
        current,
        lines_with(&[(22, "0.14.0")]),
        &[pin(Hold::Line)],
    );

    // Assert
    assert_eq!(differences, Vec::<String>::new());
}

#[test]
fn differences_of_an_edit_not_made_name_its_line() {
    // Arrange
    let current = gadget_moved(&[("gizmo", "2.7.5")]);

    // Act
    let differences = found("breaking:gadget", current, lines(), &[]);

    // Assert
    assert_eq!(
        differences,
        vec!["Cargo.toml:22: `gadget` still reads `0.13`, where the unit writes `0.14.0`"]
    );
}

#[test]
fn differences_of_moves_no_unit_makes_are_each_extra() {
    // Arrange
    let current = lock(&[
        ("gizmo", "2.7.6"),
        ("libc", "0.2.177"),
        ("tiny", "1.0.0"),
        ("widget", "1.4.1"),
    ]);

    // Act
    let differences = found("compatible:widget", current, lines(), &[]);

    // Assert
    assert_eq!(
        differences,
        vec![
            "Cargo.lock: `gadget` 0.13.1 left the lock, which no unit of the plan removes",
            "Cargo.lock: `tiny` 1.0.0 joined the lock, which no unit of the plan adds",
        ]
    );
}

#[test]
fn differences_say_each_time_what_to_do() {
    // Arrange
    let plan = plan();
    let current = CurrentState {
        lock: plan.lock.clone(),
        lines: lines(),
        git: Vec::new(),
        edges: Vec::new(),
    };
    let sorting = Sorting::new(&plan, &current);
    let target = TargetFinder::new(&plan)
        .find("breaking:gadget")
        .expect("a unit");
    let accountable = Accountable::new(&target, sorting.ownership(), sorting.lock());

    // Act
    let differences =
        DifferenceFinder::new(&target, &sorting, &[], &accountable).differences(&sorting.sort());

    // Assert
    assert_eq!(
        differences
            .iter()
            .map(|difference| difference.action.as_str())
            .collect::<Vec<_>>(),
        vec![GADGET_REVERT, GADGET_REVERT]
    );
}

#[test]
fn transitive_of_a_breaking_unit_applied_is_every_move_no_unit_explains() {
    // Arrange
    let plan = plan();
    let current = CurrentState {
        lock: gadget_moved(&[("gizmo", "2.7.5"), ("tiny", "1.0.0")]),
        lines: lines_with(&[(22, "0.14.0")]),
        git: Vec::new(),
        edges: Vec::new(),
    };
    let sorting = Sorting::new(&plan, &current);
    let target = TargetFinder::new(&plan)
        .find("breaking:gadget")
        .expect("a unit");
    let accountable = Accountable::new(&target, sorting.ownership(), sorting.lock());

    // Act
    let tail =
        DifferenceFinder::new(&target, &sorting, &[], &accountable).transitive(&sorting.sort());

    // Assert
    assert_eq!(tail, vec![transitive("tiny", "1.0.0")]);
}

#[test]
fn transitive_of_a_compatible_unit_is_none() {
    // Arrange
    let plan = plan();
    let current = CurrentState {
        lock: lock(&[
            ("gadget", "0.13.1"),
            ("gizmo", "2.7.6"),
            ("libc", "0.2.180"),
            ("widget", "1.4.1"),
        ]),
        lines: lines(),
        git: Vec::new(),
        edges: Vec::new(),
    };
    let sorting = Sorting::new(&plan, &current);
    let target = TargetFinder::new(&plan)
        .find("compatible:widget")
        .expect("a unit");
    let accountable = Accountable::new(&target, sorting.ownership(), sorting.lock());

    // Act
    let tail =
        DifferenceFinder::new(&target, &sorting, &[], &accountable).transitive(&sorting.sort());

    // Assert
    assert_eq!(tail, Vec::new());
    assert_eq!(
        sorting.sort().unexplained,
        vec![moved("libc", "0.2.177", "0.2.180")]
    );
}
