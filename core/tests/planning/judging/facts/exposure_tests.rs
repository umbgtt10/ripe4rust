// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::surveyed::histories;
use crate::support::surveyed::surveyed;
use crate::support::surveyed::toolchain;
use crate::support::units::breaking;
use crate::support::units::compatible;
use crate::support::units::moved;
use ripe4rust_core::planning::judging::facts::exposure::Exposure;
use ripe4rust_core::planning::judging::facts::plan_context::PlanContext;
use ripe4rust_core::planning::judging::raise::Raise;
use ripe4rust_core::planning::pins_and_policy::exposure_override::ExposureOverride;
use ripe4rust_core::values::partial_version::PartialVersion;

#[test]
fn exposed_in_follows_the_parse_where_the_policy_is_silent() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let exposed = Exposure::new(&context).exposed_in(&breaking("widget", "1.4.0", "2.1.0"));

    // Assert
    assert_eq!(exposed, vec![String::from("app-core")]);
}

#[test]
fn exposed_in_of_a_compatible_unit_is_empty() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let exposed = Exposure::new(&context).exposed_in(&compatible(
        "widget",
        vec![moved("widget", "1.4.0", "1.4.1")],
    ));

    // Assert
    assert_eq!(exposed, Vec::<String>::new());
}

#[test]
fn exposed_in_of_a_crate_the_policy_calls_private_is_empty() {
    // Arrange
    let mut surveyed = surveyed();
    surveyed.policy.exposure = vec![ExposureOverride {
        name: String::from("widget"),
        public: false,
    }];
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let exposed = Exposure::new(&context).exposed_in(&breaking("widget", "1.4.0", "2.1.0"));

    // Assert
    assert_eq!(exposed, Vec::<String>::new());
}

#[test]
fn exposed_in_of_a_crate_the_policy_calls_public_is_every_published_user() {
    // Arrange
    let mut surveyed = surveyed();
    surveyed.policy.published = vec![String::from("app")];
    surveyed.policy.exposure = vec![ExposureOverride {
        name: String::from("gadget"),
        public: true,
    }];
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let exposed = Exposure::new(&context).exposed_in(&breaking("gadget", "0.13.1", "0.14.0"));

    // Assert
    assert_eq!(exposed, vec![String::from("app")]);
}

#[test]
fn exposed_in_of_sources_not_yet_scanned_is_empty() {
    // Arrange
    let mut surveyed = surveyed();
    surveyed.scan = None;
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let exposed = Exposure::new(&context).exposed_in(&breaking("widget", "1.4.0", "2.1.0"));

    // Assert
    assert_eq!(exposed, Vec::<String>::new());
}

#[test]
fn raises_name_every_user_whose_rust_version_the_target_needs_raised() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let needs = PartialVersion::parse("1.88").expect("a rust-version");

    // Act
    let raises = Exposure::new(&context).raises(&breaking("widget", "1.4.0", "2.1.0"));

    // Assert
    assert_eq!(
        raises,
        vec![
            Raise {
                member: String::from("app"),
                to: needs.clone(),
                published: false,
            },
            Raise {
                member: String::from("app-core"),
                to: needs,
                published: true,
            },
        ]
    );
}

#[test]
fn raises_of_a_target_declaring_no_rust_version_are_none() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let raises = Exposure::new(&context).raises(&breaking("widget", "1.4.0", "2.0.1"));

    // Assert
    assert_eq!(raises, Vec::new());
}

#[test]
fn unknown_in_of_a_compatible_unit_is_empty() {
    // Arrange
    let mut surveyed = surveyed();
    surveyed.scan = None;
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let unknown = Exposure::new(&context).unknown_in(&compatible(
        "widget",
        vec![moved("widget", "1.4.0", "1.4.1")],
    ));

    // Assert
    assert_eq!(unknown, Vec::<String>::new());
}

#[test]
fn unknown_in_of_a_crate_the_policy_overrides_is_empty() {
    // Arrange
    let mut surveyed = surveyed();
    surveyed.scan = None;
    surveyed.policy.exposure = vec![ExposureOverride {
        name: String::from("widget"),
        public: false,
    }];
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let unknown = Exposure::new(&context).unknown_in(&breaking("widget", "1.4.0", "2.1.0"));

    // Assert
    assert_eq!(unknown, Vec::<String>::new());
}

#[test]
fn unknown_in_of_scanned_sources_is_empty() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let unknown = Exposure::new(&context).unknown_in(&breaking("widget", "1.4.0", "2.1.0"));

    // Assert
    assert_eq!(unknown, Vec::<String>::new());
}

#[test]
fn unknown_in_of_sources_not_yet_scanned_is_every_published_user() {
    // Arrange
    let mut surveyed = surveyed();
    surveyed.scan = None;
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let unknown = Exposure::new(&context).unknown_in(&breaking("widget", "1.4.0", "2.1.0"));

    // Assert
    assert_eq!(unknown, vec![String::from("app-core")]);
}
