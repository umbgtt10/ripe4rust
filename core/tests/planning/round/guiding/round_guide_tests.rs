// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::planning::round::guiding::round_guide::RoundGuide;

#[test]
fn steps_open_with_the_plan_and_close_when_every_unit_is_done() {
    // Arrange & Act
    let steps = RoundGuide.steps();

    // Assert
    assert_eq!(
        steps.first().map(String::as_str),
        Some("1. Run `cargo ripe4rust plan --format json > plan.json`.")
    );
    assert_eq!(
        steps.last().map(String::as_str),
        Some(
            "6. The round is done when every unit is committed, held, answered or ripening. A last `plan` shows only those held, asked or ripening."
        )
    );
}

#[test]
fn steps_say_what_to_do_on_each_code_check_exits_with() {
    // Arrange & Act
    let steps = RoundGuide.steps();

    // Assert
    for code in ["`0`:", "`1`:", "`2`:", "`3`:"] {
        assert!(
            steps
                .iter()
                .any(|step| step.trim_start().starts_with(&format!("- {code}"))),
            "{code} in {steps:?}"
        );
    }
}

#[test]
fn steps_say_when_to_plan_with_every_answer_fetched_again() {
    // Arrange & Act
    let steps = RoundGuide.steps();

    // Assert
    assert_eq!(
        steps.get(2).map(String::as_str),
        Some(
            "   - A plan reads the registry's answers and the Rust distribution's list of releases from the day's cache, so one made later the same day does not see a release published since the day's first plan. Where one came out since -- a crate's, or Rust's --, plan with `--refresh`: every answer is fetched again and the day's copy overwritten."
        )
    );
}
