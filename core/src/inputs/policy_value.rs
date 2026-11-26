// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::policy_entry::PolicyEntry;
use alloc::string::String;
use alloc::vec::Vec;

/// A policy value as the manifest writes it, before the policy is read; a
/// date the manifest writes bare arrives as text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PolicyValue {
    Boolean(bool),
    Integer(i64),
    List(Vec<PolicyValue>),
    Table(Vec<PolicyEntry>),
    Text(String),
}
