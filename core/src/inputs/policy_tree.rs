// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::policy_entry::PolicyEntry;
use crate::values::location::Location;
use alloc::vec::Vec;

/// `[workspace.metadata.ripe4rust]` as the workspace's files hold it, every
/// key with the line it is on; `at` is the table's own line, `None` where the
/// manifest has none.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyTree {
    pub at: Option<Location>,
    pub entries: Vec<PolicyEntry>,
}
