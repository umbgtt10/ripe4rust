// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::date;
use crate::support::releases::version;
use crate::support::surveyed::at;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::diagnostics::severity::Severity;
use ripe4rust_core::inputs::toolchain_file::ToolchainFile;
use ripe4rust_core::inputs::unparsed_file::UnparsedFile;
use ripe4rust_core::planning::findings::Findings;
use ripe4rust_core::planning::pins_and_policy::hold::Hold;
use ripe4rust_core::planning::pins_and_policy::pin::Pin;

#[test]
fn pin_for_absent_crate_is_a_warning_at_the_pin() {
    // Arrange
    let pin = Pin {
        name: String::from("gone"),
        reason: String::from("a migration"),
        review: None,
        until: None,
        hold: Hold::Line,
        at: at("Cargo.toml", 44),
    };

    // Act
    let diagnostic = Findings.pin_for_absent_crate(&pin);

    // Assert
    assert_eq!(
        (
            diagnostic.code,
            diagnostic.severity,
            diagnostic.at,
            diagnostic.what
        ),
        (
            DiagnosticCode::PinForAbsentCrate,
            Severity::Warning,
            Some(at("Cargo.toml", 44)),
            String::from("the pin on `gone` names a crate no longer in the tree")
        )
    );
}

#[test]
fn registry_cache_old_names_both_days() {
    // Arrange & Act
    let diagnostic = Findings.registry_cache_old(date("2026-11-20"), date("2026-11-26"));

    // Assert
    assert_eq!(diagnostic.code, DiagnosticCode::RegistryCacheOld);
    assert_eq!(
        diagnostic.what,
        "the registry's answers are those cached on 2026-11-20, not on 2026-11-26: what was published since is not in the plan"
    );
}

#[test]
fn source_unparsed_names_the_file_and_why() {
    // Arrange
    let file = UnparsedFile {
        file: String::from("src/broken.rs"),
        at: at("src/broken.rs", 12),
        detail: String::from("expected `;`"),
    };

    // Act
    let diagnostic = Findings.source_unparsed(&file);

    // Assert
    assert_eq!(diagnostic.code, DiagnosticCode::SourceUnparsed);
    assert_eq!(
        diagnostic.at.map(|at| at.to_string()),
        Some(String::from("src/broken.rs:12"))
    );
    assert_eq!(
        diagnostic.what,
        "`src/broken.rs` could not be parsed (expected `;`): the footprint of the crates it names may be low"
    );
}

#[test]
fn source_unscanned_is_a_warning_that_nothing_of_the_sources_is_known() {
    // Arrange & Act
    let diagnostic = Findings.source_unscanned();

    // Assert
    assert_eq!(
        (diagnostic.code, diagnostic.severity, diagnostic.at),
        (DiagnosticCode::SourceUnscanned, Severity::Warning, None)
    );
    assert_eq!(
        diagnostic.what,
        "not one of the workspace's source files could be parsed, so no breaking unit's footprint or exposure is known: none is ripe by its footprint (rule 11), and one a published member uses is asked (rule 8)"
    );
    assert_eq!(
        diagnostic.action,
        "run `cargo check`: where it fails, fix what it names and plan again; where it builds, the sources use syntax ripe4rust's parser does not read -- report it at https://github.com/umbgtt10/ripe4rust/issues --, and size each breaking unit's migration by hand meanwhile"
    );
}

#[test]
fn target_unknown_needs_a_person() {
    // Arrange & Act
    let diagnostic = Findings.target_unknown("thumbv7em-none-eabihff", Some(at("Cargo.toml", 33)));

    // Assert
    assert_eq!(diagnostic.code, DiagnosticCode::TargetUnknown);
    assert!(diagnostic.needs_person);
    assert_eq!(
        diagnostic.what,
        "`thumbv7em-none-eabihff` in `validate-beyond-gates` is a target the toolchain does not know"
    );
}

#[test]
fn toolchain_releases_unknown_is_a_notice_at_the_channel() {
    // Arrange & Act
    let diagnostic = Findings.toolchain_releases_unknown(at("rust-toolchain.toml", 9));

    // Assert
    assert_eq!(
        (diagnostic.code, diagnostic.severity, diagnostic.at),
        (
            DiagnosticCode::ToolchainReleasesUnknown,
            Severity::Notice,
            Some(at("rust-toolchain.toml", 9))
        )
    );
    assert_eq!(
        diagnostic.what,
        "the toolchain's releases are not read yet: no toolchain unit is offered, and the versions a newer toolchain would release are not named"
    );
    assert_eq!(
        diagnostic.action,
        "nothing is needed; until a release of ripe4rust reads them, compare `rust-toolchain.toml`'s channel with the stable releases by hand"
    );
}

#[test]
fn toolchain_unpinned_of_a_channel_naming_no_release_quotes_it() {
    // Arrange
    let file = ToolchainFile {
        channel: String::from("stable"),
        at: at("rust-toolchain.toml", 2),
    };

    // Act
    let diagnostic = Findings.toolchain_unpinned(Some(&file), &version("1.99.1"));

    // Assert
    assert_eq!(
        (diagnostic.code, diagnostic.severity, diagnostic.at),
        (
            DiagnosticCode::ToolchainUnpinned,
            Severity::Notice,
            Some(at("rust-toolchain.toml", 2))
        )
    );
    assert_eq!(
        diagnostic.what,
        "`rust-toolchain.toml`'s channel, `stable`, pins no stable release: there is no toolchain unit, and the versions are chosen for rustc 1.99.1"
    );
}

#[test]
fn toolchain_unpinned_without_a_file_says_so() {
    // Arrange & Act
    let diagnostic = Findings.toolchain_unpinned(None, &version("1.99.1"));

    // Assert
    assert_eq!(diagnostic.at, None);
    assert_eq!(
        diagnostic.what,
        "the workspace has no `rust-toolchain.toml`: there is no toolchain unit, and the versions are chosen for rustc 1.99.1"
    );
}
