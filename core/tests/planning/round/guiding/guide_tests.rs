// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::planning::round::guiding::commands_guide::CommandsGuide;
use ripe4rust_core::planning::round::guiding::fields_guide::FieldsGuide;
use ripe4rust_core::planning::round::guiding::guide::Guide;
use ripe4rust_core::planning::round::guiding::round_guide::RoundGuide;

#[test]
fn text_carries_the_round_every_command_and_the_document_s_fields() {
    // Arrange & Act
    let text = Guide::new("0.0.1").text();

    // Assert
    assert_eq!(
        (text.round, text.commands, text.fields),
        (
            RoundGuide.steps(),
            CommandsGuide.commands(),
            FieldsGuide.fields()
        )
    );
}

#[test]
fn text_has_an_entry_for_every_code_the_core_can_produce() {
    // Arrange & Act
    let text = Guide::new("0.0.1").text();

    // Assert
    assert_eq!(text.codes, DiagnosticCode::ALL.to_vec());
}

#[test]
fn text_is_versioned_with_the_binary() {
    // Arrange & Act
    let text = Guide::new("1.2.3").text();

    // Assert
    assert_eq!(text.version, "1.2.3");
}
