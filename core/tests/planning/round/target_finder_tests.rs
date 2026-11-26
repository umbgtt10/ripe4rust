// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::recorded::plan;
use crate::support::units::moved;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::planning::round::target::Target;
use ripe4rust_core::planning::round::target_finder::TargetFinder;

#[test]
fn find_of_a_split_id_is_its_one_move() {
    // Arrange
    let plan = plan();

    // Act
    let found = TargetFinder::new(&plan).find("compatible:widget/gizmo");

    // Assert
    assert_eq!(
        found,
        Ok(Target {
            id: String::from("compatible:widget/gizmo"),
            unit: plan.units[2].clone(),
            moves: vec![moved("gizmo", "2.7.5", "2.7.6")],
            edits: Vec::new(),
        })
    );
}

#[test]
fn find_of_a_unit_s_id_is_the_unit_with_its_moves_and_edits() {
    // Arrange
    let plan = plan();

    // Act
    let found = TargetFinder::new(&plan).find("breaking:gadget");

    // Assert
    assert_eq!(
        found,
        Ok(Target {
            id: String::from("breaking:gadget"),
            unit: plan.units[0].clone(),
            moves: plan.units[0].moves.clone(),
            edits: plan.units[0].edits.clone(),
        })
    );
}

#[test]
fn find_of_an_id_the_plan_does_not_hold_is_unit_unknown() {
    // Arrange
    let plan = plan();

    // Act
    let refused = [
        "compatible:serde",
        "compatible:widget/serde",
        "compatible:widget/",
    ]
    .map(|id| TargetFinder::new(&plan).find(id).err());

    // Assert
    for (refused, id) in refused.iter().zip([
        "compatible:serde",
        "compatible:widget/serde",
        "compatible:widget/",
    ]) {
        let refused = refused.as_ref().expect("a refusal");
        assert_eq!(refused.code, DiagnosticCode::UnitUnknown);
        assert_eq!(
            refused.what,
            format!("`{id}` is no unit of the plan, nor a move one of its units splits into")
        );
        assert_eq!(
            refused.action,
            "name a unit by the `id` the plan gives it, or one of its moves by an `id` of its `steps.split`"
        );
    }
}
