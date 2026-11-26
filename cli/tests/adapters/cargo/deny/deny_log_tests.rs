// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::cargo::deny::advisory_hit::AdvisoryHit;
use cargo_ripe4rust::adapters::cargo::deny::deny_log::DenyLog;
use ripe4rust_core::values::version::Version;

fn hit(id: &str, name: &str, version: &str) -> AdvisoryHit {
    AdvisoryHit {
        id: String::from(id),
        name: String::from(name),
        version: Version::parse(version).expect("a version"),
    }
}

#[test]
fn read_of_a_config_it_cannot_parse_is_its_error() {
    // Arrange & Act
    let findings = DenyLog.read(include_str!("../../../fixtures/cargo-deny/broken.jsonl"));

    // Assert
    assert_eq!(
        findings.errors,
        vec![String::from(
            "failed to parse config from '/work/ws/deny.toml': expected a right bracket, found a newline"
        )]
    );
}

#[test]
fn read_of_a_lock_that_needs_moving_is_cargo_s_error_on_one_line() {
    // Arrange & Act
    let findings = DenyLog.read(include_str!("../../../fixtures/cargo-deny/unlocked.jsonl"));

    // Assert
    assert_eq!(
        findings.errors.first().map(String::as_str),
        Some(
            "failed to fetch crates: Updating crates.io index error: cannot update the lock file /work/ws/Cargo.lock because --locked was passed to prevent this help: to generate the lock file without accessing the network, remove the --locked flag and use --offline instead."
        )
    );
}

#[test]
fn read_of_an_ignored_advisory_leaves_it_out() {
    // Arrange & Act
    let findings = DenyLog.read(include_str!("../../../fixtures/cargo-deny/ignored.jsonl"));

    // Assert
    assert_eq!(
        findings.hits,
        vec![hit("RUSTSEC-2021-0139", "ansi_term", "0.12.1")]
    );
    assert_eq!(findings.errors, Vec::<String>::new());
}

#[test]
fn read_of_an_unknown_key_is_its_error_first() {
    // Arrange & Act
    let findings = DenyLog.read(include_str!(
        "../../../fixtures/cargo-deny/unknown-key.jsonl"
    ));

    // Assert
    assert_eq!(findings.errors.len(), 2);
    assert!(
        findings.errors[0].starts_with("found 1 unexpected keys, expected: "),
        "{}",
        findings.errors[0]
    );
}

#[test]
fn read_of_lines_that_are_not_json_skips_them() {
    // Arrange & Act
    let findings = DenyLog.read("    Blocking waiting for file lock on package cache\n");

    // Assert
    assert_eq!(
        (findings.hits, findings.errors),
        (Vec::new(), Vec::<String>::new())
    );
}

#[test]
fn read_of_unmaintained_and_vulnerable_crates_hits_both() {
    // Arrange & Act
    let findings = DenyLog.read(include_str!(
        "../../../fixtures/cargo-deny/unmaintained.jsonl"
    ));

    // Assert
    assert_eq!(
        findings.hits,
        vec![
            hit("RUSTSEC-2021-0139", "ansi_term", "0.12.1"),
            hit("RUSTSEC-2021-0003", "smallvec", "1.6.0"),
        ]
    );
}
