// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::port_failure::PortFailure;

const HELP: &str = " help: ";
const LOCKED: &str = "--locked was passed to prevent this";

/// What a failing `cargo metadata --locked` means: a lock it would have to
/// write -- missing, or out of step with the manifests -- is `lock-outdated`,
/// cargo's own advice to drop `--locked` left out, since ripe4rust writes no
/// lock; anything else is `cargo-metadata-failed`.
pub struct MetadataFailure;

impl MetadataFailure {
    pub fn of(&self, detail: String) -> PortFailure {
        if detail.contains(LOCKED) {
            let refusal = detail.split(HELP).next().unwrap_or_default();
            PortFailure::LockOutdated {
                detail: String::from(refusal),
            }
        } else {
            PortFailure::CargoMetadataFailed { detail }
        }
    }
}
