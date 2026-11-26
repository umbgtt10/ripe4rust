// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::unreadable::Unreadable;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::release::Release;
use ripe4rust_core::values::date::Date;
use ripe4rust_core::values::partial_version::PartialVersion;
use ripe4rust_core::values::version::Version;
use serde::Deserialize;
use serde_json::from_str;

const DATE: usize = 10;

#[derive(Deserialize)]
struct IndexEntry {
    pub vers: String,
    pub yanked: bool,
    pub rust_version: Option<String>,
    pub pubtime: Option<String>,
}

/// A crate's file of the sparse index, an entry a line: every release with
/// its yanked flag, its `rust_version` and the day of its `pubtime`, none
/// where the index carries none.
pub struct IndexReader;

impl IndexReader {
    pub fn releases(&self, name: &str, text: &str) -> Result<Vec<Release>, PortFailure> {
        let unreadable = Unreadable::new(&format!("reading the index's answer for `{name}`"));
        text.lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let entry = from_str::<IndexEntry>(line)
                    .map_err(|error| unreadable.failure(&error.to_string()))?;
                Ok(Release {
                    version: Version::parse(&entry.vers)
                        .map_err(|failure| unreadable.unparsed(&failure))?,
                    yanked: entry.yanked,
                    rust_version: entry
                        .rust_version
                        .as_deref()
                        .map(PartialVersion::parse)
                        .transpose()
                        .map_err(|failure| unreadable.unparsed(&failure))?,
                    published: entry
                        .pubtime
                        .as_deref()
                        .map(|time| Date::parse(time.get(..DATE).unwrap_or(time)))
                        .transpose()
                        .map_err(|failure| unreadable.unparsed(&failure))?,
                })
            })
            .collect()
    }
}
