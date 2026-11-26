// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::memory_files::MemoryFiles;
use cargo_ripe4rust::adapters::workspace::plan::plan_file::PlanFile;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::traits::plan_source::PlanSource;
use std::path::PathBuf;

const PLAN: &str = include_str!("../../../fixtures/workspace/plan.json");

#[test]
fn plan_of_a_file_not_there_is_unreadable_saying_so() {
    // Arrange
    let files = MemoryFiles::new();

    // Act
    let plan = PlanFile::new(&files, PathBuf::from("/work/ws/plan.json"), "plan.json").plan();

    // Assert
    assert_eq!(
        plan,
        Err(PortFailure::PlanUnreadable {
            file: String::from("plan.json"),
            detail: String::from("it is not there"),
        })
    );
}

#[test]
fn plan_of_a_file_the_system_refuses_is_unreadable_with_what_it_said() {
    // Arrange
    let files = MemoryFiles::new()
        .with("/work/ws/plan.json", PLAN)
        .refusing("/work/ws/plan.json");

    // Act
    let plan = PlanFile::new(&files, PathBuf::from("/work/ws/plan.json"), "plan.json").plan();

    // Assert
    assert_eq!(
        plan,
        Err(PortFailure::PlanUnreadable {
            file: String::from("plan.json"),
            detail: String::from("permission denied"),
        })
    );
}

#[test]
fn plan_of_the_recorded_plan_is_read_from_its_path() {
    // Arrange
    let files = MemoryFiles::new().with("/work/ws/plans/today.json", PLAN);

    // Act
    let plan = PlanFile::new(
        &files,
        PathBuf::from("/work/ws/plans/today.json"),
        "plans/today.json",
    )
    .plan();

    // Assert
    assert_eq!(
        plan.map(|plan| plan.fingerprint),
        Ok(String::from("af9871a4ebb0c8b9"))
    );
}
