// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::locked_package::LockedPackage;
use crate::inputs::member::Member;
use alloc::vec::Vec;

/// What `cargo metadata` says of the workspace: its members and the
/// registry packages of its lock.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Workspace {
    pub members: Vec<Member>,
    pub lock: Vec<LockedPackage>,
}
