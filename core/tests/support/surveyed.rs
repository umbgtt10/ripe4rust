// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::date;
use crate::support::releases::needing;
use crate::support::releases::release;
use crate::support::releases::version;
use crate::support::units::dependency;
use crate::support::units::moved;
use ripe4rust_core::inputs::crate_use::CrateUse;
use ripe4rust_core::inputs::locked_package::LockedPackage;
use ripe4rust_core::inputs::member::Member;
use ripe4rust_core::inputs::release::Release;
use ripe4rust_core::inputs::resolved_dependency::ResolvedDependency;
use ripe4rust_core::inputs::scan::Scan;
use ripe4rust_core::inputs::workspace::Workspace;
use ripe4rust_core::inputs::written_line::WrittenLine;
use ripe4rust_core::planning::pins_and_policy::policy::Policy;
use ripe4rust_core::planning::pins_and_policy::thresholds::Thresholds;
use ripe4rust_core::planning::surveyed::Surveyed;
use ripe4rust_core::planning::units::choice::histories::Histories;
use ripe4rust_core::planning::units::choice::release_history::ReleaseHistory;
use ripe4rust_core::planning::units::dependency_moves::DependencyMoves;
use ripe4rust_core::planning::units::direct_dependency::DirectDependency;
use ripe4rust_core::values::location::Location;
use ripe4rust_core::values::partial_version::PartialVersion;
use ripe4rust_core::values::requirement::Requirement;
use ripe4rust_core::values::version::Version;

pub fn at(file: &str, line: u32) -> Location {
    Location {
        file: String::from(file),
        line,
    }
}

pub fn direct() -> Vec<DirectDependency> {
    let users = |members: &[&str]| members.iter().map(|member| String::from(*member)).collect();
    vec![
        DirectDependency {
            members: users(&["app"]),
            ..dependency("gadget", "0.13.1")
        },
        DirectDependency {
            members: users(&["app", "app-core"]),
            ..dependency("widget", "1.4.0")
        },
    ]
}

pub fn histories(surveyed: &Surveyed) -> Histories {
    Histories::new(
        surveyed
            .releases
            .iter()
            .map(|(name, releases)| {
                (
                    name.clone(),
                    ReleaseHistory::new(releases.clone(), Version::line),
                )
            })
            .collect(),
    )
}

fn locked(name: &str, at: &str, dependencies: &[(&str, &str)]) -> LockedPackage {
    LockedPackage {
        name: String::from(name),
        version: version(at),
        dependencies: dependencies
            .iter()
            .map(|(name, at)| resolved(name, at))
            .collect(),
    }
}

fn member(name: &str, dependencies: &[(&str, &str)]) -> Member {
    Member {
        name: String::from(name),
        rust_version: Some(PartialVersion::parse("1.85").expect("a rust-version")),
        dependencies: dependencies
            .iter()
            .map(|(name, at)| resolved(name, at))
            .collect(),
    }
}

fn policy() -> Policy {
    Policy {
        at: Some(at("Cargo.toml", 30)),
        gates: vec![String::from("just stage1"), String::from("just stage2")],
        published: vec![String::from("app-core")],
        validate_beyond_gates: Vec::new(),
        freeze_until: None,
        exposure: Vec::new(),
        thresholds: Thresholds {
            young_days: 30,
            young_releases: 3,
            toolchain_young_days: 0,
            stale_months: 6,
            small_footprint: 10,
            moderate_footprint: 100,
            hold_days: 14,
        },
        pins: Vec::new(),
    }
}

fn releases() -> Vec<(String, Vec<Release>)> {
    vec![
        (
            String::from("gadget"),
            vec![
                release("0.13.0", "2025-01-01"),
                release("0.13.1", "2026-07-01"),
                release("0.14.0", "2026-11-20"),
            ],
        ),
        (
            String::from("gizmo"),
            vec![
                release("2.7.5", "2025-05-01"),
                release("2.7.6", "2026-11-01"),
            ],
        ),
        (String::from("libc"), vec![release("0.2.177", "2026-01-01")]),
        (
            String::from("widget"),
            vec![
                release("1.0.0", "2024-01-10"),
                release("1.4.0", "2025-03-01"),
                release("1.4.1", "2026-10-01"),
                release("2.0.0", "2026-01-05"),
                release("2.0.1", "2026-02-01"),
                needing("2.1.0", "2026-06-01", "1.88"),
            ],
        ),
    ]
}

fn resolved(name: &str, at: &str) -> ResolvedDependency {
    ResolvedDependency {
        name: String::from(name),
        version: version(at),
        requirement: Requirement::parse(at).expect("a requirement"),
    }
}

fn scan() -> Scan {
    Scan {
        uses: vec![
            CrateUse {
                name: String::from("gadget"),
                locations: vec![at("src/net.rs", 4)],
                exposed_in: Vec::new(),
            },
            CrateUse {
                name: String::from("widget"),
                locations: vec![
                    at("src/lib.rs", 9),
                    at("src/lib.rs", 3),
                    at("src/main.rs", 2),
                ],
                exposed_in: vec![String::from("app-core")],
            },
        ],
        unparsed: Vec::new(),
        unread_apis: Vec::new(),
    }
}

pub fn surveyed() -> Surveyed {
    Surveyed {
        today: date("2026-11-26"),
        registry_read: date("2026-11-26"),
        fingerprint: String::from("9f2c41d0"),
        policy: policy(),
        workspace: workspace(),
        direct: direct(),
        pinned: Some(version("1.99.1")),
        channel_at: Some(at("rust-toolchain.toml", 9)),
        toolchain: version("1.99.1"),
        toolchain_releases: vec![
            release("1.99.0", "2026-08-27"),
            release("1.99.1", "2026-09-10"),
            release("1.100.0", "2026-10-08"),
        ],
        releases: releases(),
        advisories: Vec::new(),
        dependency_moves: vec![
            DependencyMoves {
                dependency: direct()[0].clone(),
                moves: Vec::new(),
            },
            DependencyMoves {
                dependency: direct()[1].clone(),
                moves: vec![
                    moved("widget", "1.4.0", "1.4.1"),
                    moved("gizmo", "2.7.5", "2.7.6"),
                ],
            },
        ],
        workspace_moves: vec![
            moved("gizmo", "2.7.5", "2.7.6"),
            moved("libc", "0.2.177", "0.2.178"),
            moved("widget", "1.4.0", "1.4.1"),
        ],
        repositories: Vec::new(),
        reached: Vec::new(),
        scan: Some(scan()),
        requirement_lines: vec![
            (String::from("gadget"), vec![at("Cargo.toml", 22)]),
            (String::from("widget"), vec![at("Cargo.toml", 21)]),
        ],
        rust_version_lines: vec![
            (String::from("app"), at("Cargo.toml", 5)),
            (String::from("app-core"), at("Cargo.toml", 5)),
        ],
        written: vec![
            written(at("Cargo.toml", 5), "rust-version", "1.85"),
            written(at("Cargo.toml", 21), "widget", "1.4"),
            written(at("Cargo.toml", 22), "gadget", "0.13"),
            written(at("rust-toolchain.toml", 9), "channel", "1.99.1"),
        ],
        diagnostics: Vec::new(),
    }
}

pub fn toolchain(surveyed: &Surveyed) -> ReleaseHistory {
    ReleaseHistory::new(surveyed.toolchain_releases.clone(), Version::toolchain_line)
}

fn workspace() -> Workspace {
    Workspace {
        members: vec![
            member("app", &[("widget", "1.4.0"), ("gadget", "0.13.1")]),
            member("app-core", &[("widget", "1.4.0")]),
        ],
        lock: vec![
            locked("gadget", "0.13.1", &[]),
            locked("gizmo", "2.7.5", &[]),
            locked("libc", "0.2.177", &[]),
            locked("widget", "1.4.0", &[("gizmo", "2.7.5")]),
        ],
    }
}

pub fn written(at: Location, key: &str, value: &str) -> WrittenLine {
    WrittenLine {
        at,
        key: String::from(key),
        value: String::from(value),
    }
}
