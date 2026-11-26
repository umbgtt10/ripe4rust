// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::lock_move::LockMove;
use ripe4rust_core::values::version::Version;

const NOTES: [&str; 3] = ["(available: v", "(latest: v", "(requires Rust "];

/// A line of `cargo update` that moves a crate: `Updating` or
/// `Downgrading <crate> v<from> -> v<to>`, `Adding <crate> v<to>`, `Removing
/// <crate> v<from>`, each with no note or one cargo is known to write. Any
/// other line is no move.
pub struct MoveLine;

impl MoveLine {
    pub fn read(&self, line: &str) -> Option<LockMove> {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let [verb, name, rest @ ..] = words.as_slice() else {
            return None;
        };
        let version = |word: &str| Version::parse(word.strip_prefix('v')?).ok();
        let (from, to, note) = match (*verb, rest) {
            ("Updating" | "Downgrading", [from, "->", to, note @ ..]) => {
                (Some(version(from)?), Some(version(to)?), note)
            }
            ("Adding", [to, note @ ..]) => (None, Some(version(to)?), note),
            ("Removing", [from, note @ ..]) => (Some(version(from)?), None, note),
            _ => return None,
        };
        let note = note.join(" ");
        let known = note.is_empty()
            || (note.ends_with(')') && NOTES.iter().any(|known| note.starts_with(known)));
        known.then(|| LockMove {
            name: String::from(*name),
            from,
            to,
        })
    }
}
