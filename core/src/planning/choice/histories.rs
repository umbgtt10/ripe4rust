// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::choice::release_history::ReleaseHistory;
use crate::values::date::Date;
use crate::values::version::Version;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// The release history of every crate the plan read from the index.
pub struct Histories {
    histories: BTreeMap<String, ReleaseHistory>,
}

impl Histories {
    pub fn new(entries: Vec<(String, ReleaseHistory)>) -> Self {
        Self {
            histories: entries.into_iter().collect(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&ReleaseHistory> {
        self.histories.get(name)
    }

    pub fn is_yanked(&self, name: &str, version: &Version) -> bool {
        self.get(name)
            .is_some_and(|history| history.is_yanked(version))
    }

    pub fn published_on(&self, name: &str, version: &Version) -> Option<Date> {
        self.get(name)?.published_on(version)
    }
}
