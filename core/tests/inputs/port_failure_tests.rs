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
fn diagnostic_of_a_lock_out_of_date_says_to_bring_it_in_step_and_commit_it() {
    // Arrange & Act
    let diagnostic = PortFailure::LockOutdated {
        detail: String::from(
            "cannot update the lock file /work/ws/Cargo.lock because --locked was passed to prevent this",
        ),
    }
    .diagnostic();

    // Assert
    assert_eq!(
        (diagnostic.what, diagnostic.action),
        (
            String::from(
                "`Cargo.lock` no longer matches the workspace's manifests, and ripe4rust writes no lock: cannot update the lock file /work/ws/Cargo.lock because --locked was passed to prevent this"
            ),
            String::from(
                "bring the lock in step with the manifests -- `cargo update --workspace` writes it and moves no locked version they still allow --, commit it, then plan again"
            ),
        )
    );
}

#[test]
fn diagnostic_of_a_missing_lock_asks_the_owner_whether_to_commit_one() {
    // Arrange & Act
    let diagnostic = PortFailure::LockMissing {
        detail: String::from(
            "cannot create the lock file /work/ws/Cargo.lock because --locked was passed to prevent this",
        ),
    }
    .diagnostic();

    // Assert
    assert_eq!(
        (diagnostic.what, diagnostic.action, diagnostic.needs_person),
        (
            String::from(
                "the workspace has no `Cargo.lock`, and ripe4rust writes no lock: cannot create the lock file /work/ws/Cargo.lock because --locked was passed to prevent this"
            ),
            String::from(
                "ask the owner whether to commit a lock -- a library may leave it out on purpose --; with their word, `cargo generate-lockfile` writes it: commit it, then plan again"
            ),
            true
        )
    );
}

#[test]
fn diagnostic_of_a_plan_of_another_schema_says_to_plan_again() {
    // Arrange & Act
    let diagnostic = PortFailure::PlanSchemaUnknown {
        file: String::from("plan.json"),
        schema: String::from("2"),
    }
    .diagnostic();

    // Assert
    assert_eq!(
        (diagnostic.what, diagnostic.action),
        (
            String::from(
                "`plan.json` is a plan of schema 2, and this release of ripe4rust reads schema 1"
            ),
            String::from(
                "plan again with this release -- `cargo ripe4rust plan --format json > plan.json` -- and go on with the new plan"
            ),
        )
    );
}

#[test]
fn diagnostic_of_a_plan_that_does_not_read_says_to_write_it_again() {
    // Arrange & Act
    let diagnostic = PortFailure::PlanUnreadable {
        file: String::from("plan.json"),
        detail: String::from("it is not there"),
    }
    .diagnostic();

    // Assert
    assert_eq!(
        (diagnostic.what, diagnostic.action),
        (
            String::from("`plan.json` could not be read as a plan: it is not there"),
            String::from(
                "write the plan with `cargo ripe4rust plan --format json > plan.json`, and run the command again"
            ),
        )
    );
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
fn diagnostic_of_an_unusable_cache_says_how_to_move_it() {
    // Arrange & Act
    let diagnostic = PortFailure::RegistryCacheUnusable { detail: detail() }.diagnostic();

    // Assert
    assert_eq!(
        (diagnostic.what, diagnostic.action),
        (
            String::from("the registry's answers could not be cached: connection refused"),
            String::from(
                "make the cache directory readable and writable, or set `XDG_CACHE_HOME` (`LOCALAPPDATA` on Windows) to one that is, then plan again"
            ),
        )
    );
}

#[test]
fn diagnostic_of_curl_missing_says_where_curl_comes_from() {
    // Arrange & Act
    let diagnostic = PortFailure::CurlMissing.diagnostic();

    // Assert
    assert_eq!(
        (diagnostic.what, diagnostic.action),
        (
            String::from(
                "curl is not installed, and ripe4rust reads the registry and the toolchain's releases through it"
            ),
            String::from(
                "install curl -- Windows 10 and later, macOS and most Linux distributions ship it -- or put it on the PATH, then plan again; or plan with `--offline` to read the cache alone"
            ),
        )
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
        (PortFailure::CurlMissing, DiagnosticCode::CurlMissing),
        (
            PortFailure::Internal {
                doing: String::from("reading the cache"),
                detail: detail(),
            },
            DiagnosticCode::Internal,
        ),
        (
            PortFailure::LockMissing { detail: detail() },
            DiagnosticCode::LockMissing,
        ),
        (
            PortFailure::LockOutdated { detail: detail() },
            DiagnosticCode::LockOutdated,
        ),
        (
            PortFailure::PlanSchemaUnknown {
                file: String::from("plan.json"),
                schema: String::from("2"),
            },
            DiagnosticCode::PlanSchemaUnknown,
        ),
        (
            PortFailure::PlanUnreadable {
                file: String::from("plan.json"),
                detail: detail(),
            },
            DiagnosticCode::PlanUnreadable,
        ),
        (
            PortFailure::RegistryCacheUnusable { detail: detail() },
            DiagnosticCode::RegistryCacheUnusable,
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
