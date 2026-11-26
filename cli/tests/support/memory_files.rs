// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::system::model::file_failure::FileFailure;
use cargo_ripe4rust::system::traits::file_system::FileSystem;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::Path;

/// Files in memory, by path written with `/` on every system; a directory
/// is there while a file is under it.
pub struct MemoryFiles {
    pub files: RefCell<BTreeMap<String, String>>,
    pub refused: Vec<String>,
}

impl MemoryFiles {
    pub fn new() -> Self {
        Self {
            files: RefCell::new(BTreeMap::new()),
            refused: Vec::new(),
        }
    }

    pub fn contents(&self, path: &str) -> Option<String> {
        self.files.borrow().get(path).cloned()
    }

    pub fn refusing(mut self, path: &str) -> Self {
        self.refused.push(String::from(path));
        self
    }

    pub fn with(self, path: &str, contents: &str) -> Self {
        self.files
            .borrow_mut()
            .insert(String::from(path), String::from(contents));
        self
    }

    fn key(path: &Path) -> String {
        path.to_string_lossy().replace('\\', "/")
    }
}

impl FileSystem for MemoryFiles {
    fn entries(&self, directory: &Path) -> Result<Vec<String>, FileFailure> {
        let prefix = format!("{}/", Self::key(directory));
        let mut names = self
            .files
            .borrow()
            .keys()
            .filter_map(|path| path.strip_prefix(&prefix))
            .filter_map(|rest| rest.split('/').next())
            .map(String::from)
            .collect::<Vec<String>>();
        names.dedup();
        if names.is_empty() {
            Err(FileFailure::Missing)
        } else {
            Ok(names)
        }
    }

    fn read(&self, path: &Path) -> Result<String, FileFailure> {
        let key = Self::key(path);
        if self.refused.contains(&key) {
            return Err(FileFailure::Failed {
                detail: String::from("permission denied"),
            });
        }
        self.contents(&key).ok_or(FileFailure::Missing)
    }

    fn write(&self, path: &Path, contents: &str) -> Result<(), FileFailure> {
        let key = Self::key(path);
        if self.refused.contains(&key) {
            return Err(FileFailure::Failed {
                detail: String::from("read-only file system"),
            });
        }
        self.files.borrow_mut().insert(key, String::from(contents));
        Ok(())
    }
}
