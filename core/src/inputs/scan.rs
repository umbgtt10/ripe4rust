// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::crate_use::CrateUse;
use crate::inputs::unparsed_file::UnparsedFile;
use alloc::vec::Vec;

/// The source scan: footprint and exposure for every crate named, and the
/// files it could not read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scan {
    pub uses: Vec<CrateUse>,
    pub unparsed: Vec<UnparsedFile>,
}
