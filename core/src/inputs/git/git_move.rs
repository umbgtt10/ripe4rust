// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::version::Version;
use alloc::string::String;

/// One line of `cargo update --dry-run` moving a crate of a git source: the
/// crate, its source as cargo writes it -- the URL and its branch, tag or
/// rev spec --, the version cargo names, and the short revs it leaves and
/// takes: no `from` where it is added, no `to` where it is removed. cargo
/// names no version a moved crate takes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GitMove {
    pub name: String,
    pub source: String,
    pub version: Version,
    pub from: Option<String>,
    pub to: Option<String>,
}
