// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

const MESSAGES: [&str; 2] = ["note: ", "warning: "];

/// A line of `cargo update` that moves nothing and is known: the index or a
/// git source updated, the summary of what is locked, a wait for cargo's
/// lock, an unchanged crate, and cargo's notes and warnings.
pub struct StatusLine;

impl StatusLine {
    pub fn knows(&self, line: &str) -> bool {
        let line = line.trim();
        let quoted = |word: &str| word.len() > 1 && word.starts_with('`') && word.ends_with('`');
        match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["Updating", "crates.io", "index"]
            | ["Blocking", "waiting", "for", "file", "lock", ..]
            | ["Unchanged", _, _, ..] => true,
            ["Updating", registry, "index"] => quoted(registry),
            ["Updating", "git", "repository" | "submodule", source] => quoted(source),
            ["Locking", count, "package" | "packages", "to", ..] => count.parse::<u64>().is_ok(),
            _ => MESSAGES.iter().any(|message| line.starts_with(message)),
        }
    }
}
