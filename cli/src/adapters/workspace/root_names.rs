// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;

/// A file named from the workspace's root, with `/` on every system; one
/// outside it keeps its own path.
pub struct RootNames<'a> {
    root: &'a Path,
}

impl<'a> RootNames<'a> {
    pub fn new(root: &'a Path) -> Self {
        Self { root }
    }

    pub fn name(&self, path: &Path) -> String {
        let Ok(relative) = path.strip_prefix(self.root) else {
            return path.to_string_lossy().into_owned();
        };
        relative
            .components()
            .map(|part| part.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<String>>()
            .join("/")
    }
}
