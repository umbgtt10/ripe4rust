// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::cargo::crates_io::CratesIo;
use crate::adapters::workspace::toml_file::TomlFile;
use ripe4rust_core::inputs::lock_entry::LockEntry;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::values::version::Version;
use toml::Spanned;
use toml::de::DeValue;

const LOCK: &str = "Cargo.lock";

/// `Cargo.lock` read as it stands, without cargo: its crates.io packages,
/// each by name and version, in that order. cargo is not asked, so a lock a
/// manifest edited by hand no longer matches is still read as it is.
pub struct LockReader;

impl LockReader {
    pub fn entries(&self, text: &str) -> Result<Vec<LockEntry>, PortFailure> {
        let document = TomlFile::new(LOCK, text).table()?;
        let packages = match document.get_ref().get("package").map(Spanned::get_ref) {
            Some(DeValue::Array(packages)) => packages.iter().collect(),
            _ => Vec::new(),
        };
        let mut entries = Vec::new();
        for package in packages
            .iter()
            .filter_map(|package| package.get_ref().as_table())
        {
            let field = |key: &str| package.get(key).and_then(|value| value.get_ref().as_str());
            let (Some(name), Some(version)) = (field("name"), field("version")) else {
                continue;
            };
            if CratesIo.holds(field("source")) {
                entries.push(LockEntry {
                    name: String::from(name),
                    version: Version::parse(version).map_err(|_| {
                        PortFailure::WorkspaceUnreadable {
                            file: String::from(LOCK),
                            detail: format!(
                                "the package `{name}` is at `{version}`, which is not a version"
                            ),
                        }
                    })?,
                });
            }
        }
        entries.sort();
        Ok(entries)
    }
}
