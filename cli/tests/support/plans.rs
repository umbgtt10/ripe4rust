// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::git::git_source::GitSource;
use ripe4rust_core::inputs::lock_move::LockMove;
use ripe4rust_core::inputs::written_line::WrittenLine;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::suggestion::Suggestion;
use ripe4rust_core::planning::plan::Plan;
use ripe4rust_core::planning::planned_unit::PlannedUnit;
use ripe4rust_core::planning::units::choice::line_stats::LineStats;
use ripe4rust_core::planning::units::unit::Unit;
use ripe4rust_core::planning::units::unit_id::UnitId;
use ripe4rust_core::planning::workspace_state::WorkspaceState;
use ripe4rust_core::planning::writing::answer::Answer;
use ripe4rust_core::planning::writing::evidence::Evidence;
use ripe4rust_core::planning::writing::question::Question;
use ripe4rust_core::planning::writing::question_option::QuestionOption;
use ripe4rust_core::planning::writing::revert::Revert;
use ripe4rust_core::planning::writing::split_step::SplitStep;
use ripe4rust_core::planning::writing::step::Step;
use ripe4rust_core::planning::writing::steps::Steps;
use ripe4rust_core::unit_kind::UnitKind;
use ripe4rust_core::values::date::Date;
use ripe4rust_core::values::line::Line;
use ripe4rust_core::values::location::Location;
use ripe4rust_core::values::partial_version::PartialVersion;
use ripe4rust_core::values::version::Version;

pub fn at(file: &str, line: u32) -> Location {
    Location {
        file: String::from(file),
        line,
    }
}

pub fn breaking() -> PlannedUnit {
    PlannedUnit {
        unit: Unit {
            id: UnitId::of(UnitKind::Breaking, "gadget"),
            kind: UnitKind::Breaking,
            name: Some(String::from("gadget")),
            members: vec![String::from("gadget")],
            from: Some(version("0.13.1")),
            to: Some(version("0.14.0")),
            moves: vec![moved("gadget", Some("0.13.1"), Some("0.14.0"))],
            held: Vec::new(),
            git: None,
        },
        decision: Decision {
            outcome: Outcome::Ask,
            urgent: false,
            rule: Some(7),
            suggestion: Some(Suggestion::ApplyThenValidate),
            reasons: vec![
                String::from("reaches thumbv7em-none-eabihf"),
                String::from("plan it: footprint 42 lines in 7 files"),
            ],
            why: Some(String::from(
                "it reaches thumbv7em-none-eabihf, which the gates cannot validate",
            )),
        },
        summary: String::from("Migrate gadget to 0.14.0"),
        steps: Some(Steps {
            apply: vec![
                Step::Edit {
                    at: at("Cargo.toml", 22),
                    key: String::from("gadget"),
                    value: String::from("0.14.0"),
                },
                run("cargo update -p gadget"),
            ],
            check: String::from("cargo ripe4rust check breaking:gadget --plan plan.json"),
            gates: gates(),
            revert: Revert {
                restore: vec![String::from("Cargo.lock"), String::from("Cargo.toml")],
                run: String::from("git restore -- Cargo.lock Cargo.toml"),
            },
            split: Vec::new(),
        }),
        question: Some(Question {
            title: String::from("Move gadget to 0.14.0?"),
            why: String::from("it reaches thumbv7em-none-eabihf, which the gates cannot validate"),
            recommendation: Answer::PlanIt,
            options: vec![QuestionOption {
                answer: Answer::Wait,
                means: String::from("not now"),
                then: String::from(
                    "cargo ripe4rust hold breaking:gadget --plan plan.json --because \"<the person's words>\"",
                ),
            }],
        }),
        evidence: Evidence {
            line_in_use: None,
            rust_version: Some(PartialVersion::parse("1.80").expect("a rust-version")),
            advisories: Vec::new(),
            yanked_in_use: false,
            footprint: vec![at("src/net.rs", 4)],
            held_back: Vec::new(),
            held: Vec::new(),
            downgrades: Vec::new(),
            unfollowed_includes: Vec::new(),
            held_by: Vec::new(),
        },
    }
}

pub fn compatible() -> PlannedUnit {
    PlannedUnit {
        unit: Unit {
            id: UnitId::of(UnitKind::Compatible, "widget"),
            kind: UnitKind::Compatible,
            name: Some(String::from("widget")),
            members: vec![String::from("widget")],
            from: Some(version("4.6.6")),
            to: Some(version("4.6.7")),
            moves: vec![
                moved("widget", Some("4.6.6"), Some("4.6.7")),
                moved("widget_builder", Some("4.6.6"), Some("4.6.7")),
            ],
            held: Vec::new(),
            git: None,
        },
        decision: Decision {
            outcome: Outcome::Ripe,
            urgent: false,
            rule: Some(10),
            suggestion: None,
            reasons: vec![String::from("compatible")],
            why: None,
        },
        summary: String::from("Move widget to 4.6.7, with 1 crate it pulls in"),
        steps: Some(Steps {
            apply: vec![run("cargo update --recursive -p widget")],
            check: String::from("cargo ripe4rust check compatible:widget --plan plan.json"),
            gates: gates(),
            revert: Revert {
                restore: vec![String::from("Cargo.lock")],
                run: String::from("git restore -- Cargo.lock"),
            },
            split: vec![SplitStep {
                id: String::from("compatible:widget/widget"),
                apply: vec![run("cargo update -p widget --precise 4.6.7")],
            }],
        }),
        question: None,
        evidence: Evidence {
            line_in_use: Some(LineStats {
                line: Line {
                    major: 4,
                    minor: None,
                },
                first_release: Some(date("2022-01-01")),
                last_release: Some(date("2026-10-02")),
                releases: 41,
            }),
            rust_version: Some(PartialVersion::parse("1.74").expect("a rust-version")),
            advisories: Vec::new(),
            yanked_in_use: false,
            footprint: Vec::new(),
            held_back: Vec::new(),
            held: Vec::new(),
            downgrades: Vec::new(),
            unfollowed_includes: Vec::new(),
            held_by: Vec::new(),
        },
    }
}

pub fn date(text: &str) -> Date {
    Date::parse(text).expect("a date")
}

pub fn gates() -> Vec<String> {
    vec![String::from("just stage1"), String::from("just stage2")]
}

pub fn git_unit() -> PlannedUnit {
    let mut git = compatible();
    git.unit = Unit {
        id: UnitId::of(UnitKind::Git, "gitcore"),
        kind: UnitKind::Git,
        name: Some(String::from("gitcore")),
        members: vec![String::from("gitcore"), String::from("gitmacros")],
        from: None,
        to: None,
        moves: vec![moved("itoa", Some("1.0.10"), Some("1.0.19"))],
        held: Vec::new(),
        git: Some(GitSource {
            source: String::from("https://github.com/example/upstream?branch=main"),
            from: String::from("9625c3d2"),
            to: String::from("ce129c85"),
        }),
    };
    git
}

pub fn moved(name: &str, from: Option<&str>, to: Option<&str>) -> LockMove {
    LockMove {
        name: String::from(name),
        from: from.map(version),
        to: to.map(version),
    }
}

pub fn plan(units: Vec<PlannedUnit>) -> Plan {
    Plan {
        date: date("2026-10-30"),
        registry_read: date("2026-10-30"),
        workspace: WorkspaceState {
            fingerprint: String::from("9f2c41d0e3b5a7c9"),
            lock: vec![String::from("gadget 0.13.1"), String::from("widget 4.6.6")],
            lines: vec![
                written(at("Cargo.toml", 22), "gadget", "0.13"),
                written(at("rust-toolchain.toml", 2), "channel", "1.99.1"),
            ],
            git: Vec::new(),
            edges: Vec::new(),
        },
        gates: gates(),
        shared: Vec::new(),
        diagnostics: Vec::new(),
        units,
    }
}

pub fn rest() -> PlannedUnit {
    let mut rest = compatible();
    rest.unit = Unit {
        id: UnitId::rest(),
        kind: UnitKind::Compatible,
        name: None,
        members: Vec::new(),
        from: None,
        to: None,
        moves: vec![
            moved("libc", Some("0.2.177"), Some("0.2.178")),
            moved("windows-link", None, Some("0.2.1")),
        ],
        held: Vec::new(),
        git: None,
    };
    rest.decision.reasons = vec![String::from("2 moves no unit reaches")];
    rest
}

pub fn run(command: &str) -> Step {
    Step::Run {
        command: String::from(command),
    }
}

pub fn toolchain() -> PlannedUnit {
    let mut toolchain = compatible();
    toolchain.unit = Unit {
        id: UnitId::of(UnitKind::Toolchain, "rust"),
        kind: UnitKind::Toolchain,
        name: Some(String::from("rust")),
        members: vec![String::from("rust")],
        from: Some(version("1.99.1")),
        to: Some(version("1.100.0")),
        moves: Vec::new(),
        held: Vec::new(),
        git: None,
    };
    toolchain.decision.reasons = vec![
        String::from("toolchain"),
        String::from("releases 2 versions held back"),
    ];
    toolchain
}

pub fn version(text: &str) -> Version {
    Version::parse(text).expect("a version")
}

pub fn written(at: Location, key: &str, value: &str) -> WrittenLine {
    WrittenLine {
        at,
        key: String::from(key),
        value: String::from(value),
    }
}
