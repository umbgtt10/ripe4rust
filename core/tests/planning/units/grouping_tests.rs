// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::release;
use crate::support::releases::version;
use crate::support::units::breaking;
use crate::support::units::compatible;
use crate::support::units::moved;
use ripe4rust_core::inputs::locked_package::LockedPackage;
use ripe4rust_core::inputs::resolved_dependency::ResolvedDependency;
use ripe4rust_core::planning::choice::histories::Histories;
use ripe4rust_core::planning::choice::release_history::ReleaseHistory;
use ripe4rust_core::planning::units::crate_repository::CrateRepository;
use ripe4rust_core::planning::units::grouping::Grouping;
use ripe4rust_core::values::requirement::Requirement;
use ripe4rust_core::values::version::Version;

const FRAMEWORK: &str = "https://example.org/framework";

fn histories() -> Histories {
    let history = |releases| ReleaseHistory::new(releases, Version::line);
    Histories::new(vec![
        (
            String::from("frame"),
            history(vec![release("2.0.0", "2026-10-01")]),
        ),
        (
            String::from("frame-macros"),
            history(vec![release("2.0.0", "2026-10-01")]),
        ),
        (
            String::from("frame-test"),
            history(vec![release("2.0.0", "2026-10-03")]),
        ),
        (
            String::from("other"),
            history(vec![release("2.0.0", "2026-10-01")]),
        ),
    ])
}

fn lock() -> Vec<LockedPackage> {
    vec![LockedPackage {
        name: String::from("engine"),
        version: version("0.4.1"),
        dependencies: vec![ResolvedDependency {
            name: String::from("engine-sys"),
            version: version("0.4.1"),
            requirement: Requirement::parse("=0.4.1").expect("a requirement"),
        }],
    }]
}

fn repositories() -> Vec<CrateRepository> {
    ["frame", "frame-macros", "frame-test"]
        .iter()
        .map(|name| CrateRepository {
            name: String::from(*name),
            repository: String::from(FRAMEWORK),
        })
        .collect()
}

#[test]
fn group_leaves_unrelated_units_apart() {
    // Arrange
    let (repositories, lock, histories) = (repositories(), lock(), histories());
    let units = vec![
        breaking("frame", "1.4.0", "2.0.0"),
        breaking("other", "1.0.0", "2.0.0"),
    ];

    // Act
    let grouped = Grouping::new(&repositories, &lock, &histories).group(units.clone());

    // Assert
    assert_eq!(grouped, units);
}

#[test]
fn group_merges_related_units_under_the_first_name() {
    // Arrange
    let (repositories, lock, histories) = (repositories(), lock(), histories());
    let units = vec![
        breaking("frame", "1.4.0", "2.0.0"),
        breaking("frame-macros", "1.4.0", "2.0.0"),
        breaking("other", "1.0.0", "2.0.0"),
    ];

    // Act
    let grouped = Grouping::new(&repositories, &lock, &histories).group(units);

    // Assert
    assert_eq!(grouped.len(), 2);
    assert_eq!(grouped[0].id.to_string(), "breaking:frame");
    assert_eq!(
        grouped[0].members,
        vec![String::from("frame"), String::from("frame-macros")]
    );
    assert_eq!(
        grouped[0].moves,
        vec![
            moved("frame", "1.4.0", "2.0.0"),
            moved("frame-macros", "1.4.0", "2.0.0")
        ]
    );
    assert_eq!(grouped[1].id.to_string(), "breaking:other");
}

#[test]
fn related_of_crates_one_requiring_the_other_exactly_is_true() {
    // Arrange
    let (repositories, lock, histories) = (repositories(), lock(), histories());
    let grouping = Grouping::new(&repositories, &lock, &histories);

    // Act
    let related = grouping.related(
        &compatible("engine-sys", vec![moved("engine-sys", "0.4.1", "0.4.2")]),
        &compatible("engine", vec![moved("engine", "0.4.1", "0.4.2")]),
    );

    // Assert
    assert!(related);
}

#[test]
fn related_of_crates_released_apart_is_false() {
    // Arrange
    let (repositories, lock, histories) = (repositories(), lock(), histories());
    let grouping = Grouping::new(&repositories, &lock, &histories);

    // Act
    let related = grouping.related(
        &breaking("frame", "1.4.0", "2.0.0"),
        &breaking("frame-test", "1.4.0", "2.0.0"),
    );

    // Assert
    assert!(!related);
}

#[test]
fn related_of_crates_released_together_from_one_repository_is_true() {
    // Arrange
    let (repositories, lock, histories) = (repositories(), lock(), histories());
    let grouping = Grouping::new(&repositories, &lock, &histories);

    // Act
    let related = grouping.related(
        &breaking("frame", "1.4.0", "2.0.0"),
        &breaking("frame-macros", "1.4.0", "2.0.0"),
    );

    // Assert
    assert!(related);
}

#[test]
fn related_of_the_same_release_from_unknown_repositories_is_false() {
    // Arrange
    let (repositories, lock, histories) = (repositories(), lock(), histories());
    let grouping = Grouping::new(&repositories, &lock, &histories);

    // Act
    let related = grouping.related(
        &breaking("frame", "1.4.0", "2.0.0"),
        &breaking("other", "1.0.0", "2.0.0"),
    );

    // Assert
    assert!(!related);
}
