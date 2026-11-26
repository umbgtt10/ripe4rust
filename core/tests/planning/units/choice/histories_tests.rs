// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::date;
use crate::support::releases::release;
use crate::support::releases::version;
use crate::support::releases::yanked;
use ripe4rust_core::planning::units::choice::histories::Histories;
use ripe4rust_core::planning::units::choice::release_history::ReleaseHistory;
use ripe4rust_core::values::version::Version;

fn histories() -> Histories {
    Histories::new(vec![(
        String::from("spin"),
        ReleaseHistory::new(
            vec![
                yanked("0.10.0", "2026-11-01"),
                release("0.10.1", "2026-11-18"),
            ],
            Version::line,
        ),
    )])
}

#[test]
fn get_of_a_crate_not_read_is_none() {
    // Arrange
    let histories = histories();

    // Act
    let history = histories.get("tokio");

    // Assert
    assert!(history.is_none());
}

#[test]
fn get_of_a_crate_read_is_its_history() {
    // Arrange & Act
    let history = histories()
        .get("spin")
        .map(|history| history.candidates(&version("1.99.1")));

    // Assert
    assert_eq!(history, Some(vec![version("0.10.1")]));
}

#[test]
fn is_yanked_asks_the_crate_s_history() {
    // Arrange & Act
    let withdrawn = histories().is_yanked("spin", &version("0.10.0"));

    // Assert
    assert!(withdrawn);
}

#[test]
fn published_on_of_a_crate_not_read_is_none() {
    // Arrange & Act
    let published = histories().published_on("tokio", &version("1.0.0"));

    // Assert
    assert_eq!(published, None);
}

#[test]
fn published_on_of_a_release_is_its_date() {
    // Arrange & Act
    let published = histories().published_on("spin", &version("0.10.1"));

    // Assert
    assert_eq!(published, Some(date("2026-11-18")));
}
