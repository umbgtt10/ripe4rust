// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::port_failure::PortFailure;

const HELP: &str = " help: ";
const LOCKED: &str = "--locked was passed to prevent this";
const MISSING: &str = "cannot create the lock file";

/// What a failing `cargo metadata --locked` means: a lock it would have to
/// create is `lock-missing`, and one out of step with the manifests, which it
/// would have to update, `lock-outdated` -- cargo's own advice to drop
/// `--locked` left out of both, since ripe4rust writes no lock; anything else
/// is `cargo-metadata-failed`.
pub struct MetadataFailure;

impl MetadataFailure {
    pub fn of(&self, detail: String) -> PortFailure {
        if !detail.contains(LOCKED) {
            return PortFailure::CargoMetadataFailed { detail };
        }
        let refusal = String::from(detail.split(HELP).next().unwrap_or_default());
        if refusal.contains(MISSING) {
            PortFailure::LockMissing { detail: refusal }
        } else {
            PortFailure::LockOutdated { detail: refusal }
        }
    }
}
