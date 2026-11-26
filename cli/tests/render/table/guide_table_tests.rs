// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::render::table::guide_table::GuideTable;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::planning::round::guiding::guide::Guide;

#[test]
fn text_of_the_guide_closes_with_the_document_s_fields() {
    // Arrange
    let guide = Guide::new("0.0.1").text();

    // Act
    let text = GuideTable.text(&guide);

    // Assert
    assert!(
        text.contains("\nThe documents' fields, schema 1\nevery schema: the shape of the document"),
        "{text}"
    );
    assert!(
        text.ends_with("guide fields[].means: what it holds\n"),
        "{text}"
    );
}

#[test]
fn text_of_the_guide_lists_every_code_with_its_remedy() {
    // Arrange
    let guide = Guide::new("0.0.1").text();

    // Act
    let text = GuideTable.text(&guide);

    // Assert
    for code in DiagnosticCode::ALL {
        assert!(
            text.contains(&format!("{} ({}", code.label(), code.severity().label())),
            "{} in {text}",
            code.label()
        );
        assert!(text.contains(code.entry().remedy), "{}", code.label());
    }
    assert!(text.contains(
        "gates-undeclared (warning, needs a person): no `gates` are declared, so nothing can validate a unit: every unit that would be ripe is asked. Do: ask the owner"
    ));
}

#[test]
fn text_of_the_guide_opens_with_its_release_and_the_round() {
    // Arrange
    let guide = Guide::new("0.0.1").text();

    // Act
    let text = GuideTable.text(&guide);

    // Assert
    assert!(
        text.starts_with(
            "ripe4rust 0.0.1: the guide\n\nThe round\n1. Run `cargo ripe4rust plan --format json > plan.json`.\n   - Exit `2`: carry out each error's `do`"
        ),
        "{text}"
    );
}

#[test]
fn text_of_the_guide_says_what_each_command_takes_and_exits_with() {
    // Arrange
    let guide = Guide::new("0.0.1").text();

    // Act
    let text = GuideTable.text(&guide);

    // Assert
    assert!(
        text.contains(
            "\ncargo ripe4rust check <unit> --plan <file> [--format table|json]\n  compares the workspace"
        ),
        "{text}"
    );
    assert!(
        text.contains("\n  exit 3: the plan is stale: plan again, and go on with the new plan\n"),
        "{text}"
    );
    assert!(
        text.contains("\n  --because <text>: the pin's reason"),
        "{text}"
    );
}
