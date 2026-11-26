// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::git::git_move::GitMove;
use ripe4rust_core::values::version::Version;

/// A line of `cargo update` that moves a crate of a git source: `Updating`
/// or `Downgrading <crate> v<version> (<source>#<rev>) -> #<rev>`, `Adding`
/// or `Removing <crate> v<version> (<source>#<rev>)`, the source its URL and
/// spec as cargo writes it. cargo names no version a moved crate takes. Any
/// other line is no git move.
pub struct GitMoveLine;

impl GitMoveLine {
    pub fn read(&self, line: &str) -> Option<GitMove> {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let [verb, name, version, at, rest @ ..] = words.as_slice() else {
            return None;
        };
        let version = Version::parse(version.strip_prefix('v')?).ok()?;
        let (source, rev) = at
            .strip_prefix('(')?
            .strip_suffix(')')?
            .rsplit_once('#')
            .filter(|(source, rev)| !source.is_empty() && !rev.is_empty())?;
        let (from, to) = match (*verb, rest) {
            ("Updating" | "Downgrading", ["->", to]) => (Some(rev), Some(to.strip_prefix('#')?)),
            ("Adding", []) => (None, Some(rev)),
            ("Removing", []) => (Some(rev), None),
            _ => return None,
        };
        Some(GitMove {
            name: String::from(*name),
            source: String::from(source),
            version,
            from: from.map(String::from),
            to: to.map(String::from),
        })
    }
}
