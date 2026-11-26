// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::diagnostics::code_entry::CodeEntry;
use ripe4rust_core::diagnostics::diagnostic::Diagnostic;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::diagnostics::severity::Severity;
use ripe4rust_core::values::location::Location;

const EVERY_CODE: [(DiagnosticCode, &str, Severity, bool); 30] = [
    (
        DiagnosticCode::CargoDenyFailed,
        "cargo-deny-failed",
        Severity::Error,
        false,
    ),
    (
        DiagnosticCode::CargoDenyMissing,
        "cargo-deny-missing",
        Severity::Error,
        false,
    ),
    (
        DiagnosticCode::CargoMetadataFailed,
        "cargo-metadata-failed",
        Severity::Error,
        false,
    ),
    (
        DiagnosticCode::CargoUpdateFailed,
        "cargo-update-failed",
        Severity::Error,
        false,
    ),
    (
        DiagnosticCode::CargoUpdateUnrecognised,
        "cargo-update-unrecognised",
        Severity::Error,
        false,
    ),
    (
        DiagnosticCode::CurlMissing,
        "curl-missing",
        Severity::Error,
        false,
    ),
    (
        DiagnosticCode::GatesUndeclared,
        "gates-undeclared",
        Severity::Warning,
        true,
    ),
    (DiagnosticCode::Internal, "internal", Severity::Error, false),
    (
        DiagnosticCode::LockOutdated,
        "lock-outdated",
        Severity::Error,
        false,
    ),
    (
        DiagnosticCode::PinForAbsentCrate,
        "pin-for-absent-crate",
        Severity::Warning,
        false,
    ),
    (
        DiagnosticCode::PinWithoutReason,
        "pin-without-reason",
        Severity::Error,
        true,
    ),
    (
        DiagnosticCode::PinWithoutRelease,
        "pin-without-release",
        Severity::Error,
        true,
    ),
    (
        DiagnosticCode::PlanSchemaUnknown,
        "plan-schema-unknown",
        Severity::Error,
        false,
    ),
    (
        DiagnosticCode::PlanStale,
        "plan-stale",
        Severity::Error,
        false,
    ),
    (
        DiagnosticCode::PlanUnreadable,
        "plan-unreadable",
        Severity::Error,
        false,
    ),
    (
        DiagnosticCode::PolicyInvalidValue,
        "policy-invalid-value",
        Severity::Error,
        true,
    ),
    (
        DiagnosticCode::PolicyUnknownKey,
        "policy-unknown-key",
        Severity::Error,
        true,
    ),
    (
        DiagnosticCode::RegistryCacheOld,
        "registry-cache-old",
        Severity::Warning,
        false,
    ),
    (
        DiagnosticCode::RegistryCacheUnusable,
        "registry-cache-unusable",
        Severity::Error,
        false,
    ),
    (
        DiagnosticCode::RegistryUnreachable,
        "registry-unreachable",
        Severity::Error,
        false,
    ),
    (
        DiagnosticCode::SourceUnparsed,
        "source-unparsed",
        Severity::Warning,
        false,
    ),
    (
        DiagnosticCode::SourceUnscanned,
        "source-unscanned",
        Severity::Notice,
        false,
    ),
    (
        DiagnosticCode::TargetUnknown,
        "target-unknown",
        Severity::Error,
        true,
    ),
    (
        DiagnosticCode::ToolchainUnpinned,
        "toolchain-unpinned",
        Severity::Notice,
        false,
    ),
    (
        DiagnosticCode::ToolchainReleasesUnknown,
        "toolchain-releases-unknown",
        Severity::Notice,
        false,
    ),
    (
        DiagnosticCode::ToolchainUnreachable,
        "toolchain-unreachable",
        Severity::Error,
        false,
    ),
    (
        DiagnosticCode::WorkspaceUnreadable,
        "workspace-unreadable",
        Severity::Error,
        false,
    ),
    (
        DiagnosticCode::UsageInvalid,
        "usage-invalid",
        Severity::Error,
        false,
    ),
    (
        DiagnosticCode::UnitUnknown,
        "unit-unknown",
        Severity::Error,
        false,
    ),
    (
        DiagnosticCode::UnitUrgent,
        "unit-urgent",
        Severity::Error,
        true,
    ),
];
const SOURCE: &str = include_str!("../../src/diagnostics/diagnostic_code.rs");

#[test]
fn all_holds_every_code_the_enum_declares_once_in_the_order_of_its_labels() {
    // Arrange
    let declared = SOURCE
        .split_once("pub enum DiagnosticCode {")
        .and_then(|(_, rest)| rest.split_once('}'))
        .map(|(body, _)| body)
        .expect("the enum");
    let mut variants = declared
        .split(',')
        .map(str::trim)
        .filter(|variant| !variant.is_empty())
        .map(String::from)
        .collect::<Vec<String>>();
    variants.sort();

    // Act
    let mut all = DiagnosticCode::ALL
        .iter()
        .map(|code| format!("{code:?}"))
        .collect::<Vec<String>>();
    let labels = DiagnosticCode::ALL.map(|code| code.label());
    all.sort();

    // Assert
    assert_eq!(all, variants);
    assert_eq!(all.len(), EVERY_CODE.len());
    assert!(
        labels.windows(2).all(|pair| pair[0] < pair[1]),
        "{labels:?}"
    );
}

#[test]
fn diagnose_takes_severity_and_needs_person_from_the_code() {
    // Arrange
    let at = Location {
        file: String::from("Cargo.toml"),
        line: 41,
    };

    // Act
    let diagnostic = DiagnosticCode::PinWithoutReason.diagnose(
        Some(at.clone()),
        String::from("the pin on `syn` gives no reason"),
        String::from("add a reason"),
    );

    // Assert
    assert_eq!(
        diagnostic,
        Diagnostic {
            severity: Severity::Error,
            code: DiagnosticCode::PinWithoutReason,
            at: Some(at),
            what: String::from("the pin on `syn` gives no reason"),
            action: String::from("add a reason"),
            needs_person: true,
        }
    );
}

#[test]
fn entry_of_every_code_says_what_it_means_and_its_remedy_apart() {
    // Arrange & Act
    let said = DiagnosticCode::ALL.map(|code| (code.entry().means, code.entry().remedy));

    // Assert
    for (meaning, remedy) in said {
        assert!(!meaning.is_empty() && !remedy.is_empty());
        assert_ne!(meaning, remedy);
    }
    assert_eq!(
        DiagnosticCode::PlanStale.entry(),
        CodeEntry {
            means: "the workspace changed outside the plan: a change no unit of it explains",
            remedy: "plan again -- `cargo ripe4rust plan --format json > plan.json` -- and go on with the new plan",
        }
    );
}

#[test]
fn label_of_every_code_is_its_stable_name() {
    // Arrange & Act
    let labels: Vec<&str> = EVERY_CODE.iter().map(|(code, ..)| code.label()).collect();

    // Assert
    let expected: Vec<&str> = EVERY_CODE.iter().map(|(_, label, ..)| *label).collect();
    assert_eq!(labels, expected);
}

#[test]
fn needs_person_of_every_code_is_whether_only_the_owner_can_fix_it() {
    // Arrange & Act
    let needs: Vec<bool> = EVERY_CODE
        .iter()
        .map(|(code, ..)| code.needs_person())
        .collect();

    // Assert
    let expected: Vec<bool> = EVERY_CODE.iter().map(|(.., needs)| *needs).collect();
    assert_eq!(needs, expected);
}

#[test]
fn severity_of_every_code_is_the_design_s() {
    // Arrange & Act
    let severities: Vec<Severity> = EVERY_CODE
        .iter()
        .map(|(code, ..)| code.severity())
        .collect();

    // Assert
    let expected: Vec<Severity> = EVERY_CODE
        .iter()
        .map(|(_, _, severity, _)| *severity)
        .collect();
    assert_eq!(severities, expected);
}
