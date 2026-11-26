// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_entry::LockEntry;
use crate::inputs::policy_tree::PolicyTree;
use crate::inputs::port_failure::PortFailure;
use crate::inputs::toolchain_file::ToolchainFile;
use crate::inputs::written_line::WrittenLine;
use crate::values::location::Location;
use alloc::string::String;
use alloc::vec::Vec;

/// The workspace's own files: the policy as written, `rust-toolchain.toml`,
/// the crates.io packages `Cargo.lock` holds, the lines that state a
/// dependency's requirement or a member's `rust-version`, every line a unit's
/// `apply` can edit with what it says, and the fingerprint of the lock, the
/// manifests and the toolchain file.
pub trait WorkspaceFiles {
    fn fingerprint(&self) -> Result<String, PortFailure>;
    fn lock(&self) -> Result<Vec<LockEntry>, PortFailure>;
    fn policy(&self) -> Result<PolicyTree, PortFailure>;
    fn requirement_lines(&self, name: &str) -> Result<Vec<Location>, PortFailure>;
    fn rust_version_line(&self, member: &str) -> Result<Option<Location>, PortFailure>;
    fn toolchain(&self) -> Result<Option<ToolchainFile>, PortFailure>;
    fn written(&self) -> Result<Vec<WrittenLine>, PortFailure>;
}
