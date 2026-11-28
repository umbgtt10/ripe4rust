// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::fakes::fake_ports::FakePorts;
use crate::support::policy_trees::entry;
use crate::support::policy_trees::texts;
use crate::support::policy_trees::tree;
use crate::support::recorded::edge;
use crate::support::releases::date;
use crate::support::releases::release;
use crate::support::releases::version;
use crate::support::surveyed::at;
use crate::support::surveyed::direct;
use crate::support::surveyed::surveyed;
use ripe4rust_core::diagnostics::diagnostic::Diagnostic;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::inputs::git::git_entry::GitEntry;
use ripe4rust_core::inputs::git::git_move::GitMove;
use ripe4rust_core::inputs::git::git_source::GitSource;
use ripe4rust_core::inputs::git::moves::Moves;
use ripe4rust_core::inputs::lock_move::LockMove;
use ripe4rust_core::inputs::policy_value::PolicyValue;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::source_scan::unparsed_file::UnparsedFile;
use ripe4rust_core::inputs::toolchain_file::ToolchainFile;
use ripe4rust_core::planning::pins_and_policy::hold::Hold;
use ripe4rust_core::planning::pins_and_policy::pin::Pin;
use ripe4rust_core::planning::pins_and_policy::release_condition::ReleaseCondition;
use ripe4rust_core::planning::survey::Survey;
use ripe4rust_core::planning::units::crate_repository::CrateRepository;
use ripe4rust_core::planning::units::source_moves::SourceMoves;
use ripe4rust_core::values::requirement::Requirement;

fn codes(diagnostics: &[Diagnostic]) -> Vec<DiagnosticCode> {
    diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code)
        .collect()
}

fn pin(name: &str) -> Pin {
    Pin {
        name: String::from(name),
        reason: String::from("a migration"),
        review: Some(date("2027-01-31")),
        until: None,
        hold: Hold::Line,
        at: at("Cargo.toml", 41),
    }
}

#[test]
fn moves_ask_cargo_for_each_direct_dependency() {
    // Arrange
    let fakes = FakePorts::new();

    // Act
    let moves = Survey::new(fakes.ports()).moves(&direct());

    // Assert
    assert_eq!(moves, Ok(surveyed().dependency_moves));
}

#[test]
fn policy_reads_the_files_tree() {
    // Arrange
    let fakes = FakePorts::new();

    // Act
    let policy = Survey::new(fakes.ports()).policy();

    // Assert
    assert_eq!(policy.map(|reading| reading.value), Ok(surveyed().policy));
}

#[test]
fn reached_asks_the_metadata_for_each_target() {
    // Arrange
    let mut fakes = FakePorts::new();
    fakes.metadata.reached = vec![(
        String::from("thumbv7em-none-eabihf"),
        vec![String::from("gizmo")],
    )];
    let mut policy = surveyed().policy;
    policy.validate_beyond_gates = vec![String::from("thumbv7em-none-eabihf")];

    // Act
    let reached = Survey::new(fakes.ports()).reached(&policy);

    // Assert
    assert_eq!(
        reached,
        Ok(vec![(
            String::from("thumbv7em-none-eabihf"),
            vec![String::from("gizmo")]
        )])
    );
}

#[test]
fn releases_ask_the_index_once_for_every_name_in_order() {
    // Arrange
    let fakes = FakePorts::new();

    // Act
    let _ = Survey::new(fakes.ports()).releases(&surveyed().workspace, &surveyed().policy);

    // Assert
    assert_eq!(
        *fakes.index.asked.borrow(),
        vec![
            ["gadget", "gizmo", "libc", "widget"]
                .map(String::from)
                .to_vec()
        ]
    );
}

#[test]
fn releases_read_every_locked_crate_and_every_crate_a_pin_waits_for() {
    // Arrange
    let mut fakes = FakePorts::new();
    fakes.index.releases.push((
        String::from("widget-tools"),
        vec![release("0.32.0", "2026-11-01")],
    ));
    let mut policy = surveyed().policy;
    policy.pins = vec![Pin {
        until: Some(ReleaseCondition::Release {
            name: String::from("widget-tools"),
            requirement: Requirement::parse(">=0.32").expect("a requirement"),
        }),
        ..pin("widget")
    }];

    // Act
    let releases = Survey::new(fakes.ports()).releases(&surveyed().workspace, &policy);

    // Assert
    assert_eq!(
        releases.map(|read| read.into_iter().map(|(name, _)| name).collect::<Vec<_>>()),
        Ok(vec![
            String::from("gadget"),
            String::from("gizmo"),
            String::from("libc"),
            String::from("widget"),
            String::from("widget-tools"),
        ])
    );
}

#[test]
fn repositories_keep_those_the_registry_knows() {
    // Arrange
    let mut fakes = FakePorts::new();
    fakes.repositories.known = vec![(
        String::from("widget"),
        String::from("https://example.org/widget"),
    )];

    // Act
    let repositories = Survey::new(fakes.ports()).repositories(&direct());

    // Assert
    assert_eq!(
        repositories,
        Ok(vec![CrateRepository {
            name: String::from("widget"),
            repository: String::from("https://example.org/widget"),
        }])
    );
}

#[test]
fn requirement_lines_are_read_for_each_direct_dependency() {
    // Arrange
    let fakes = FakePorts::new();

    // Act
    let lines = Survey::new(fakes.ports()).requirement_lines(&direct());

    // Assert
    assert_eq!(lines, Ok(surveyed().requirement_lines));
}

#[test]
fn rust_version_lines_are_read_for_each_member() {
    // Arrange
    let fakes = FakePorts::new();

    // Act
    let lines = Survey::new(fakes.ports()).rust_version_lines(&surveyed().workspace);

    // Assert
    assert_eq!(lines, Ok(surveyed().rust_version_lines));
}

#[test]
fn source_moves_ask_cargo_for_each_git_source_moving_by_its_first_crate_in_the_lock() {
    // Arrange
    let mut fakes = FakePorts::new();
    let pulled = Moves {
        registry: vec![LockMove {
            name: String::from("itoa"),
            from: Some(version("1.0.10")),
            to: Some(version("1.0.19")),
        }],
        git: Vec::new(),
    };
    fakes.lock_moves.of_source = vec![(String::from("gitextra"), pulled.clone())];
    let git = |name: &str, from: Option<&str>, to: Option<&str>| GitMove {
        name: String::from(name),
        source: String::from("https://github.com/example/upstream?branch=main"),
        version: version("0.1.0"),
        from: from.map(String::from),
        to: to.map(String::from),
    };
    let moves = [
        git("gitmacros", Some("9625c3d2"), Some("b52bc079")),
        git("gitcore", None, Some("b52bc079")),
        git("gitextra", Some("9625c3d2"), None),
    ];

    // Act
    let sources = Survey::new(fakes.ports()).source_moves(&moves);

    // Assert
    assert_eq!(
        sources,
        Ok(vec![SourceMoves {
            source: GitSource {
                source: String::from("https://github.com/example/upstream?branch=main"),
                from: String::from("9625c3d2"),
                to: String::from("b52bc079"),
            },
            name: String::from("gitextra"),
            version: version("0.1.0"),
            crates: vec![
                String::from("gitcore"),
                String::from("gitextra"),
                String::from("gitmacros")
            ],
            moves: pulled.registry,
        }])
    );
}

#[test]
fn survey_carries_the_lock_s_edges_as_the_files_read_them() {
    // Arrange
    let mut fakes = FakePorts::new();
    fakes.files.edges = vec![edge(("gadget", "0.13.1"), ("gizmo", "2.7.5"))];

    // Act
    let survey = Survey::new(fakes.ports()).survey();

    // Assert
    assert_eq!(
        survey.map(|surveyed| surveyed.edges),
        Ok(vec![edge(("gadget", "0.13.1"), ("gizmo", "2.7.5"))])
    );
}

#[test]
fn survey_of_a_failing_port_stops_with_what_was_found_so_far() {
    // Arrange
    let mut fakes = FakePorts::new();
    fakes.files.policy = tree(Vec::new());
    fakes.metadata.failure = Some(PortFailure::CargoMetadataFailed {
        detail: String::from("no manifest"),
    });

    // Act
    let survey = Survey::new(fakes.ports()).survey();

    // Assert
    assert_eq!(
        survey.map_err(|failure| codes(&failure.diagnostics)),
        Err(vec![
            DiagnosticCode::GatesUndeclared,
            DiagnosticCode::CargoMetadataFailed
        ])
    );
}

#[test]
fn survey_of_a_policy_in_error_stops_before_any_other_port() {
    // Arrange
    let mut fakes = FakePorts::new();
    fakes.files.policy = tree(vec![
        entry("gates", texts(&["just stage1"]), 31),
        entry("young-dyas", PolicyValue::Integer(3), 32),
    ]);
    fakes.advisories.failure = Some(PortFailure::CargoDenyMissing);

    // Act
    let survey = Survey::new(fakes.ports()).survey();

    // Assert
    assert_eq!(
        survey.map_err(|failure| codes(&failure.diagnostics)),
        Err(vec![DiagnosticCode::PolicyUnknownKey])
    );
}

#[test]
fn survey_of_a_scan_that_failed_as_a_whole_carries_its_warning() {
    // Arrange
    let mut fakes = FakePorts::new();
    fakes.scan.scan = None;

    // Act
    let survey = Survey::new(fakes.ports()).survey().expect("a survey");

    // Assert
    assert_eq!(survey.scan, None);
    assert_eq!(
        codes(&survey.diagnostics),
        vec![DiagnosticCode::SourceUnscanned]
    );
}

#[test]
fn survey_of_an_unknown_target_stops_the_plan() {
    // Arrange
    let mut fakes = FakePorts::new();
    fakes.files.policy = tree(vec![
        entry("gates", texts(&["just stage1"]), 31),
        entry(
            "validate-beyond-gates",
            texts(&["thumbv7em-none-eabihff"]),
            33,
        ),
    ]);

    // Act
    let survey = Survey::new(fakes.ports()).survey();

    // Assert
    assert_eq!(
        survey.map_err(|failure| codes(&failure.diagnostics)),
        Err(vec![DiagnosticCode::TargetUnknown])
    );
}

#[test]
fn survey_of_an_unreadable_policy_stops_at_once() {
    // Arrange
    let mut fakes = FakePorts::new();
    fakes.files.failure = Some(PortFailure::WorkspaceUnreadable {
        file: String::from("Cargo.toml"),
        detail: String::from("permission denied"),
    });

    // Act
    let survey = Survey::new(fakes.ports()).survey();

    // Assert
    assert_eq!(
        survey.map_err(|failure| codes(&failure.diagnostics)),
        Err(vec![DiagnosticCode::WorkspaceUnreadable])
    );
}

#[test]
fn survey_of_the_fakes_is_the_fixture() {
    // Arrange
    let fakes = FakePorts::new();

    // Act
    let survey = Survey::new(fakes.ports()).survey();

    // Assert
    assert_eq!(survey, Ok(surveyed()));
}

#[test]
fn toolchain_of_a_channel_pinning_no_release_is_rustc_s_with_a_notice() {
    // Arrange
    let mut fakes = FakePorts::new();
    fakes.files.toolchain = Some(ToolchainFile {
        channel: String::from("stable"),
        at: at("rust-toolchain.toml", 2),
    });
    fakes.toolchain.rustc = version("1.98.0");

    // Act
    let toolchain = Survey::new(fakes.ports())
        .toolchain(&surveyed().policy)
        .expect("a toolchain");

    // Assert
    assert_eq!(toolchain.value.pinned, None);
    assert_eq!(toolchain.value.in_use, version("1.98.0"));
    assert_eq!(toolchain.value.releases, Vec::new());
    assert_eq!(
        codes(&toolchain.diagnostics),
        vec![DiagnosticCode::ToolchainUnpinned]
    );
}

#[test]
fn toolchain_of_a_channel_pinning_no_release_reads_no_releases() {
    // Arrange
    let mut fakes = FakePorts::new();
    fakes.files.toolchain = None;
    fakes.toolchain.releases = Err(PortFailure::ToolchainUnreachable {
        detail: String::from("Could not resolve host: static.rust-lang.org"),
    });

    // Act
    let toolchain = Survey::new(fakes.ports())
        .toolchain(&surveyed().policy)
        .map(|toolchain| toolchain.value.releases);

    // Assert
    assert_eq!(toolchain, Ok(Vec::new()));
}

#[test]
fn toolchain_of_a_pinned_channel_is_its_release() {
    // Arrange
    let fakes = FakePorts::new();

    // Act
    let toolchain = Survey::new(fakes.ports())
        .toolchain(&surveyed().policy)
        .expect("a toolchain");

    // Assert
    assert_eq!(toolchain.value.pinned, Some(version("1.99.1")));
    assert_eq!(toolchain.value.in_use, version("1.99.1"));
    assert_eq!(toolchain.value.releases.len(), 3);
    assert_eq!(toolchain.diagnostics, Vec::new());
}

#[test]
fn toolchain_of_releases_that_cannot_be_read_is_their_failure() {
    // Arrange
    let mut fakes = FakePorts::new();
    let failure = PortFailure::ToolchainUnreachable {
        detail: String::from("Could not resolve host: static.rust-lang.org"),
    };
    fakes.toolchain.releases = Err(failure.clone());

    // Act
    let toolchain = Survey::new(fakes.ports())
        .toolchain(&surveyed().policy)
        .map(|toolchain| toolchain.value);

    // Assert
    assert_eq!(toolchain, Err(failure));
}

#[test]
fn warnings_name_absent_pins_an_old_cache_and_unparsed_files() {
    // Arrange
    let fakes = FakePorts::new();
    let mut surveyed = surveyed();
    surveyed.policy.pins = vec![pin("gone"), pin("rust"), pin("widget")];
    let mut scan = surveyed.scan.clone().expect("a scan");
    scan.unparsed = vec![UnparsedFile {
        file: String::from("src/broken.rs"),
        at: at("src/broken.rs", 12),
        detail: String::from("expected `;`"),
    }];

    // Act
    let warnings = Survey::new(fakes.ports()).warnings(
        &surveyed.policy,
        &surveyed.workspace,
        &[],
        Some(&scan),
        date("2026-11-20"),
        surveyed.today,
    );

    // Assert
    assert_eq!(
        codes(&warnings),
        vec![
            DiagnosticCode::PinForAbsentCrate,
            DiagnosticCode::RegistryCacheOld,
            DiagnosticCode::SourceUnparsed,
        ]
    );
}

#[test]
fn warnings_of_a_pin_on_a_crate_of_a_git_source_are_none() {
    // Arrange
    let fakes = FakePorts::new();
    let mut surveyed = surveyed();
    surveyed.policy.pins = vec![pin("gitcore")];
    let git = [GitEntry {
        name: String::from("gitcore"),
        source: String::from("https://github.com/example/upstream?branch=main"),
        rev: String::from("9625c3d2"),
    }];

    // Act
    let warnings = Survey::new(fakes.ports()).warnings(
        &surveyed.policy,
        &surveyed.workspace,
        &git,
        surveyed.scan.as_ref(),
        surveyed.today,
        surveyed.today,
    );

    // Assert
    assert_eq!(warnings, Vec::new());
}
