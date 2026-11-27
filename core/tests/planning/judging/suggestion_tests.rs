// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::planning::judging::suggestion::Suggestion;

#[test]
fn label_of_every_suggestion_is_its_kebab_name() {
    // Arrange & Act
    let labels = [
        Suggestion::ApplyThenValidate,
        Suggestion::BreakingChangeOfOurs,
        Suggestion::DeclareTheGates,
        Suggestion::MigrateNow,
        Suggestion::MovesBack,
        Suggestion::Overripe,
        Suggestion::PlanIt,
        Suggestion::ReplaceOrAccept,
        Suggestion::RevisitThePin,
        Suggestion::UntilItSettles,
        Suggestion::UntilItsDependentsMove,
        Suggestion::UntilTheWindowCloses,
        Suggestion::Wait,
    ]
    .map(|suggestion| suggestion.label());

    // Assert
    assert_eq!(
        labels,
        [
            "apply-then-validate",
            "breaking-change-of-ours",
            "declare-the-gates",
            "migrate-now",
            "moves-back",
            "overripe",
            "plan-it",
            "replace-or-accept",
            "revisit-the-pin",
            "until-it-settles",
            "until-its-dependents-move",
            "until-the-window-closes",
            "wait",
        ]
    );
}

#[test]
fn words_of_a_suggestion_are_its_label_spaced() {
    // Arrange & Act
    let words = Suggestion::PlanIt.words();

    // Assert
    assert_eq!(words, "plan it");
}
