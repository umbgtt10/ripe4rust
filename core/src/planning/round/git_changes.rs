// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::git::git_entry::GitEntry;
use crate::inputs::git::git_source::GitSource;
use alloc::string::String;
use alloc::vec::Vec;

/// The lock's git sources as the plan found them and as they are now: the
/// rev each is at, whether a source shows the rev a unit takes it to, and the
/// sources moved that no unit applied accounts for. A source moves as a whole,
/// so its crates are read by their source alone.
pub struct GitChanges {
    base: Vec<GitEntry>,
    current: Vec<GitEntry>,
}

impl GitChanges {
    pub fn new(base: &[GitEntry], current: &[GitEntry]) -> Self {
        Self {
            base: base.to_vec(),
            current: current.to_vec(),
        }
    }

    pub fn applied(&self, source: &GitSource) -> bool {
        self.rev(&source.source).as_ref() == Some(&source.to)
    }

    pub fn rev(&self, source: &str) -> Option<String> {
        Self::rev_in(&self.current, source)
    }

    pub fn unexplained(&self, applied: &[GitSource]) -> Vec<GitSource> {
        let mut sources = self
            .base
            .iter()
            .chain(&self.current)
            .map(|entry| entry.source.clone())
            .collect::<Vec<String>>();
        sources.sort();
        sources.dedup();
        sources
            .into_iter()
            .filter_map(|source| {
                let then = Self::rev_in(&self.base, &source).unwrap_or_default();
                let now = Self::rev_in(&self.current, &source).unwrap_or_default();
                let explained = applied
                    .iter()
                    .any(|moved| moved.source == source && moved.to == now);
                (then != now && !explained).then_some(GitSource {
                    source,
                    from: then,
                    to: now,
                })
            })
            .collect()
    }

    fn rev_in(entries: &[GitEntry], source: &str) -> Option<String> {
        entries
            .iter()
            .find(|entry| entry.source == source)
            .map(|entry| entry.rev.clone())
    }
}
