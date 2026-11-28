// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::member::Member;
use crate::inputs::source_scan::scan::Scan;
use crate::planning::judging::footprint::Footprint;
use crate::values::location::Location;
use alloc::string::String;
use alloc::vec::Vec;

/// Where the workspace names some crates, together: every `file:line` once,
/// and the includes the parse could not follow in the members using them,
/// which make the footprint a floor; nothing is known of it where the scan
/// failed as a whole.
pub struct Footprints<'a> {
    scan: Option<&'a Scan>,
    members: &'a [Member],
}

impl<'a> Footprints<'a> {
    pub fn new(scan: Option<&'a Scan>, members: &'a [Member]) -> Self {
        Self { scan, members }
    }

    pub fn footprint(&self, names: &[String]) -> Option<Footprint> {
        self.scan?;
        let locations = self.locations(names);
        let mut files = locations
            .iter()
            .map(|location| &location.file)
            .collect::<Vec<&String>>();
        files.dedup();
        Some(Footprint {
            lines: u32::try_from(locations.len()).unwrap_or(u32::MAX),
            files: u32::try_from(files.len()).unwrap_or(u32::MAX),
            complete: self.unfollowed(names).is_empty(),
        })
    }

    pub fn locations(&self, names: &[String]) -> Vec<Location> {
        let mut locations = self
            .scan
            .iter()
            .flat_map(|scan| scan.uses.iter())
            .filter(|used| names.contains(&used.name))
            .flat_map(|used| used.locations.iter().cloned())
            .collect::<Vec<Location>>();
        locations.sort();
        locations.dedup();
        locations
    }

    pub fn unfollowed(&self, names: &[String]) -> Vec<Location> {
        let using = |member: &str| {
            self.members.iter().any(|used| {
                used.name == member
                    && used
                        .dependencies
                        .iter()
                        .any(|dependency| names.contains(&dependency.name))
            })
        };
        let mut unfollowed = self
            .scan
            .iter()
            .flat_map(|scan| scan.unfollowed.iter())
            .filter(|include| using(&include.member))
            .map(|include| include.at.clone())
            .collect::<Vec<Location>>();
        unfollowed.sort();
        unfollowed.dedup();
        unfollowed
    }
}
