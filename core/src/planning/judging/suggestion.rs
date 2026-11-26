// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

/// What a decision suggests the caller does, or what holds it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suggestion {
    ApplyThenValidate,
    BreakingChangeOfOurs,
    DeclareTheGates,
    MigrateNow,
    MovesBack,
    Overripe,
    PlanIt,
    ReplaceOrAccept,
    RevisitThePin,
    UntilItSettles,
    UntilTheWindowCloses,
    Wait,
}

impl Suggestion {
    pub fn label(&self) -> &'static str {
        match self {
            Self::ApplyThenValidate => "apply-then-validate",
            Self::BreakingChangeOfOurs => "breaking-change-of-ours",
            Self::DeclareTheGates => "declare-the-gates",
            Self::MigrateNow => "migrate-now",
            Self::MovesBack => "moves-back",
            Self::Overripe => "overripe",
            Self::PlanIt => "plan-it",
            Self::ReplaceOrAccept => "replace-or-accept",
            Self::RevisitThePin => "revisit-the-pin",
            Self::UntilItSettles => "until-it-settles",
            Self::UntilTheWindowCloses => "until-the-window-closes",
            Self::Wait => "wait",
        }
    }

    pub fn words(&self) -> String {
        self.label().replace('-', " ")
    }
}
