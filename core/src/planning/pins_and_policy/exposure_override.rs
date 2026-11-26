// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

/// The policy's word on a crate's exposure, where the parse is wrong.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExposureOverride {
    pub name: String,
    pub public: bool,
}
