// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::plans::moved;
use ripe4rust_core::planning::round::checking::check_report::CheckReport;
use ripe4rust_core::planning::round::checking::difference::Difference;

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
        applied: vec![
            String::from("compatible:widget"),
            String::from("breaking:gadget"),
        ],
        to_go: vec![String::from("rest")],
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
        applied: Vec::new(),
        to_go: vec![String::from("compatible:widget"), String::from("rest")],
        diagnostics: Vec::new(),
    }
}
