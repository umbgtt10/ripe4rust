// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::source_scan::crate_use::CrateUse;
use crate::inputs::source_scan::unfollowed_include::UnfollowedInclude;
use crate::inputs::source_scan::unparsed_file::UnparsedFile;
use alloc::string::String;
use alloc::vec::Vec;

/// The source scan: footprint and exposure for every crate named, the files
/// it could not read, and the members whose public API it could not read in
/// full -- a file of their library among those --, so whose exposure is not
/// known.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scan {
    pub uses: Vec<CrateUse>,
    pub unparsed: Vec<UnparsedFile>,
    pub unread_apis: Vec<String>,
    pub unfollowed: Vec<UnfollowedInclude>,
}
