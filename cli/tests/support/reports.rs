// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::plans::moved;
use ripe4rust_core::planning::round::checking::check_report::CheckReport;
use ripe4rust_core::planning::round::checking::difference::Difference;
use ripe4rust_core::planning::round::holding::hold_report::HoldReport;
use ripe4rust_core::planning::round::holding::pin_line::PinLine;
use ripe4rust_core::values::location::Location;

pub fn as_planned() -> CheckReport {
    CheckReport {
        unit: String::from("breaking:gadget"),
        as_planned: true,
        transitive: vec![
            moved("gizmo", Some("2.7.5"), Some("2.7.6")),
            moved("tiny", None, Some("1.0.0")),
            moved("old", Some("1.0.0"), None),
        ],
        differences: Vec::new(),
        shared: Vec::new(),
        applied: vec![
            String::from("compatible:widget"),
            String::from("breaking:gadget"),
        ],
        to_go: vec![String::from("rest")],
        diagnostics: Vec::new(),
    }
}

pub fn held(table: bool) -> HoldReport {
    let at = |line: u32| Location {
        file: String::from("Cargo.toml"),
        line,
    };
    HoldReport {
        unit: String::from("compatible:widget"),
        section: String::from("[workspace.metadata.ripe4rust.pins]"),
        at: table.then(|| at(40)),
        pins: vec![
            PinLine {
                name: String::from("widget"),
                line: String::from(
                    r#"widget = { reason = "red", review = "2026-12-10", hold = "all" }"#,
                ),
                replaces: table.then(|| at(41)),
            },
            PinLine {
                name: String::from("gizmo"),
                line: String::from(
                    r#"gizmo = { reason = "red", review = "2026-12-10", hold = "all" }"#,
                ),
                replaces: None,
            },
        ],
        action: String::from("put the pins there, and commit it"),
        diagnostics: Vec::new(),
    }
}

pub fn not_as_planned() -> CheckReport {
    CheckReport {
        unit: String::from("compatible:widget"),
        as_planned: false,
        transitive: Vec::new(),
        differences: vec![Difference {
            at: String::from("Cargo.lock"),
            what: String::from("`libc` moved 0.2.177 -> 0.2.180, a move no unit of the plan makes"),
            action: String::from("carry out the unit's `revert`"),
        }],
        shared: vec![String::from(
            "`wasip2` 1.0.3 left the lock: the units share the crate, and its moves depend on the order they are applied in",
        )],
        applied: Vec::new(),
        to_go: vec![String::from("compatible:widget"), String::from("rest")],
        diagnostics: Vec::new(),
    }
}
