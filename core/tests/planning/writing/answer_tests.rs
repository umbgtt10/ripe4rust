// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::planning::writing::answer::Answer;

#[test]
fn label_of_every_answer_is_its_kebab_name() {
    // Arrange & Act
    let labels = [
        Answer::Accept,
        Answer::DeclareGates,
        Answer::KeepPinned,
        Answer::MigrateNow,
        Answer::PlanIt,
        Answer::Remove,
        Answer::Replace,
        Answer::TakeIt,
        Answer::Unpin,
        Answer::Wait,
    ]
    .map(|answer| answer.label());

    // Assert
    assert_eq!(
        labels,
        [
            "accept",
            "declare-gates",
            "keep-pinned",
            "migrate-now",
            "plan-it",
            "remove",
            "replace",
            "take-it",
            "unpin",
            "wait",
        ]
    );
}
