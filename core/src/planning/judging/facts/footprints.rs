// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::scan::Scan;
use crate::planning::judging::footprint::Footprint;
use crate::values::location::Location;
use alloc::string::String;
use alloc::vec::Vec;

/// Where the workspace names some crates, together: every `file:line` once;
/// nothing is known of it while the sources are not parsed.
pub struct Footprints<'a> {
    scan: Option<&'a Scan>,
}

impl<'a> Footprints<'a> {
    pub fn new(scan: Option<&'a Scan>) -> Self {
        Self { scan }
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
}
