// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::outputs::failed;
use crate::support::replay_runner::ReplayRunner;
use crate::support::workspace::LAYOUT;
use crate::support::workspace::layout;
use cargo_ripe4rust::adapters::workspace::cargo_layout::CargoLayout;
use cargo_ripe4rust::system::traits::layout_source::LayoutSource;
use ripe4rust_core::inputs::port_failure::PortFailure;
use std::path::PathBuf;

#[test]
fn layout_asks_cargo_once_from_wherever_the_command_runs() {
    // Arrange
    let runner = layout();
    let source = CargoLayout::new(&runner, PathBuf::from("/work/ws/app"));

    // Act
    let layouts = [source.layout(), source.layout()];

    // Assert
    assert_eq!(layouts[0], layouts[1]);
    assert_eq!(
        layouts[0].as_ref().map(|layout| layout.root.clone()),
        Ok(PathBuf::from("/work/ws"))
    );
    assert_eq!(
        runner.commands(),
        vec![String::from("cargo metadata --format-version 1 --no-deps")]
    );
    assert_eq!(runner.calls.borrow()[0].dir, PathBuf::from("/work/ws/app"));
}

#[test]
fn layout_outside_a_workspace_says_what_cargo_said() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &LAYOUT,
        failed(101, include_str!("../../fixtures/cargo/locate-nowhere.txt")),
    );

    // Act
    let layout = CargoLayout::new(&runner, PathBuf::from("/work/nowhere")).layout();

    // Assert
    assert_eq!(
        layout,
        Err(PortFailure::CargoMetadataFailed {
            detail: String::from(
                "could not find `Cargo.toml` in `/work/nowhere` or any parent directory"
            ),
        })
    );
}
