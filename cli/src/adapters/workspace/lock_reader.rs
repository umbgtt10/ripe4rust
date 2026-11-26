// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::cargo::crates_io::CratesIo;
use crate::adapters::workspace::toml_file::TomlFile;
use ripe4rust_core::inputs::git::git_entry::GitEntry;
use ripe4rust_core::inputs::lock_entry::LockEntry;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::values::version::Version;
use toml::Spanned;
use toml::de::DeValue;

const GIT: &str = "git+";
const LOCK: &str = "Cargo.lock";
const SHORT: usize = 8;

struct LockPackage {
    pub name: String,
    pub version: String,
    pub source: Option<String>,
}

/// `Cargo.lock` read as it stands, without cargo: its crates.io packages,
/// each by name and version, in that order; and the crates of its git
/// sources, each by name, source and the short rev cargo prints. cargo is not asked, so a lock a
/// manifest edited by hand no longer matches is still read as it is.
pub struct LockReader;

impl LockReader {
    pub fn entries(&self, text: &str) -> Result<Vec<LockEntry>, PortFailure> {
        let mut entries = Vec::new();
        for LockPackage {
            name,
            version,
            source,
        } in Self::packages(text)?
        {
            if CratesIo.holds(source.as_deref()) {
                entries.push(LockEntry {
                    version: Version::parse(&version).map_err(|_| {
                        PortFailure::WorkspaceUnreadable {
                            file: String::from(LOCK),
                            detail: format!(
                                "the package `{name}` is at `{version}`, which is not a version"
                            ),
                        }
                    })?,
                    name,
                });
            }
        }
        entries.sort();
        Ok(entries)
    }

    pub fn git(&self, text: &str) -> Result<Vec<GitEntry>, PortFailure> {
        let mut entries = Self::packages(text)?
            .into_iter()
            .filter_map(|LockPackage { name, source, .. }| {
                let source = source?;
                let (source, rev) = source.strip_prefix(GIT)?.rsplit_once('#')?;
                Some(GitEntry {
                    name,
                    source: String::from(source),
                    rev: rev.chars().take(SHORT).collect(),
                })
            })
            .collect::<Vec<GitEntry>>();
        entries.sort();
        Ok(entries)
    }

    fn packages(text: &str) -> Result<Vec<LockPackage>, PortFailure> {
        let document = TomlFile::new(LOCK, text).table()?;
        let packages = match document.get_ref().get("package").map(Spanned::get_ref) {
            Some(DeValue::Array(packages)) => packages.iter().collect(),
            _ => Vec::new(),
        };
        Ok(packages
            .iter()
            .filter_map(|package| package.get_ref().as_table())
            .filter_map(|package| {
                let field = |key: &str| package.get(key).and_then(|value| value.get_ref().as_str());
                Some(LockPackage {
                    name: String::from(field("name")?),
                    version: String::from(field("version")?),
                    source: field("source").map(String::from),
                })
            })
            .collect())
    }
}
