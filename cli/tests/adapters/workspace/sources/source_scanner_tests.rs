// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::memory_files::MemoryFiles;
use crate::support::outputs::failed;
use crate::support::replay_runner::ReplayRunner;
use crate::support::workspace::LAYOUT;
use crate::support::workspace::files;
use crate::support::workspace::layout;
use cargo_ripe4rust::adapters::workspace::cargo_layout::CargoLayout;
use cargo_ripe4rust::adapters::workspace::sources::member_findings::MemberFindings;
use cargo_ripe4rust::adapters::workspace::sources::source_scanner::SourceScanner;
use ripe4rust_core::inputs::crate_use::CrateUse;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::scan::Scan;
use ripe4rust_core::inputs::unparsed_file::UnparsedFile;
use ripe4rust_core::traits::source_scan::SourceScan;
use ripe4rust_core::values::location::Location;
use std::path::PathBuf;

fn at(file: &str, line: u32) -> Location {
    Location {
        file: String::from(file),
        line,
    }
}

fn found(named: &[(&str, Location)], exposed: &[&str]) -> MemberFindings {
    MemberFindings {
        named: named
            .iter()
            .map(|(package, at)| (String::from(*package), at.clone()))
            .collect(),
        exposed: exposed
            .iter()
            .map(|package| String::from(*package))
            .collect(),
        unparsed: Vec::new(),
        api_unread: false,
        parsed: 1,
    }
}

fn unparsed(file: &str, line: u32) -> UnparsedFile {
    UnparsedFile {
        file: String::from(file),
        at: at(file, line),
        detail: String::from("expected `;`"),
    }
}

#[test]
fn scan_of_a_library_that_does_not_parse_names_it_and_its_api_unread() {
    // Arrange
    let runner = layout();
    let source = CargoLayout::new(&runner, PathBuf::from("/work/ws"));
    let files = files().with("/work/ws/app-core/src/lib.rs", "pub fn digits(\n");

    // Act
    let scan = SourceScanner::new(&source, &files).scan();

    // Assert
    assert_eq!(
        scan.map(|scan| scan.map(|scan| (scan.unparsed.len(), scan.unread_apis))),
        Ok(Some((1, vec![String::from("app-core")])))
    );
}

#[test]
fn scan_of_sources_none_of_which_parses_is_not_known() {
    // Arrange
    let runner = layout();
    let source = CargoLayout::new(&runner, PathBuf::from("/work/ws"));
    let files = MemoryFiles::new()
        .with("/work/ws/app/src/main.rs", "fn main() {")
        .with("/work/ws/app-core/src/lib.rs", "pub fn digits(");

    // Act
    let scan = SourceScanner::new(&source, &files).scan();

    // Assert
    assert_eq!(scan, Ok(None));
}

#[test]
fn scan_of_the_recorded_workspace_is_every_crate_named_where_and_where_exposed() {
    // Arrange
    let runner = layout();
    let source = CargoLayout::new(&runner, PathBuf::from("/work/ws"));
    let files = files();

    // Act
    let scan = SourceScanner::new(&source, &files).scan();

    // Assert
    assert_eq!(
        scan,
        Ok(Some(Scan {
            uses: vec![
                CrateUse {
                    name: String::from("itertools"),
                    locations: vec![at("app/src/main.rs", 1)],
                    exposed_in: Vec::new(),
                },
                CrateUse {
                    name: String::from("itoa"),
                    locations: vec![
                        at("app-core/src/lib.rs", 3),
                        at("app-core/src/lib.rs", 6),
                        at("app-core/src/lib.rs", 7),
                    ],
                    exposed_in: vec![String::from("app-core")],
                },
                CrateUse {
                    name: String::from("smallvec"),
                    locations: vec![at("app/src/main.rs", 2), at("app/src/main.rs", 5)],
                    exposed_in: Vec::new(),
                },
            ],
            unparsed: Vec::new(),
            unread_apis: Vec::new(),
        }))
    );
}

#[test]
fn scan_without_a_layout_is_cargo_s_failure() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &LAYOUT,
        failed(
            101,
            include_str!("../../../fixtures/cargo/locate-nowhere.txt"),
        ),
    );
    let source = CargoLayout::new(&runner, PathBuf::from("/work/nowhere"));

    // Act
    let scan = SourceScanner::new(&source, &MemoryFiles::new()).scan();

    // Assert
    assert!(
        matches!(scan, Err(PortFailure::CargoMetadataFailed { .. })),
        "{scan:?}"
    );
}

#[test]
fn unparsed_of_members_is_each_file_once_in_order() {
    // Arrange
    let runner = layout();
    let source = CargoLayout::new(&runner, PathBuf::from("/work/ws"));
    let files = MemoryFiles::new();
    let members = [
        (
            String::from("app"),
            MemberFindings {
                unparsed: vec![unparsed("b.rs", 2), unparsed("a.rs", 9)],
                ..found(&[], &[])
            },
        ),
        (
            String::from("tests"),
            MemberFindings {
                unparsed: vec![unparsed("a.rs", 9)],
                ..found(&[], &[])
            },
        ),
    ];

    // Act
    let unparsed_files = SourceScanner::new(&source, &files).unparsed(&members);

    // Assert
    assert_eq!(
        unparsed_files,
        vec![unparsed("a.rs", 9), unparsed("b.rs", 2)]
    );
}

#[test]
fn uses_of_members_gather_each_crate_s_lines_and_the_members_exposing_it() {
    // Arrange
    let runner = layout();
    let source = CargoLayout::new(&runner, PathBuf::from("/work/ws"));
    let files = MemoryFiles::new();
    let members = [
        (
            String::from("app"),
            found(&[("itoa", at("b.rs", 2)), ("itoa", at("a.rs", 4))], &[]),
        ),
        (
            String::from("app-core"),
            found(&[("itoa", at("a.rs", 4))], &["itoa", "url"]),
        ),
    ];

    // Act
    let uses = SourceScanner::new(&source, &files).uses(&members);

    // Assert
    assert_eq!(
        uses,
        vec![
            CrateUse {
                name: String::from("itoa"),
                locations: vec![at("a.rs", 4), at("b.rs", 2)],
                exposed_in: vec![String::from("app-core")],
            },
            CrateUse {
                name: String::from("url"),
                locations: Vec::new(),
                exposed_in: vec![String::from("app-core")],
            },
        ]
    );
}
