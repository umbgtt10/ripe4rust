// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::cargo::deny::advisory_hit::AdvisoryHit;
use cargo_ripe4rust::adapters::cargo::deny::audit_report::AuditReport;
use ripe4rust_core::inputs::advisory::Advisory;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::values::requirement::Requirement;
use ripe4rust_core::values::version::Version;

fn hit(id: &str, name: &str, version: &str) -> AdvisoryHit {
    AdvisoryHit {
        id: String::from(id),
        name: String::from(name),
        version: Version::parse(version).expect("a version"),
    }
}

fn requirements(texts: &[&str]) -> Vec<Requirement> {
    texts
        .iter()
        .map(|text| Requirement::parse(text).expect("a requirement"))
        .collect()
}

#[test]
fn advisories_of_a_hit_the_report_does_not_hold_clear_nothing() {
    // Arrange & Act
    let advisories = AuditReport.advisories(
        include_str!("../../../fixtures/workspace/cargo-deny/audit.json"),
        &[hit("RUSTSEC-2099-0001", "smallvec", "1.6.0")],
    );

    // Assert
    assert_eq!(
        advisories,
        Ok(vec![Advisory {
            id: String::from("RUSTSEC-2099-0001"),
            name: String::from("smallvec"),
            version: Version::parse("1.6.0").expect("a version"),
            patched: Vec::new(),
            unaffected: Vec::new(),
        }])
    );
}

#[test]
fn advisories_of_a_range_that_does_not_read_are_internal() {
    // Arrange
    let report = r#"{"vulnerabilities":[{"advisory":{"id":"RUSTSEC-2021-0003"},"package":{"name":"smallvec","version":"1.6.0"},"versions":{"patched":["at least two"],"unaffected":[]}}],"warnings":{}}"#;

    // Act
    let advisories =
        AuditReport.advisories(report, &[hit("RUSTSEC-2021-0003", "smallvec", "1.6.0")]);

    // Assert
    assert_eq!(
        advisories,
        Err(PortFailure::Internal {
            doing: String::from("reading what `cargo deny` printed"),
            detail: String::from("`at least two` is not a version requirement"),
        })
    );
}

#[test]
fn advisories_of_a_vulnerability_carry_its_patched_and_unaffected_ranges() {
    // Arrange & Act
    let advisories = AuditReport.advisories(
        include_str!("../../../fixtures/workspace/cargo-deny/audit.json"),
        &[hit("RUSTSEC-2021-0003", "smallvec", "1.6.0")],
    );

    // Assert
    assert_eq!(
        advisories,
        Ok(vec![Advisory {
            id: String::from("RUSTSEC-2021-0003"),
            name: String::from("smallvec"),
            version: Version::parse("1.6.0").expect("a version"),
            patched: requirements(&[">=0.6.14, <1.0.0", ">=1.6.1"]),
            unaffected: requirements(&["<0.6.3"]),
        }])
    );
}

#[test]
fn advisories_of_an_unmaintained_crate_are_read_from_the_warnings() {
    // Arrange & Act
    let advisories = AuditReport.advisories(
        include_str!("../../../fixtures/cargo-deny/unmaintained.json"),
        &[hit("RUSTSEC-2021-0139", "ansi_term", "0.12.1")],
    );

    // Assert
    assert_eq!(
        advisories.map(|found| found
            .into_iter()
            .map(|advisory| (advisory.id, advisory.patched))
            .collect::<Vec<_>>()),
        Ok(vec![(String::from("RUSTSEC-2021-0139"), Vec::new())])
    );
}
