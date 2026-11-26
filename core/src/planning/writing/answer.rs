// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

const LABELS: [(Answer, &str); 9] = [
    (Answer::Accept, "accept"),
    (Answer::DeclareGates, "declare-gates"),
    (Answer::KeepPinned, "keep-pinned"),
    (Answer::MigrateNow, "migrate-now"),
    (Answer::PlanIt, "plan-it"),
    (Answer::Replace, "replace"),
    (Answer::TakeIt, "take-it"),
    (Answer::Unpin, "unpin"),
    (Answer::Wait, "wait"),
];

/// An answer a person can give to a unit's question.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Answer {
    Accept,
    DeclareGates,
    KeepPinned,
    MigrateNow,
    PlanIt,
    Replace,
    TakeIt,
    Unpin,
    Wait,
}

impl Answer {
    pub fn label(&self) -> &'static str {
        LABELS
            .iter()
            .find(|(answer, _)| answer == self)
            .map_or("", |(_, label)| label)
    }
}
