// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::cargo::crates_io::CratesIo;
use crate::adapters::workspace::toml_file::TomlFile;
use ripe4rust_core::inputs::git::git_entry::GitEntry;
use ripe4rust_core::inputs::lock_edge::LockEdge;
use ripe4rust_core::inputs::lock_entry::LockEntry;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::values::version::Version;
use toml::Spanned;
use toml::de::DeValue;

const GIT: &str = "git+";
const LOCK: &str = "Cargo.lock";
const SHORT: usize = 8;

#[derive(PartialEq, Eq)]
struct LockTarget<'a> {
    pub name: &'a str,
    pub version: &'a str,
}

struct LockPackage {
    pub name: String,
    pub version: String,
    pub source: Option<String>,
    pub dependencies: Vec<String>,
}

/// `Cargo.lock` read as it stands, without cargo: its crates.io packages,
/// each by name and version, in that order; the crates of its git
/// sources, each by name, source and the short rev cargo prints; and the
/// dependencies it writes with a version -- `"windows-sys 0.52.0"`, a source
/// after it where the version alone is ambiguous --, every package's on a
/// crates.io package, in order. A bare name is a crate the lock holds once,
/// which no edge can be re-pointed between. cargo is not asked, so a lock a
/// manifest edited by hand no longer matches is still read as it is.
pub struct LockReader;

impl LockReader {
    pub fn edges(&self, text: &str) -> Result<Vec<LockEdge>, PortFailure> {
        let packages = Self::packages(text)?;
        let registry = packages
            .iter()
            .filter(|package| CratesIo.holds(package.source.as_deref()))
            .map(|package| LockTarget {
                name: &package.name,
                version: &package.version,
            })
            .collect::<Vec<LockTarget>>();
        let mut edges = Vec::new();
        for package in &packages {
            for target in package
                .dependencies
                .iter()
                .filter_map(|dependency| Self::versioned(dependency))
                .filter(|target| registry.contains(target))
            {
                edges.push(LockEdge {
                    dependent: package.name.clone(),
                    dependent_version: Self::version(&package.name, &package.version)?,
                    dependency: String::from(target.name),
                    dependency_version: Self::version(target.name, target.version)?,
                });
            }
        }
        edges.sort();
        Ok(edges)
    }

    pub fn entries(&self, text: &str) -> Result<Vec<LockEntry>, PortFailure> {
        let mut entries = Vec::new();
        for LockPackage {
            name,
            version,
            source,
            ..
        } in Self::packages(text)?
        {
            if CratesIo.holds(source.as_deref()) {
                entries.push(LockEntry {
                    version: Self::version(&name, &version)?,
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
                    dependencies: match package.get("dependencies").map(Spanned::get_ref) {
                        Some(DeValue::Array(dependencies)) => dependencies
                            .iter()
                            .filter_map(|dependency| dependency.get_ref().as_str())
                            .map(String::from)
                            .collect(),
                        _ => Vec::new(),
                    },
                })
            })
            .collect())
    }

    fn version(name: &str, version: &str) -> Result<Version, PortFailure> {
        Version::parse(version).map_err(|_| PortFailure::WorkspaceUnreadable {
            file: String::from(LOCK),
            detail: format!("the package `{name}` is at `{version}`, which is not a version"),
        })
    }

    fn versioned(dependency: &str) -> Option<LockTarget<'_>> {
        let mut words = dependency.splitn(3, ' ');
        let (name, version) = (words.next()?, words.next()?);
        let crates_io = words.next().is_none_or(|source| {
            CratesIo.holds(
                source
                    .strip_prefix('(')
                    .and_then(|source| source.strip_suffix(')')),
            )
        });
        crates_io.then_some(LockTarget { name, version })
    }
}
