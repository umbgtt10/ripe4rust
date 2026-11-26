// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::unreadable::Unreadable;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::release::Release;
use ripe4rust_core::values::date::Date;
use ripe4rust_core::values::version::Version;
use std::collections::BTreeMap;

const DIST: &str = "static.rust-lang.org/dist/";
const MANIFEST: &str = "channel-rust-";
const TOML: &str = ".toml";

/// The Rust distribution's list of every channel manifest it published, one
/// a line, `static.rust-lang.org/dist/<date>/channel-rust-<name>.toml`: the
/// stable releases, a full version each, dated by the day of their last
/// listing -- a release listed earlier too was promoted ahead of its day --;
/// the nightlies, the betas and other pre-releases, a line's alias such as
/// `1.100` and `stable` itself are passed over. A list naming no stable
/// release does not read.
pub struct ManifestList;

impl ManifestList {
    pub fn releases(&self, text: &str) -> Result<Vec<Release>, PortFailure> {
        let mut days = BTreeMap::new();
        for line in text.lines() {
            if let Some((version, day)) = Self::listed(line)? {
                let last = days
                    .get(&version)
                    .map_or(day, |known: &Date| day.max(*known));
                days.insert(version, last);
            }
        }
        if days.is_empty() {
            return Err(Self::unreadable().failure("the list names no stable release"));
        }
        Ok(days
            .into_iter()
            .map(|(version, day)| Release {
                version,
                yanked: false,
                rust_version: None,
                published: Some(day),
            })
            .collect())
    }

    fn listed(line: &str) -> Result<Option<(Version, Date)>, PortFailure> {
        let Some((day, manifest)) = line
            .trim()
            .strip_prefix(DIST)
            .and_then(|rest| rest.split_once('/'))
        else {
            return Ok(None);
        };
        let version = manifest
            .strip_prefix(MANIFEST)
            .and_then(|name| name.strip_suffix(TOML))
            .and_then(|name| Version::parse(name).ok())
            .filter(|version| !version.is_pre_release());
        let Some(version) = version else {
            return Ok(None);
        };
        let day = Date::parse(day).map_err(|failure| Self::unreadable().unparsed(&failure))?;
        Ok(Some((version, day)))
    }

    fn unreadable() -> Unreadable {
        Unreadable::new("reading the Rust distribution's list of manifests")
    }
}
