// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::git::git_entry::GitEntry;
use ripe4rust_core::inputs::lock_entry::LockEntry;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::values::date::Date;
use ripe4rust_core::values::location::Location;
use ripe4rust_core::values::version::Version;

/// The plan document's values read back, each the kind it was written as or
/// `plan-unreadable`, naming the plan and the value: a date, a version, a
/// `file:line`, a lock entry `<crate> <version>`.
pub struct PlanText {
    file: String,
}

impl PlanText {
    pub fn new(file: &str) -> Self {
        Self {
            file: String::from(file),
        }
    }

    pub fn date(&self, text: &str) -> Result<Date, PortFailure> {
        Date::parse(text)
            .map_err(|failure| self.unreadable(&format!("`{text}` is not {}", failure.expected)))
    }

    pub fn entry(&self, text: &str) -> Result<LockEntry, PortFailure> {
        let (name, version) = text.split_once(' ').ok_or_else(|| {
            self.unreadable(&format!(
                "`{text}` in the lock is not a crate and its version"
            ))
        })?;
        Ok(LockEntry {
            name: String::from(name),
            version: self.version(version)?,
        })
    }

    pub fn git_entry(&self, text: &str) -> Result<GitEntry, PortFailure> {
        text.split_once(' ')
            .and_then(|(name, at)| {
                let (source, rev) = at.rsplit_once('#')?;
                Some(GitEntry {
                    name: String::from(name),
                    source: String::from(source),
                    rev: String::from(rev),
                })
            })
            .ok_or_else(|| {
                self.unreadable(&format!(
                    "`{text}` in the lock is not a crate of a git source at its rev"
                ))
            })
    }

    pub fn location(&self, text: &str) -> Result<Location, PortFailure> {
        text.rsplit_once(':')
            .filter(|(file, _)| !file.is_empty())
            .and_then(|(file, line)| {
                line.parse::<u32>().ok().map(|line| Location {
                    file: String::from(file),
                    line,
                })
            })
            .ok_or_else(|| {
                self.unreadable(&format!("`{text}` is not a file and a line, file:line"))
            })
    }

    pub fn optional_version(&self, text: Option<&str>) -> Result<Option<Version>, PortFailure> {
        text.map(|text| self.version(text)).transpose()
    }

    pub fn unreadable(&self, detail: &str) -> PortFailure {
        PortFailure::PlanUnreadable {
            file: self.file.clone(),
            detail: String::from(detail),
        }
    }

    pub fn version(&self, text: &str) -> Result<Version, PortFailure> {
        Version::parse(text).map_err(|_| self.unreadable(&format!("`{text}` is not a version")))
    }
}
