// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::diagnostics::diagnostic::Diagnostic;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::diagnostics::severity::Severity;
use ripe4rust_core::inputs::port_failure::PortFailure;

fn detail() -> String {
    String::from("connection refused")
}

#[test]
fn diagnostic_of_an_unrecognised_line_quotes_it_and_says_where_to_report_it() {
    // Arrange
    let failure = PortFailure::CargoUpdateUnrecognised {
        line: String::from("Frobbing clap v4.6.6"),
    };

    // Act
    let diagnostic = failure.diagnostic();

    // Assert
    assert_eq!(
        diagnostic,
        Diagnostic {
            severity: Severity::Error,
            code: DiagnosticCode::CargoUpdateUnrecognised,
            at: None,
            what: String::from(
                "`cargo update --dry-run` printed a line ripe4rust does not know: `Frobbing clap v4.6.6`"
            ),
            action: String::from(
                "report it at https://github.com/umbgtt10/ripe4rust/issues with the line; no plan is made that could leave a move out"
            ),
            needs_person: false,
        }
    );
}

#[test]
fn diagnostic_of_every_failure_is_an_error_of_its_own_code() {
    // Arrange
    let failures = [
        (
            PortFailure::CargoDenyFailed { detail: detail() },
            DiagnosticCode::CargoDenyFailed,
        ),
        (
            PortFailure::CargoDenyMissing,
            DiagnosticCode::CargoDenyMissing,
        ),
        (
            PortFailure::CargoMetadataFailed { detail: detail() },
            DiagnosticCode::CargoMetadataFailed,
        ),
        (
            PortFailure::CargoUpdateFailed { detail: detail() },
            DiagnosticCode::CargoUpdateFailed,
        ),
        (
            PortFailure::CargoUpdateUnrecognised {
                line: String::from("Frobbing clap v4.6.6"),
            },
            DiagnosticCode::CargoUpdateUnrecognised,
        ),
        (
            PortFailure::Internal {
                doing: String::from("reading the cache"),
                detail: detail(),
            },
            DiagnosticCode::Internal,
        ),
        (
            PortFailure::RegistryUnreachable { detail: detail() },
            DiagnosticCode::RegistryUnreachable,
        ),
        (
            PortFailure::ToolchainUnreachable { detail: detail() },
            DiagnosticCode::ToolchainUnreachable,
        ),
        (
            PortFailure::WorkspaceUnreadable {
                file: String::from("Cargo.toml"),
                detail: detail(),
            },
            DiagnosticCode::WorkspaceUnreadable,
        ),
    ];

    // Act
    let diagnostics: Vec<(DiagnosticCode, Severity)> = failures
        .iter()
        .map(|(failure, _)| failure.diagnostic())
        .map(|diagnostic| (diagnostic.code, diagnostic.severity))
        .collect();

    // Assert
    assert_eq!(
        diagnostics,
        failures
            .iter()
            .map(|(_, code)| (*code, Severity::Error))
            .collect::<Vec<_>>()
    );
}
