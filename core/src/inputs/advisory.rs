// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::requirement::Requirement;
use crate::values::version::Version;
use alloc::string::String;
use alloc::vec::Vec;

/// An advisory against a version in use, as cargo-deny reports it, with the
/// ranges a version is clear of it in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Advisory {
    pub id: String,
    pub name: String,
    pub version: Version,
    pub patched: Vec<Requirement>,
    pub unaffected: Vec<Requirement>,
}
