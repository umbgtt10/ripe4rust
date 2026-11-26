// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::render::table::sentence::Sentence;

#[test]
fn ended_of_a_text_ending_in_a_full_stop_keeps_it_alone() {
    // Arrange & Act
    let ended = Sentence.ended("use --offline instead.");

    // Assert
    assert_eq!(ended, "use --offline instead.");
}

#[test]
fn ended_of_a_text_ending_in_a_question_mark_keeps_it() {
    // Arrange & Act
    let ended = Sentence.ended("did you mean `itoa`?");

    // Assert
    assert_eq!(ended, "did you mean `itoa`?");
}

#[test]
fn ended_of_a_text_without_an_ending_closes_it_with_a_full_stop() {
    // Arrange & Act
    let ended = Sentence.ended("the pin on `syn` gives no reason");

    // Assert
    assert_eq!(ended, "the pin on `syn` gives no reason.");
}
