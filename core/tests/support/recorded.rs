// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::fakes::fake_files::FakeFiles;
use crate::support::policy_trees::entry;
use crate::support::policy_trees::texts;
use crate::support::policy_trees::tree;
use crate::support::releases::date;
use crate::support::releases::version;
use crate::support::surveyed::at;
use crate::support::surveyed::written;
use crate::support::units::added;
use crate::support::units::moved;
use ripe4rust_core::inputs::git::git_entry::GitEntry;
use ripe4rust_core::inputs::git::git_source::GitSource;
use ripe4rust_core::inputs::lock_entry::LockEntry;
use ripe4rust_core::inputs::lock_move::LockMove;
use ripe4rust_core::inputs::recorded_plan::RecordedPlan;
use ripe4rust_core::inputs::recorded_unit::RecordedUnit;
use ripe4rust_core::inputs::written_line::WrittenLine;
use ripe4rust_core::unit_kind::UnitKind;

pub const FINGERPRINT: &str = "9f2c41d0e3b5a7c9";
pub const LOCK_ONLY: &str = "git restore -- Cargo.lock";
pub const SOURCE: &str = "https://github.com/example/upstream?branch=main";

/// The workspace the recorded plan was made from, its lock and its lines,
/// as the files hold them unchanged.
pub fn base() -> FakeFiles {
    FakeFiles {
        fingerprint: String::from(FINGERPRINT),
        lock: lock(&[
            ("gadget", "0.13.1"),
            ("gizmo", "2.7.5"),
            ("libc", "0.2.177"),
            ("widget", "1.4.0"),
        ]),
        policy: tree(vec![entry("gates", texts(&["just stage1"]), 31)]),
        toolchain: None,
        requirement_lines: Vec::new(),
        rust_version_lines: Vec::new(),
        written: lines(),
        git: Vec::new(),
        failure: None,
    }
}

/// The base workspace with its lock and lines as given, its fingerprint
/// another.
pub fn changed(lock: Vec<LockEntry>, written: Vec<WrittenLine>) -> FakeFiles {
    FakeFiles {
        fingerprint: String::from("0000000000000001"),
        lock,
        written,
        ..base()
    }
}

pub fn entry_of(name: &str, at: &str) -> LockEntry {
    LockEntry {
        name: String::from(name),
        version: version(at),
    }
}

/// The git source of the plan with a git unit, every crate of it at `rev`.
pub fn git_at(rev: &str) -> Vec<GitEntry> {
    ["gitcore", "gitmacros"]
        .map(|name| GitEntry {
            name: String::from(name),
            source: String::from(SOURCE),
            rev: String::from(rev),
        })
        .to_vec()
}

pub fn lines() -> Vec<WrittenLine> {
    vec![
        written(at("Cargo.toml", 5), "rust-version", "1.85"),
        written(at("Cargo.toml", 21), "widget", "1.4"),
        written(at("Cargo.toml", 22), "gadget", "0.13"),
        written(at("rust-toolchain.toml", 9), "channel", "1.99.1"),
    ]
}

pub fn lines_with(changes: &[(u32, &str)]) -> Vec<WrittenLine> {
    lines()
        .into_iter()
        .map(|line| {
            changes
                .iter()
                .find(|(number, _)| *number == line.at.line)
                .map_or(line.clone(), |(_, value)| WrittenLine {
                    value: String::from(*value),
                    ..line
                })
        })
        .collect()
}

pub fn lock(entries: &[(&str, &str)]) -> Vec<LockEntry> {
    entries
        .iter()
        .map(|(name, at)| entry_of(name, at))
        .collect()
}

/// A plan in plan order: a breaking unit with an edit, the toolchain, a
/// compatible unit of two moves with its split, a second breaking unit, the
/// rest, and an advisory row.
pub fn plan() -> RecordedPlan {
    RecordedPlan {
        date: date("2026-11-26"),
        fingerprint: String::from(FINGERPRINT),
        lock: base().lock,
        lines: lines(),
        git: Vec::new(),
        units: vec![
            unit(
                UnitKind::Breaking,
                "gadget",
                vec![moved("gadget", "0.13.1", "0.14.0")],
                vec![written(at("Cargo.toml", 22), "gadget", "0.14.0")],
            ),
            RecordedUnit {
                from: Some(version("1.99.1")),
                to: Some(version("1.100.0")),
                revert: Some(String::from("git restore -- rust-toolchain.toml")),
                ..unit(
                    UnitKind::Toolchain,
                    "rust",
                    Vec::new(),
                    vec![written(at("rust-toolchain.toml", 9), "channel", "1.100.0")],
                )
            },
            RecordedUnit {
                splits: vec![
                    String::from("compatible:widget/widget"),
                    String::from("compatible:widget/gizmo"),
                ],
                ..unit(
                    UnitKind::Compatible,
                    "widget",
                    vec![
                        moved("widget", "1.4.0", "1.4.1"),
                        moved("gizmo", "2.7.5", "2.7.6"),
                    ],
                    Vec::new(),
                )
            },
            unit(
                UnitKind::Breaking,
                "widget",
                vec![moved("widget", "1.4.0", "2.0.1")],
                vec![written(at("Cargo.toml", 21), "widget", "2.0.1")],
            ),
            RecordedUnit {
                id: String::from("rest"),
                name: None,
                members: Vec::new(),
                ..unit(
                    UnitKind::Compatible,
                    "rest",
                    vec![moved("libc", "0.2.177", "0.2.178")],
                    Vec::new(),
                )
            },
            RecordedUnit {
                id: String::from("advisory:gizmo@2.7.5"),
                urgent: true,
                revert: None,
                ..unit(UnitKind::Advisory, "gizmo", Vec::new(), Vec::new())
            },
        ],
    }
}

/// The recorded plan with a git unit after its first: `gitcore`'s source
/// from 9625c3d2 to ce129c85, its workspace and its files holding the source
/// at 9625c3d2.
pub fn plan_with_git() -> RecordedPlan {
    let mut plan = plan();
    plan.git = git_at("9625c3d2");
    plan.units.insert(
        1,
        RecordedUnit {
            members: vec![String::from("gitcore"), String::from("gitmacros")],
            git: Some(GitSource {
                source: String::from(SOURCE),
                from: String::from("9625c3d2"),
                to: String::from("ce129c85"),
            }),
            ..unit(UnitKind::Git, "gitcore", Vec::new(), Vec::new())
        },
    );
    plan
}

pub fn transitive(name: &str, to: &str) -> LockMove {
    added(name, to)
}

fn unit(kind: UnitKind, name: &str, moves: Vec<LockMove>, edits: Vec<WrittenLine>) -> RecordedUnit {
    let own = moves.iter().find(|each| each.name == name).cloned();
    let revert = if edits.is_empty() {
        String::from(LOCK_ONLY)
    } else {
        String::from("git restore -- Cargo.lock Cargo.toml")
    };
    RecordedUnit {
        id: format!("{}:{name}", kind.label()),
        kind,
        name: Some(String::from(name)),
        members: vec![String::from(name)],
        from: own.as_ref().and_then(|each| each.from.clone()),
        to: own.and_then(|each| each.to),
        moves,
        urgent: false,
        edits,
        splits: Vec::new(),
        revert: Some(revert),
        git: None,
    }
}

/// The base workspace with the git source at `rev`, its fingerprint another
/// where the source moved.
pub fn with_git(rev: &str) -> FakeFiles {
    FakeFiles {
        fingerprint: if rev == "9625c3d2" {
            String::from(FINGERPRINT)
        } else {
            String::from("0000000000000002")
        },
        git: git_at(rev),
        ..base()
    }
}
