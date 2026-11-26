// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::port_failure::PortFailure;
use crate::inputs::workspace::Workspace;
use alloc::string::String;
use alloc::vec::Vec;

/// What `cargo metadata` says: the members and the lock, and which crates a
/// target's build reaches.
pub trait WorkspaceMetadata {
    fn read(&self) -> Result<Workspace, PortFailure>;
    fn reached_by(&self, target: &str) -> Result<Vec<String>, PortFailure>;
}
